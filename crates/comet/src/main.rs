use axum::{response::IntoResponse, routing::get, Router};
use conf::configuration;
use serde_json::json;
use state::AppDBState;
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;

pub mod logging_tracing;
pub mod conf;
pub mod kafka;
pub mod state;
pub mod controller;
pub mod routes;
pub mod error;
pub mod payload;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {
    let config = configuration::Configuration::load().unwrap();
  //  dotenv().ok();

  logging_tracing::init(&config)?;


 let kafka_producer = kafka::init_producer::create_new_kafka_producer(&config.kafka).unwrap();

  let state = AppDBState { producer: kafka_producer };

        let comet_routes = routes::create_comet_routes();
   // let game_routes = routes::game_logic_routes::create_game_routes();
    let routes_all = Router::new()
                          .route( "/api/v1/health", get(health))
                            .nest("/api/v1/comet", comet_routes)
                         //   .nest( "/api/v1/game", game_routes)
                            .layer(ServiceBuilder::new()
                                    .layer(CookieManagerLayer::new())
                                    .layer(CorsLayer::permissive()))
                            .with_state(state);


    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3004")
        .await
        .unwrap();
    info!("starting messier service on port {}", listener.local_addr().unwrap());
    axum::serve(listener, routes_all).await.unwrap();

    Ok(())

}

pub async fn health() -> impl IntoResponse {
  axum::Json(json!({ "Messier status" : "UP" }))
}