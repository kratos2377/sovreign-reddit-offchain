use axum::{response::IntoResponse, routing::get, Router};
use conf::configuration;
use cosmic::{TransactionWSEvent, SOVEREIGN_REDDIT_ROLLUP_EVENTS};
use deadpool_lapin::{lapin::{options::BasicPublishOptions, BasicProperties, ConnectionProperties}, Manager, Pool as RabbitPool, PoolError};
use deadpool_redis::{redis::{AsyncCommands, ExistenceCheck, SetExpiry, SetOptions}, Config as RedisConfig};
use reqwest::Client;
use serde_json::json;
use state::AppDBState;
use tokio::task::JoinHandle;
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;
use tracing::info;
use std::{net::SocketAddr, result::Result as StdResult};
use futures_util::{SinkExt, StreamExt, TryStreamExt};
use reqwest_websocket::{Error, Message, RequestBuilderExt};
use crate::conf::{config_types::ServerConfiguration, configuration::Configuration};

pub mod logging_tracing;
pub mod conf;
pub mod state;
pub mod error;
pub mod payload;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {
    let config = configuration::Configuration::load().unwrap();
  //  dotenv().ok();



  logging_tracing::init(&config)?;


    let ws_handles = listen_ws_event_and_publish_to_ampq(&config).await;

    
    start_web_server(&config.server, vec![ws_handles])
    .await;

    Ok(())

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

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3004")
    .await
    .unwrap();
println!("listening on {}", listener.local_addr().unwrap());
axum::serve(listener, routes_all).with_graceful_shutdown(shutdown_signal(shutdown_handles)).await.unwrap();

    // Shutdown tracing provider
}

pub async fn health() -> impl IntoResponse {
  axum::Json(json!({ "Comet status" : "UP" }))
}


pub async fn listen_ws_event_and_publish_to_ampq(
    config: &Configuration
) -> JoinHandle<()> {


      let redis_config = RedisConfig::from_url("redis://localhost:6379");
    let redis_pool = redis_config.create_pool(Some(deadpool_redis::Runtime::Tokio1)).unwrap();

     let manager = Manager::new(config.rabbit_mq.ampq_addr.clone(), ConnectionProperties::default());
    let rabbit_pool: RabbitPool = deadpool::managed::Pool::builder(manager)
        .max_size(10)
        .build()
        .expect("not able to create RabbitMQ pool");
  
    let response = Client::new()
    .get("wss://echo.websocket.org/")
    .upgrade() // Prepares the WebSocket upgrade.
    .send()
    .await.unwrap();


  let mut websocket = response.into_websocket().await.unwrap();
   tokio::spawn(async move {

      loop {
                match websocket.try_next().await {
              Ok(message) => {
                    if let Message::Text(text) = message.unwrap() {


                        let ws_event: TransactionWSEvent  = serde_json::from_str(&text).unwrap();

                        let mut redis_connection = redis_pool.get().await.unwrap();


                        let redis_res : bool=  redis_connection.set_options(&ws_event.tx_hash, "exist" , 
                        SetOptions::default().conditional_set(ExistenceCheck::NX).with_expiration(SetExpiry::EX(600))).await.unwrap();

                  let payload = serde_json::to_vec(&text).unwrap();
                        if redis_res {


                            let lapin_conn = rabbit_pool.get().await.unwrap();
                            let channel = lapin_conn.create_channel().await.unwrap();

                                            channel
                        .basic_publish(
                            "",
                            SOVEREIGN_REDDIT_ROLLUP_EVENTS,
                            BasicPublishOptions::default(),
                            &payload,
                            BasicProperties::default().with_kind(ws_event.key.into()),
                        )
                        .await.unwrap();



                        } else {
                          println!("Key Already Set");
                        }


              }
              }

              Err(_) => {
                println!("Invalid Message Recieved")
              }
          }
      }

    })
    

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
