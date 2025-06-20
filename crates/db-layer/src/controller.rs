use std::collections::HashMap;

use axum::extract::{Path, Query};
use axum::{extract::State, Json};
use chrono::Utc;
use cosmic::payload::{AddCommentPayload, CreateAndSaveModel, GetCommentsForPosts, GetPostsForSubreddit, GetUserPostsOrCommentsPayload, JoinOrUnjoinSub, LikeOrDislikeComment, LikeOrDislikePost, SearchSubredditsPayload, UserFeedPayload};
use cosmic::{PostTXPayload, SubRedditTXPayload, UserTXPayload, POST_SCHEMA, SUBREDDIT_SCHEMA, USER_SCHEMA};
use dark_matter::{comments, posts, sub_mods, subreddit, user_joined_subs, user_liked_posts, users};
use migration::Expr;
use sea_orm::prelude::Uuid;
use sea_orm::{ActiveModelTrait, ConnectionTrait, DbBackend, QueryFilter, Statement};
use migration::extension::postgres;
use redis::{AsyncCommands, RedisResult};
use sea_orm::{EntityTrait, Set};
use serde_json::json;
use sea_orm::ModelTrait;
use crate::sql_statements::get_sql_statement_for_post_upvote;
use crate::{state::DBState};
use crate::error::{Error , Result as APIResult};
use serde_json::Value;
use sea_orm::ColumnTrait;

pub async fn create_and_save_model(
        state: State<DBState>,
        Path(schema): Path<String>,
	payload: Json<CreateAndSaveModel>,
) -> APIResult<Json<Value>> {


    if payload.schema_type == "" || payload.data == "" {
        return Err(Error::MissingParams)
    }


    let postgres_conn = state.connection.clone();

    tokio::spawn( async move {
         if payload.schema_type == USER_SCHEMA.to_string() {

                let parsed_user_payload_rs: Result<UserTXPayload, serde_json::Error> = serde_json::from_str(&payload.data);
                let parsed_user_payload = parsed_user_payload_rs.unwrap();

                let active_user_model = users::ActiveModel{
                    sov_id: Set(parsed_user_payload.user_address),
                    username: Set(parsed_user_payload.username),
                    created_at: Set(Utc::now().naive_utc()),
                    updated_at: Set(Utc::now().naive_utc()),
                };


                active_user_model.insert(&postgres_conn).await;
                

        } else if payload.schema_type == SUBREDDIT_SCHEMA.to_string() {

              let parsed_sub_payload_rs: Result<SubRedditTXPayload, serde_json::Error> = serde_json::from_str(&payload.data);
                let parsed_sub_payload = parsed_sub_payload_rs.unwrap();

                let active_sub_model = subreddit::ActiveModel{
                    created_at: Set(Utc::now().naive_utc()),
                    updated_at: Set(Utc::now().naive_utc()),
                    sub_sov_id: Set(parsed_sub_payload.subaddress.clone()),
                    subname: Set(parsed_sub_payload.subname.clone()),
                    sub_description: Set(parsed_sub_payload.description),
                };

                let active_sub_mods_model = sub_mods::ActiveModel {
                   sub_sov_id: Set(parsed_sub_payload.subaddress.clone()),
                    user_sov_id: Set(parsed_sub_payload.mods.get(0).unwrap().to_string().clone()),
                    created_at: Set(Utc::now().naive_utc()),
                    updated_at: Set(Utc::now().naive_utc()),
                    subname: Set(parsed_sub_payload.subname),
                };

                let user_joined_sub_active_model = user_joined_subs::ActiveModel {
                    id: Set(Uuid::new_v4()),
                     user_sov_id: Set(parsed_sub_payload.mods.get(0).unwrap().to_string()),
                         sub_sov_id: Set(parsed_sub_payload.subaddress),
                     created_at: Set(Utc::now().naive_utc()),
                    updated_at: Set(Utc::now().naive_utc()),
                };

                let _ = active_sub_model.insert(&postgres_conn).await;
                let _ = active_sub_mods_model.insert(&postgres_conn).await;
                let _ = user_joined_sub_active_model.insert(&postgres_conn).await;


        } else  {


                  
                     let parsed_post_payload_rsp: Result<PostTXPayload, serde_json::Error> = serde_json::from_str(&payload.data);
                let parsed_reddit_payload = parsed_post_payload_rsp.unwrap();

                let active_post_model = posts::ActiveModel{
                    created_at: Set(Utc::now().naive_utc()),
                    updated_at: Set(Utc::now().naive_utc()),
                    sub_sov_id: Set(parsed_reddit_payload.subaddress),
                    post_sov_id: Set(parsed_reddit_payload.post_address.clone()),
                    title: Set(parsed_reddit_payload.title),
                    content: Set(parsed_reddit_payload.content),
                    flair: Set(parsed_reddit_payload.flair),
                    user_sov_id: Set(parsed_reddit_payload.user_address.clone()),
                    upvote: Set(1),
                    downvote: Set(0),
                    score: Set(1),
                };


                let user_liked_post_active_model = user_liked_posts::ActiveModel {
                    id: Set(Uuid::new_v4()),
                    post_sov_id: Set(parsed_reddit_payload.post_address),
                    value: Set(1 as i32),
                     created_at: Set(Utc::now().naive_utc()),
                    updated_at: Set(Utc::now().naive_utc()),
                    user_sov_id: Set(parsed_reddit_payload.user_address),
                };

                active_post_model.insert(&postgres_conn).await;

                let _ = user_liked_post_active_model.insert(&postgres_conn).await;

        };


    });

       let body = Json(json!({
         "result": {
             "success": true
         },
     }));
 
     Ok(body)

}



pub async fn fetch_model_from_db_by_primary_key(
    mut state: State<DBState>,
    Path((schema, primary_key)): Path<(String , String)>,
) -> APIResult<Json<Value>> {
    use serde_json::json;
    use serde_json;

    let rsp = match schema.as_str() {
        "user" => {
            let redis_key = get_redis_key("user", &primary_key);
            let redis_rsp: RedisResult<String> = state.redis_connection.get(&redis_key).await;

            if let Ok(cached) = redis_rsp {
                // Cache hit: parse Vec<Model> from JSON
                let models: Vec<users::Model> = serde_json::from_str(&cached).unwrap_or_default();
                Ok(json!({"models": models}))
            } else {
                // Cache miss: fetch from Postgres
                let postgres_resp = users::Entity::find_by_id(primary_key).one(&state.connection).await;
                match postgres_resp {
                    Ok(opt_model) => {
                        let models: Vec<users::Model> = opt_model.into_iter().collect();
                        let json_str = serde_json::to_string(&models).unwrap();
                        let _: RedisResult<()> = state.redis_connection.set_ex(&redis_key, json_str.clone(), 60).await;
                        Ok(json!({"models": models}))
                    },
                    Err(_) => Err("user fetch error"),
                }
            }
        },
        "subreddit" => {
            let redis_key = get_redis_key("subreddit", &primary_key);
            let redis_rsp: RedisResult<String> = state.redis_connection.get(&redis_key).await;
            if let Ok(cached) = redis_rsp {
                let models: Vec<subreddit::Model> = serde_json::from_str(&cached).unwrap_or_default();
                Ok(json!({"models": models}))
            } else {
                let postgres_resp = subreddit::Entity::find_by_id(primary_key).one(&state.connection).await;
                match postgres_resp {
                    Ok(opt_model) => {
                        let models: Vec<subreddit::Model> = opt_model.into_iter().collect();
                        let json_str = serde_json::to_string(&models).unwrap();
                        let _: RedisResult<()> = state.redis_connection.set_ex(&redis_key, json_str.clone(), 60).await;
                        Ok(json!({"models": models}))
                    },
                    Err(_) => Err("subreddit fetch error"),
                }
            }
        },
        "post" => {
            let redis_key = get_redis_key("post", &primary_key);
            let redis_rsp: RedisResult<String> = state.redis_connection.get(&redis_key).await;
            if let Ok(cached) = redis_rsp {
                let models: Vec<posts::Model> = serde_json::from_str(&cached).unwrap_or_default();
                Ok(json!({"models": models}))
            } else {
                let postgres_resp = posts::Entity::find_by_id(primary_key).one(&state.connection).await;
                match postgres_resp {
                    Ok(opt_model) => {
                        let models: Vec<posts::Model> = opt_model.into_iter().collect();
                        let json_str = serde_json::to_string(&models).unwrap();
                        let _: RedisResult<()> = state.redis_connection.set_ex(&redis_key, json_str.clone(), 60).await;
                        Ok(json!({"models": models}))
                    },
                    Err(_) => Err("post fetch error"),
                }
            }
        },
        _ => Err("Invalid Schema"),
    };

    if let Err(_) = rsp {
        return Err(Error::UnexpectedError)
    }

    let body = Json(json!({
        "result": {
            "success": true
        },
        "data": rsp.unwrap()
    }));
    Ok(body)
}


pub async fn join_and_unjoin_sub(
    state: State<DBState>,
    payload: Json<JoinOrUnjoinSub>
) -> APIResult<Json<Value>> {

    if payload.user_sov_id == "" || payload.sub_sov_id == "" {
        return Err(Error::MissingParams)
    }

 let rsp = match payload.value {

    -1 => {

       let sub_mod_rsp =  user_joined_subs::Entity::find_by_sub_and_user_sov_id(&payload.sub_sov_id, &payload.user_sov_id)
                    .one(&state.connection).await;

        if sub_mod_rsp.is_err() {
            Err("Some DB Error Occured")
        } else {
            
 

        let sub_mod_model = sub_mod_rsp.unwrap().unwrap();

         let _  = sub_mod_model.delete(&state.connection).await;

          Ok("Unjoined Sub")
        }
        

    },

    1 => {


        let sub_mod_active_model = user_joined_subs::ActiveModel {
            sub_sov_id: Set(payload.sub_sov_id.clone()),
            user_sov_id: Set(payload.user_sov_id.clone()),
            id: Set(Uuid::new_v4()),
        created_at: Set(Utc::now().naive_utc()),
        updated_at: Set(Utc::now().naive_utc()),
        };

       let rsp =  sub_mod_active_model.insert(&state.connection).await;

       if rsp.is_err() {
              Err("Some DB Error Occured")
       } else {
        
        Ok("Joined The Sub")
       }


    },


    _ => Err("Invalid Value Type")

 };


 if rsp.is_err() {
    return Err(Error::SomeErrorOccurred)
 }




    let body = Json(json!({
         "result": {
             "success": true
         },
     }));
 
     Ok(body)

}


pub async fn add_comments(
    state: State<DBState>,
    payload: Json<AddCommentPayload>
) -> APIResult<Json<Value>> {


    if payload.content == "" || payload.sub_sov_id == "" || payload.post_sov_id == "" || payload.user_sov_id == "" {
        return Err(Error::MissingParams)
    }

    let comment_active_model = comments::ActiveModel {
        id: Set(Uuid::new_v4()),
        user_sov_id: Set(payload.user_sov_id.clone()),
        post_sov_id: Set(payload.post_sov_id.clone()),
        content: Set(payload.content.clone()),
        upvote: Set(1),
        downvote: Set(0),
        score: Set(1),
        created_at: Set(Utc::now().naive_utc()),
        updated_at: Set(Utc::now().naive_utc()),
    };
    


    let rsp = comment_active_model.insert(&state.connection).await;


    if rsp.is_err() {
        return Err(Error::DBInsertError)
    }

    let body = Json(json!({
         "result": {
             "success": true
         },
     }));
 
     Ok(body)

}


pub async fn like_or_dislike_post(
    state: State<DBState>,
    payload: Json<LikeOrDislikePost>
) -> APIResult<Json<Value>> {


    if payload.post_sov_id == "" || payload.user_sov_id == "" {
        return Err(Error::MissingParams)
    }

    let redis_key = get_redis_key_from_keys("post", "upvote_metadata", &payload.post_sov_id);
 
   let redis_script =  if payload.prev_value == -1 {

        if payload.value == 1 {

                 redis::Script::new(
            r#"
            local key = KEYS[1]

            redis.call('HINCRBY', key, 'downvote', -1)
            redis.call('HINCRBY', key, 'score', 2)
            redis.call('HINCRBY', key, 'upvote', 1)  
            return "OK"
            "#
        )

        } else {

                 redis::Script::new(
            r#"
            local key = KEYS[1]

            redis.call('HINCRBY', key, 'downvote', -1)
            redis.call('HINCRBY', key, 'score', 1)
            redis.call('HINCRBY', key, 'upvote', 0)  
            return "OK"
            "#
        )

        }


    } else if payload.prev_value == 1 {


        if payload.value == -1 {
            
                redis::Script::new(
            r#"
            local key = KEYS[1]

            redis.call('HINCRBY', key, 'upvote', -1)
            redis.call('HINCRBY', key, 'score', -2)
            redis.call('HINCRBY', key, 'downvote', 1)
            return "OK"
            "#
        )
        } else {

                redis::Script::new(
            r#"
            local key = KEYS[1]

            redis.call('HINCRBY', key, 'upvote', -1)
            redis.call('HINCRBY', key, 'score', -1)
            redis.call('HINCRBY', key, 'downvote', 0)
            return "OK"
            "#
        )

        }

    } else {

        if payload.value == 1 {

        redis::Script::new(
            r#"
            local key = KEYS[1]

            redis.call('HINCRBY', key, 'downvote', 0)
            redis.call('HINCRBY', key, 'score', 1)
            redis.call('HINCRBY', key, 'upvote', 1)  
            return "OK"
            "#
        )

        } else {

                    redis::Script::new(
            r#"
            local key = KEYS[1]

            redis.call('HINCRBY', key, 'downvote', 1)
            redis.call('HINCRBY', key, 'score', -1)
            redis.call('HINCRBY', key, 'upvote', 0)  
            return "OK"
            "#
        )

        }
             
    };

    let upvote_redis_rsp: RedisResult<()> = redis_script.key(redis_key).invoke_async(&mut state.redis_connection.clone()).await;


    if upvote_redis_rsp.is_err() {
        return Err(Error::RedisUpdateFailed)
    }

    let postgres_conn = state.connection.clone();
 
 
    if payload.value == 0 {

        let user_liked_model = user_liked_posts::Entity::find_by_post_and_user_sov_id(&payload.post_sov_id, &payload.user_sov_id).one(&state.connection).await;
        
        
        let user_liked_rsp = user_liked_model.unwrap().unwrap();

         let _  = user_liked_rsp.delete(&state.connection).await;
   
    } else {

       if payload.prev_value == 0 {

        let liked_active_model = user_liked_posts::ActiveModel {
            id: Set(Uuid::new_v4()),
            user_sov_id: Set(payload.user_sov_id.clone()),
            post_sov_id: Set(payload.post_sov_id.clone()),
            value: Set(payload.value),
        created_at: Set(Utc::now().naive_utc()),
        updated_at: Set(Utc::now().naive_utc()),
        };

        let _ = liked_active_model.insert(&state.connection).await;


       } else {

           let _ =  user_liked_posts::Entity::update_many().col_expr(user_liked_posts::Column::Value, Expr::val(payload.value.clone()).into())
                .filter(user_liked_posts::Column::UserSovId.eq(payload.user_sov_id.clone()))
                .filter(user_liked_posts::Column::PostSovId.eq(payload.post_sov_id.clone()))
                .exec(&state.connection).await;
       }

    }
 
 
    let post_sov_id_clone = payload.post_sov_id.clone();
    let sql_statements_for_post = get_sql_statement_for_post_upvote(payload.value.clone(), payload.prev_value.clone());
    
 let now = Utc::now();

    tokio::spawn(async move {

       let _ =  postgres_conn.execute( Statement::from_sql_and_values(DbBackend::Postgres, 
            
       sql_statements_for_post
            , [post_sov_id_clone.into() , now.to_string().into()])
    ).await;

    });


        let body = Json(json!({
         "result": {
             "success": true
         },
     }));
 
     Ok(body)


}
 

pub async fn get_user_posts(
    state: State<DBState>,
    payload: Json<GetUserPostsOrCommentsPayload>
) -> APIResult<Json<Value>> {

    if payload.user_sov_id == "" {
        return Err(Error::MissingParams)
    }


    let posts_rsp = posts::Entity::find_by_user_id(&payload.user_sov_id).await.all(&state.connection).await;


    if posts_rsp.is_err() {
        return Err(Error::DBFetchError)
    }
    
        let body = Json(json!({
         "result": {
             "success": true
         },

           "posts": posts_rsp.unwrap(),
     }));
 
     Ok(body)



}



pub async fn get_user_comments(
    state: State<DBState>,
    payload: Json<GetUserPostsOrCommentsPayload>
) -> APIResult<Json<Value>> {

    if payload.user_sov_id == "" {
        return Err(Error::MissingParams)
    }


    let comments_rsp = comments::Entity::find_by_user_id(&payload.user_sov_id).await.all(&state.connection).await;


    if comments_rsp.is_err() {
        return Err(Error::DBFetchError)
    }
    
        let body = Json(json!({
         "result": {
             "success": true
         },

           "comments": comments_rsp.unwrap(),
     }));
 
     Ok(body)

}


pub async fn get_posts_for_subreddit(
    state: State<DBState>,
    payload: Json<GetPostsForSubreddit>
) -> APIResult<Json<Value>> {

    if payload.sub_sov_id == "" {
        return Err(Error::MissingParams)
    }

    let posts_rsp = posts::Entity::find_by_sub_id(&payload.sub_sov_id).await.all(&state.connection).await;


    if posts_rsp.is_err() {
        return Err(Error::DBFetchError)
    }
    
        let body = Json(json!({
         "result": {
             "success": true
         },

           "posts": posts_rsp.unwrap(),
     }));
 
     Ok(body)


}



pub async fn get_comments_for_posts(
    state: State<DBState>,
    payload: Json<GetCommentsForPosts>
) -> APIResult<Json<Value>> {

    if payload.post_sov_id == "" {
        return Err(Error::MissingParams)
    }

    let comments = comments::Entity::find_by_post_id(&payload.post_sov_id).await.all(&state.connection).await;


    if comments.is_err() {
        return Err(Error::DBFetchError)
    }
    
        let body = Json(json!({
         "result": {
             "success": true
         },

           "comments": comments.unwrap(),
     }));
 
     Ok(body)


}



pub async fn get_user_feed(
    state: State<DBState>,
    payload: Json<UserFeedPayload>
) -> APIResult<Json<Value>> {

    if payload.user_sov_id == "" {
        return Err(Error::MissingParams)
    }


    let result = posts::Entity::find().from_raw_sql(
        Statement::from_sql_and_values(DbBackend::Postgres, 
            r#"SELECT 
    p.post_sov_id,
    p.title,
    p.content,
    p.flair,
    p.upvote,
    p.downvote,
    p.score,
    s.subname,
    s.subdescription,
    u.username AS post_author,
    p.user_sov_id AS author_sov_id
FROM posts p
INNER JOIN subreddit s ON p.sub_sov_id = s.sub_sov_id
INNER JOIN users u ON p.user_sov_id = u.sov_id
INNER JOIN user_joined_subs ujs ON s.sub_sov_id = ujs.sub_sov_id
WHERE ujs.user_sov_id = $1
ORDER BY p.created_time DESC"#, [payload.user_sov_id.clone().into()])
    ).all(&state.connection).await;



      if result.is_err() {
        println!("The error while getting online friends is: {:?}", result.as_ref().err());
        println!("Error Is: {:?}", result.unwrap_err());
        return Err(Error::ErrorWhileFetchingUserFeed)
    }

    
          
   

    let body = Json(json!({
		"result": {
			"success": true,
		},
        "feed_posts": result.unwrap()
	}));


     Ok(body)

}


pub async fn get_user_subs(
    state: State<DBState>,
    Path(user_sov_id): Path<String>,
) -> APIResult<Json<Value>> {


    if user_sov_id == "" { 

        return Err(Error::MissingParams)
    }

    let sub_mods_rsp = sub_mods::Entity::find_by_user_id(&user_sov_id).all(&state.connection).await;

    if sub_mods_rsp.is_err() {
        return Err(Error::DBFetchError)
    }

    let body = Json(json!({
        "result": {
            "success": true
        },
        "sub_mods": sub_mods_rsp.unwrap()
    }));

    Ok(body)

    }
pub fn get_redis_key(schema: &str , primary_key: &str) -> String {
     let mut key = schema.to_string();
    
        key.push('_');
        key.push_str(primary_key);
    
    
    key
}


pub fn get_redis_key_from_keys(schema: &str , additional_key: &str , id_key: &str) -> String {
     let mut key = schema.to_string();
    key.push('_');
    key.push_str(additional_key);
    key.push('_');
    key.push_str(id_key);
    
    key
}

/// Search subreddits by subname (case-insensitive, contains)
pub async fn search_subreddits_by_name(
    state: State<DBState>,
    payload: Json<SearchSubredditsPayload>
) -> APIResult<Json<Value>> {
    if payload.query.is_empty() {
        return Err(Error::MissingParams);
    }
    let subreddits = subreddit::Entity::find()
        .filter(subreddit::Column::Subname.like(format!("%{}%", payload.query)))
        .all(&state.connection)
        .await;
    match subreddits {
        Ok(results) => {
            let body = Json(json!({
                "result": { "success": true },
                "subreddits": results
            }));
            Ok(body)
        },
        Err(_) => Err(Error::DBFetchError),
    }
}