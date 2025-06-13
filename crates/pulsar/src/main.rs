use std::net::SocketAddr;

use axum::{response::IntoResponse, routing::get, Router};
use cosmic::{utils::remove_duplicate_keys, KafkaPublishPayload, PostTXPayload, SubRedditTXPayload, TransactionWSEvent, UserTXPayload, KAFKA_POST_PUBLISH_KEY, KAFKA_POST_PUBLISH_TOPIC, KAFKA_SUBREDDIT_PUBLISH_KEY, KAFKA_SUBREDDIT_PUBLISH_TOPIC, KAFKA_USER_PUBLISH_KEY, KAFKA_USER_PUBLISH_TOPIC, POST_TX_KEY, SOVEREIGN_REDDIT_ROLLUP_EVENTS, SUBREDDIT_TX_KEY, USER_TX_KEY};
use futures_util::StreamExt;
use lapin::{options::{BasicAckOptions, BasicConsumeOptions, QueueDeclareOptions}, types::FieldTable, Connection, ConnectionProperties};
use rdkafka::{producer::{FutureProducer, FutureRecord, Producer}, util::Timeout};
use serde_json::json;
use tokio::task::JoinHandle;
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;
use futures_util::future;
use crate::{conf::{config_types::ServerConfiguration, configuration::{self, Configuration}}, kafka::init_producer};
use std::time::Duration;



pub mod conf;
pub mod error;
pub mod kafka;


#[tokio::main]
async fn main() {

      let config = configuration::Configuration::load().unwrap();
  //  dotenv().ok();

  //logging_tracing::init(&config);

    let consume_and_publish_handle = rabbit_mq_consume_and_kafka_publish(&config).await;
    
    start_web_server(&config.server, vec![consume_and_publish_handle])
    .await;


}

async fn start_web_server(
    config: &ServerConfiguration,
    shutdown_handles: Vec<JoinHandle<()>>,
) {

    // Initialize routing
 let routes_all = Router::new()
                          .route( "/api/v1/health", get(health))
                         //   .nest( "/api/v1/game", game_routes)
                            .layer(ServiceBuilder::new()
                                    .layer(CookieManagerLayer::new())
                                    .layer(CorsLayer::permissive()));

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("listening on {addr}");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3005")
    .await
    .unwrap();
println!("listening on {}", listener.local_addr().unwrap());
axum::serve(listener, routes_all).with_graceful_shutdown(shutdown_signal(shutdown_handles)).await.unwrap();

    // Shutdown tracing provider
}

pub async fn health() -> impl IntoResponse {
  axum::Json(json!({ "Pulsar status" : "UP" }))
}


pub async fn rabbit_mq_consume_and_kafka_publish(
    config: &Configuration
) -> JoinHandle<()> {

    let conf = config.clone();

    let producer = init_producer::create_new_kafka_producer(&conf.kafka).unwrap();


   tokio::spawn(async move {
        do_listen(  &producer, conf.clone() ).await;
    })

}


pub async fn do_listen(
       producer: &FutureProducer,
    config: Configuration,
) {


     let conn = Connection::connect(
        &config.rabbit_mq.ampq_addr,
        ConnectionProperties::default(),
    )
    .await.unwrap();


    let channel = conn.create_channel().await.unwrap();

      let queue =   channel.queue_declare(SOVEREIGN_REDDIT_ROLLUP_EVENTS,   QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await.unwrap();

     println!("Declared Pulsar queue {:?}", queue);
    
    let mut consumer = channel
        .basic_consume(
            SOVEREIGN_REDDIT_ROLLUP_EVENTS,
            "consumer_",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await.unwrap();


       while let Some(delivery) = consumer.next().await {
        let delivery = delivery.expect("error in consumer");
        

        let message_key = delivery.properties.kind().clone().unwrap();


        println!("Message key is: {:?}" , message_key.clone());

        //First publish to topic and then act delivered Message

        let text : String = serde_json::from_slice(&delivery.data).unwrap();

        let payload = remove_duplicate_keys(&text).unwrap();
        let rs = match message_key.to_string().as_str() {

            USER_TX_KEY => {


                    let tx_ws_payload : TransactionWSEvent = serde_json::from_str(&payload).unwrap();


                println!("USER CREATED PAYLOAD IS: {:?}" , tx_ws_payload);


                let user_event_paylod = tx_ws_payload.additional_properties.get("value").unwrap().get("user_created_event").unwrap();

                    

                    //Parse Payload event and create kafka publish payload

                    Ok(KafkaPublishPayload{ payload: serde_json::to_string(&user_event_paylod).unwrap(), 
                    topic: KAFKA_USER_PUBLISH_TOPIC.to_string(), key: KAFKA_USER_PUBLISH_KEY.to_string() })

            },

            SUBREDDIT_TX_KEY => {

                let tx_ws_payload : TransactionWSEvent = serde_json::from_str(&payload).unwrap();


                println!("Subreddit CREATED PAYLOAD IS: {:?}" , tx_ws_payload);


                let subredd_created_event = tx_ws_payload.additional_properties.get("value").unwrap().get("sub_reddit_created_event").unwrap();

                    //Parse Payload event and create kafka publish payload

                    Ok(KafkaPublishPayload{ payload: serde_json::to_string(&subredd_created_event).unwrap(), 
                    topic: KAFKA_SUBREDDIT_PUBLISH_TOPIC.to_string(), key: KAFKA_SUBREDDIT_PUBLISH_KEY.to_string() })



            },

            POST_TX_KEY => {

                
                let tx_ws_payload : TransactionWSEvent = serde_json::from_str(&payload).unwrap();


                println!("Post CREATED PAYLOAD IS: {:?}" , tx_ws_payload);


                let post_created_payload = tx_ws_payload.additional_properties.get("value").unwrap().get("post_created_event").unwrap();

                    //Parse Payload event and create kafka publish payload

                    Ok(KafkaPublishPayload{ payload: serde_json::to_string(&post_created_payload).unwrap(), 
                    topic: KAFKA_POST_PUBLISH_TOPIC.to_string(), key: KAFKA_POST_PUBLISH_KEY.to_string() })



            }


            _ => Err("Invalid transaction key recieved")

        };



        if rs.is_err() {
            println!("Error while parsing payload")
        } else {

            let kafka_payload = rs.unwrap();

               producer.begin_transaction().unwrap();


             let kafka_result = future::try_join_all(vec![kafka_payload].iter().map(|event| async move {
        let delivery_result = producer
        .send(
            FutureRecord::to(&event.topic)
                    .payload(&event.payload)
                    .key(&event.key.clone()),
            Duration::from_secs(3),
        )
        .await;

    // This will be executed when the result is received.
  //  println!("Delivery status for message {} received", i);
    delivery_result

    })

    ).await;

  match kafka_result {
        Ok(_) => {
             let _ = delivery
            .ack(BasicAckOptions::default())
            .await;

        },
        Err(e) =>  {
            println!("Error is: {:?}" , e.0);
            println!("Error while producing kafka payload to topic")
        },
    };

    producer.commit_transaction(Timeout::from(Duration::from_secs(10))).unwrap(); 

        }        
    }

}


pub async fn shutdown_signal(shutdown_handles: Vec<JoinHandle<()>>) {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Initialization of Ctrl+C handler failed");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Initialization of signal handler failed")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    for handle in shutdown_handles {
        handle.abort();
    }
}
