use std::{collections::HashMap, net::SocketAddr};

use axum::{http::request, response::IntoResponse, routing::get, Router};
use cosmic::payload::CreateAndSaveModel;
use rdkafka::{consumer::StreamConsumer, Message};
use serde_json::json;
use tokio::{spawn, task::JoinHandle};
use tracing::warn;

use crate::{conf::{config_types::ServerConfiguration, configuration::Configuration}, context::{ContextImpl, DynContext}};



pub mod kafka;
pub mod conf;
pub mod error;
pub mod context;


#[tokio::main]
async fn main() {
    


      let config = conf::configuration::Configuration::load().unwrap();

  //  logging_tracing::init(&config).unwrap();

    let consumers = kafka::init_consumer::init_consumers(&config.kafka).unwrap();
    
    
    let reqwest_client = reqwest::Client::new();
    
    let context = ContextImpl::new_dyn_context(reqwest_client);
    
    let user_and_game_handles = init_subreddit_model_created_events(
        context,
        &config, 
        consumers
    );

    start_web_server(&config.server, vec![user_and_game_handles])
    .await;

}




async fn start_web_server(
    config: &ServerConfiguration,
    shutdown_handles: Vec<JoinHandle<()>>,
) {
    // Initialize routing
    let routing = init_routing();

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("listening on {addr}");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3008")
    .await
    .unwrap();
println!("listening on {}", listener.local_addr().unwrap());
axum::serve(listener, routing.into_make_service_with_connect_info::<SocketAddr>()).with_graceful_shutdown(shutdown_signal(shutdown_handles)).await.unwrap();

    // Shutdown tracing provider
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


fn init_routing() -> Router {
    
    let base_router = Router::new().route("/api/v1/health", get(health));

    return base_router;

}

pub async fn health() -> impl IntoResponse {
    axum::Json(json!({ "Aether Subreddit status" : "UP" }))
}

fn init_subreddit_model_created_events(
    context: DynContext,
    config: &Configuration,
    kafka_consumers: HashMap<String, StreamConsumer>,
) -> JoinHandle<()> {

    let mut kafka_joins: Vec<JoinHandle<()>> = vec![];

    for (key_topic , value) in kafka_consumers.into_iter() {
       let kf_join =  listen(
            context.clone(),
            config,
            value,
            key_topic
        );

        kafka_joins.push(kf_join);
    }

    let join_handle = spawn(async move {
        for handle in kafka_joins {
            handle.await.unwrap();
        }
    });

    return join_handle;
    

}


pub fn listen(
    context: DynContext,
    config: &Configuration,
    stream_consumer: StreamConsumer,
    key_topic: String,
) -> JoinHandle<()> {
    let topic = key_topic.clone();

    // Start listener
    tokio::spawn(async move {
        do_listen( context, &stream_consumer, topic ).await;
    })
}

pub async fn do_listen(
    context: DynContext,
    stream_consumer: &StreamConsumer,
    topic_name: String,
) {

 

    loop {
        match stream_consumer.recv().await {
            Err(e) => warn!("Error: {}", e),
            Ok(message) => {
 
            let payload = String::from_utf8(message.payload().unwrap().to_vec()).unwrap();

                //Parse subreddit payload
                let client = context.get_reqwest_client();

                let reddit_user_payload = CreateAndSaveModel {
                    schema_type: "subreddit".to_string(),
                    data: payload,
                };


                
                for _i in 0..3 {
                                  let rsp = client.post("http://localhost:3006/api/v1/model/create/subreddit").header("Content-Type", "application/json")
                .body(&reddit_user_payload).send().await;


            if rsp.is_ok() {
                break;
            }
                }
            
    
                
        }
        }
}

}


