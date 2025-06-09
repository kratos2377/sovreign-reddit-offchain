use axum::{routing::post, Router};

use crate::{controller, state::DBState};



pub fn create_db_layer_routes() -> Router<DBState> {
   Router::new()
        .route("/create/:schema", post(controller::create_and_save_model))
        .route("/fetch/:schema", post(controller::fetch_model_from_db_by_primary_key))
       
}