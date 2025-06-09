use reqwest::Body;
use serde::{Deserialize , Serialize};



#[derive(Clone , Deserialize , Serialize )]
pub struct CreateAndSaveModel {
    pub schema_type: String,
    pub data: String
}

impl Into<reqwest::Body> for CreateAndSaveModel {
    fn into(self) -> reqwest::Body {
        // Serialize to JSON and convert to Body
        match serde_json::to_vec(&self) {
            Ok(json_bytes) => Body::from(json_bytes),
            Err(_) => Body::from("{}"), // Fallback to empty JSON object on serialization error
        }
    }
}

impl Into<reqwest::Body> for &CreateAndSaveModel {
    fn into(self) -> reqwest::Body {
        // Also implement for references to avoid unnecessary cloning
        match serde_json::to_vec(self) {
            Ok(json_bytes) => Body::from(json_bytes),
            Err(_) => Body::from("{}"), // Fallback to empty JSON object on serialization error
        }
    }
}

// Alternative implementation with Result handling if you prefer explicit error handling
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
