use serde::{Deserialize , Serialize};


#[derive(Serialize , Deserialize)]
pub struct KafkaGeneralEvent {

    pub payload: String,
    pub topic: String,
    pub key: String

}