use std::collections::HashMap;

use serde::{Deserialize , Serialize};
use serde_json::Value;
pub mod payload;
pub mod utils;

// Constants


pub const SOVEREIGN_REDDIT_ROLLUP_EVENTS: &str = "sovereign_reddit_rollup_events";




// AMPQ Keys

pub const USER_TX_KEY: &str = "Reddit/UserCreatedEvent";
pub const SUBREDDIT_TX_KEY: &str = "Reddit/SubRedditCreatedEvent";
pub const POST_TX_KEY: &str = "Reddit/PostCreatedEvent";


// TXs Payload

#[derive(Serialize , Deserialize , Clone , Debug)]
pub struct UserTXPayload {
    pub tx_hash: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub number: u64,
    pub key: String,
    #[serde(flatten)]
    pub additional_properties: HashMap<String, Value>,

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

pub const KAFKA_USER_PUBLISH_TOPIC: &str = "user";
pub const KAFKA_SUBREDDIT_PUBLISH_TOPIC: &str = "subreddit";
pub const KAFKA_POST_PUBLISH_TOPIC: &str = "post";



pub const KAFKA_USER_PUBLISH_KEY: &str = "user_created_key";
pub const KAFKA_SUBREDDIT_PUBLISH_KEY: &str = "subreddit_created_key";
pub const KAFKA_POST_PUBLISH_KEY: &str = "post_created_key";



// WS TX Structs

#[derive(Serialize , Deserialize , Clone , Debug)]
pub struct TransactionWSEvent {
    pub tx_hash: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub number: u64,
    pub key: String,
    #[serde(flatten)]
    pub additional_properties: HashMap<String, Value>,
}



//Schema

pub const USER_SCHEMA: &str = "user";
pub const SUBREDDIT_SCHEMA: &str = "subreddit";
pub const POST_SCHEMA: &str = "post";
