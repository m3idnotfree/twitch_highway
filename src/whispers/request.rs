use serde::Serialize;

use crate::AsBody;

#[derive(Debug, Serialize)]
pub struct SendWhisperRequest {
    message: String,
}

impl SendWhisperRequest {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl AsBody for SendWhisperRequest {
    fn as_body(&self) -> Option<String> {
        Some(serde_json::to_string(&self).unwrap())
    }
}
