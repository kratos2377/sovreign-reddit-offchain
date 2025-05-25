use axum::{middleware, routing::{post,get,put}, Router};

use crate::{controller, state::AppDBState, };

pub fn create_comet_routes() -> Router<AppDBState> {
    Router::new()
    .route("/publish_chain_changes", post(controller::publish_state_change_to_topic))
}