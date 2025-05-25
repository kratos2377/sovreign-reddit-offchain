use std::time::Duration;

use axum::extract::State;
use cosmic::KafkaGeneralEvent;
use rdkafka::{error::KafkaError, producer::{FutureProducer, FutureRecord, Producer}, util::Timeout};
use serde::{Deserialize, Serialize};
use error::Result as APIResult;
use serde_json::{json, Value};
use crate::{error::{self, Error}, payload::StateChangePayload, state::AppDBState};
use axum::Json;
use futures_util::future;


pub async fn publish_state_change_to_topic(
    state: State<AppDBState>,
	payload: Json<StateChangePayload>,
) -> APIResult<Json<Value>> { 


    if payload.address == "" || payload.change == ""  {
        return Err(Error::MissingParams)
    }



    let kafka_events = vec![

    KafkaGeneralEvent { payload: payload.change.clone(), topic: "topic".to_string(), key: "key".to_string() }

    ];

    state.producer.begin_transaction().unwrap();

     let kafka_result = future::try_join_all(kafka_events.iter().map(|event| async  {
        let delivery_result = state.producer
        .send(
            FutureRecord::to(&event.topic)
                    .payload(&event.payload)
                    .key(&event.key.clone()),
            Duration::from_secs(3),
        )
        .await;

    // This will be executed when the result is received.
  //  println!("Delivery status for message {} received", i);
    delivery_result

    })

    ).await;

    match kafka_result {
        Ok(_) => (),
        Err(e) => return Err(Error::MissingParams),
    }

    state.producer.commit_transaction(Timeout::from(Duration::from_secs(5))).unwrap(); 

       let body = Json(json!({
		"result": {
			"success": true
		}
	}));


     Ok(body)

}