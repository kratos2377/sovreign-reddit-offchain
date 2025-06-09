use axum::{extract::State, Json};
use cosmic::payload::CreateAndSaveModel;
use serde_json::json;

use crate::{state::DBState};
use crate::error::{Error , Result as APIResult};
use serde_json::Value;

pub async fn create_and_save_model(
        state: State<DBState>,
	payload: Json<CreateAndSaveModel>,
) -> APIResult<Json<Value>> {


    if payload.schema_type == "" || payload.data == "" {
        return Err(Error::MissingParams)
    }


    let postgres_conn = state.connection.clone();

    tokio::spawn( async move {
        


    });

       let body = Json(json!({
         "result": {
             "success": true
         },
     }));
 
     Ok(body)

}