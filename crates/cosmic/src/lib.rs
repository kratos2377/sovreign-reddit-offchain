use serde::{Deserialize , Serialize};



// Constants


pub const SOVEREIGN_REDDIT_ROLLUP_EVENTS: &str = "sovereign_reddit_rollup_events";




// AMPQ Keys

pub const USER_TX_KEY: &str = "Exchange/UserCreated";
pub const SUBREDDIT_TX_KEY: &str = "Exchange/SubredditCreated";
pub const POST_TX_KEY: &str = "Exchange/PostCreated";


// TXs Payload

#[derive(Serialize , Deserialize , Clone)]
pub struct UserTXPayload {
    pub tx_hash: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub number: u64,
    pub key: String,
    pub value: String,
    pub module: String,
}


#[derive(Serialize , Deserialize , Clone)]
pub struct SubRedditTXPayload {
    pub tx_hash: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub number: u64,
    pub key: String,
    pub value: String,
    pub module: String,
}



#[derive(Serialize , Deserialize , Clone)]
pub struct PostTXPayload {
    pub tx_hash: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub number: u64,
    pub key: String,
    pub value: String,
    pub module: String,
}


//Kafka Structs and keys

#[derive(Serialize , Deserialize , Clone)]
pub struct KafkaPublishPayload {

    pub payload: String,
    pub topic: String,
    pub key: String

}

pub const KAFKA_USER_PUBLISH_TOPIC: &str = "user_created";
pub const KAFKA_SUBREDDIT_PUBLISH_TOPIC: &str = "subreddit_created";
pub const KAFKA_POST_PUBLISH_TOPIC: &str = "post_created";



pub const KAFKA_USER_PUBLISH_KEY: &str = "user_created_key";
pub const KAFKA_SUBREDDIT_PUBLISH_KEY: &str = "subreddit_created_key";
pub const KAFKA_POST_PUBLISH_KEY: &str = "post_created_key";



// WS TX Structs

#[derive(Serialize , Deserialize)]
pub struct TransactionWSEvent {
    pub tx_hash: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub number: u64,
    pub key: String,
    pub value: String,
    pub module: String,
}