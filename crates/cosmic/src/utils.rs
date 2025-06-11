use serde_json::{Map, Value};

pub fn remove_duplicate_keys(json: &str) -> Result<String, serde_json::Error> {
    let mut value: Value = serde_json::from_str(json)?;
    if let Value::Object(map) = value {
        let mut unique_map = Map::new();
        for (k, v) in map {
            unique_map.insert(k, v);
        }
        value = Value::Object(unique_map);
    }
    serde_json::to_string(&value)
}