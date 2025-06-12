use axum::{response::IntoResponse, routing::get, Router};
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use serde_json::json;
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;
use tracing::info;

use crate::{conf::configuration, state::DBState};

pub mod routes;
pub mod controller;
pub mod state;
pub mod error;
pub mod conf;
pub mod sql_statements;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {
    let config = configuration::Configuration::load().unwrap();
  //  dotenv().ok();

  //logging_tracing::init(&config)?;

    //Connect with database
    let connection = match Database::connect(config.postgres_url.url).await {
        Ok(connection) => connection,
        Err(e) => panic!("{:?}",e)
    };

    Migrator::up(&connection, None).await?;

    let client = redis::Client::open(config.redis_url.url).unwrap();
    let redis_connection = client.get_multiplexed_async_connection().await.unwrap(); 
 
  let state = DBState { connection: connection, redis_connection: redis_connection };


  let schema_routes  = routes::create_db_layer_routes();

  let routes_all = Router::new()
                          .route( "/api/v1/health", get(health))
                           .nest( "/api/v1/schema", schema_routes)
                            .layer(ServiceBuilder::new()
                                    .layer(CookieManagerLayer::new())
                                    .layer(CorsLayer::permissive()))
                            .with_state(state);


    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3006")
        .await
        .unwrap();
    info!("starting DB Layer service on port {}", listener.local_addr().unwrap());
    axum::serve(listener, routes_all).await.unwrap();

    Ok(())
}


pub async fn health() -> impl IntoResponse {
  axum::Json(json!({ "DB-Layer status" : "UP" }))
}