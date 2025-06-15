use axum::{routing::{get, post}, Router};

use crate::{controller, state::DBState};



pub fn create_db_layer_routes() -> Router<DBState> {
   Router::new()
        .route("/create/{schema}", post(controller::create_and_save_model))
        .route("/fetch/{schema}", get(controller::fetch_model_from_db_by_primary_key))
        .route("/join_or_unjoin_sub", post(controller::join_and_unjoin_sub))
        .route("/add_comments", post(controller::add_comments))
        .route("/like_or_dislike_post", post(controller::like_or_dislike_post))
        .route("/get_user_posts", post(controller::get_user_posts))
        .route("/get_user_comments", post(controller::get_user_comments))
        .route("/get_posts_for_subreddit", post(controller::get_posts_for_subreddit))
        .route("/get_comments_for_posts", post(controller::get_comments_for_posts))
        .route("/get_user_feed", post(controller::get_user_feed))
        .route("/get_user_subs/{user_sov_id}", get(controller::get_user_subs))
       
}