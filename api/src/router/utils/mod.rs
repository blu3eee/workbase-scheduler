use axum::{ response::Response, body::{ Body, to_bytes } };
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::result::Result;

pub async fn extract_response_body<T>(response: Response<Body>) -> Result<T, String>
    where T: DeserializeOwned
{
    // Convert the body to bytes
    let body_bytes = to_bytes(response.into_body(), usize::MAX).await.map_err(|e| e.to_string())?;

    // Convert bytes to String
    let body_string = String::from_utf8(body_bytes.to_vec()).map_err(|e| e.to_string())?;

    // Deserialize the string into JSON
    let body_json: Value = serde_json::from_str(&body_string).map_err(|e| e.to_string())?;

    // Extract the "data" field and deserialize it into T
    if let Some(data) = body_json.get("data") {
        serde_json::from_value(data.clone()).map_err(|e| e.to_string())
    } else {
        Err("Data field not found in response".to_string())
    }
}
