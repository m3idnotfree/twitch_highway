use serde::Serialize;

use crate::IntoRequestBody;

#[derive(Debug, Serialize)]
pub struct SendWhisperBody {
    message: String,
}

impl SendWhisperBody {
    pub fn new<T: Into<String>>(message: T) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl IntoRequestBody for SendWhisperBody {
    fn as_body(&self) -> Option<String> {
        Some(serde_json::to_string(&self).unwrap())
    }
}
