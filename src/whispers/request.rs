use serde::Serialize;

use crate::RequestBody;

#[derive(Debug, Serialize)]
pub struct SendWhisperRequest {
    message: String,
}

impl SendWhisperRequest {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl RequestBody for SendWhisperRequest {
    fn as_body(&self) -> Option<String> {
        Some(serde_json::to_string(&self).unwrap())
    }
}
