use reqwest::Body;
use serde::{Deserialize , Serialize};



#[derive(Clone , Deserialize , Serialize )]
pub struct CreateAndSaveModel {
    pub schema_type: String,
    pub data: String
}

impl Into<reqwest::Body> for CreateAndSaveModel {
    fn into(self) -> reqwest::Body {
     
        match serde_json::to_vec(&self) {
            Ok(json_bytes) => Body::from(json_bytes),
            Err(_) => Body::from("{}"), 
        }
    }
}

impl Into<reqwest::Body> for &CreateAndSaveModel {
    fn into(self) -> reqwest::Body {
       
        match serde_json::to_vec(self) {
            Ok(json_bytes) => Body::from(json_bytes),
            Err(_) => Body::from("{}"),
        }
    }
}

impl CreateAndSaveModel {
    pub fn try_into_body(self) -> Result<reqwest::Body, serde_json::Error> {
        let json_bytes = serde_json::to_vec(&self)?;
        Ok(Body::from(json_bytes))
    }

    pub fn try_into_body_ref(&self) -> Result<reqwest::Body, serde_json::Error> {
        let json_bytes = serde_json::to_vec(self)?;
        Ok(Body::from(json_bytes))
    }
}





#[derive(Clone , Deserialize , Serialize )]
pub struct JoinOrUnjoinSub {
    pub user_sov_id: String,
    pub sub_sov_id: String,
    pub value: i32
}


#[derive(Clone , Deserialize , Serialize )]
pub struct AddCommentPayload {
    pub user_sov_id: String,
    pub sub_sov_id: String,
    pub post_sov_id: String,
    pub content: String,
}


#[derive(Clone , Deserialize , Serialize )]
pub struct LikeOrDislikePost {
    pub post_sov_id: String,
    pub user_sov_id: String,
    pub value: i32,
    pub prev_value: i32
}


#[derive(Clone , Deserialize , Serialize )]
pub struct LikeOrDislikeComment {
    pub comment_sov_id: String,
    pub user_sov_id: String,
    pub value: i32,
    pub prev_value: i32
}



#[derive(Clone , Deserialize , Serialize )]
pub struct GetUserPostsOrCommentsPayload {
    pub user_sov_id: String,
}


#[derive(Clone , Deserialize , Serialize )]
pub struct GetPostsForSubreddit {
    pub sub_sov_id: String,
}

#[derive(Clone , Deserialize , Serialize )]
pub struct GetCommentsForPosts {
    pub post_sov_id: String,
}

#[derive(Clone , Deserialize , Serialize)]
pub struct UserFeedPayload {
    pub user_sov_id: String
}

#[derive(Clone , Deserialize , Serialize)]
pub struct SearchSubredditsPayload {
    pub query: String
}