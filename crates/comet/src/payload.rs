use serde::{Deserialize, Serialize};

#[derive(Serialize , Deserialize)]
pub enum RedditCollections {
    USER,
    SUBREDDIT,
    POST,
}


impl RedditCollections {


    fn to_string(&self) -> String {
        match self {
            RedditCollections::USER => "USER".to_string(),
            RedditCollections::SUBREDDIT => "SUBREDDIT".to_string(),
            RedditCollections::POST => "POST".to_string(),
        }
    }

    // Convert string to enum
    fn from_string(s: &str) -> Result<Self, String> {
        match s {
            "USER" => Ok(RedditCollections::USER),
            "SUBREDDIT" => Ok(RedditCollections::SUBREDDIT),
            "POST" => Ok(RedditCollections::POST),
            _ => Err(format!("Unknown Collection: {}", s)),
        }
    }

}

#[derive(Serialize , Deserialize)]
pub enum ChangeType {
    CREATED,
    UPDATED
}


impl ChangeType {


    fn to_string(&self) -> String {
        match self {
            ChangeType::CREATED => "CREATED".to_string(),
            ChangeType::UPDATED => "UPDATED".to_string(),
        }
    }

    // Convert string to enum
    fn from_string(s: &str) -> Result<Self, String> {
        match s {
            "CREATED" => Ok(ChangeType::CREATED),
            "UPDATED" => Ok(ChangeType::UPDATED),
            _ => Err(format!("Unknown Changetype: {}", s)),
        }
    }

}



#[derive(Serialize , Deserialize)]
pub struct StateChangePayload {
    pub state: RedditCollections,
    pub change: String,
    pub address: String,
    pub change_type: ChangeType

}