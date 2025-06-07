use axum::{response::IntoResponse, routing::get, Router};
use conf::configuration;
use serde_json::json;
use state::AppDBState;
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;
use tracing::info;

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


    let routes_all = Router::new()
                          .route( "/api/v1/health", get(health))
                         //   .nest( "/api/v1/game", game_routes)
                            .layer(ServiceBuilder::new()
                                    .layer(CookieManagerLayer::new())
                                    .layer(CorsLayer::permissive()));


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