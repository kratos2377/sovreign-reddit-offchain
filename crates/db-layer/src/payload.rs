use serde::{Deserialize , Serialize};



#[derive(Clone , Deserialize , Serialize)]
pub struct CreateAndSaveModel {
    pub schema_type: String,
    pub data: String
}