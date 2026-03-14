use serde::{Deserialize, Serialize};

use crate::{UserId, WhisperId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhisperReceived {
    pub from_user_id: UserId,
    pub from_user_name: String,
    pub from_user_login: String,
    pub to_user_id: UserId,
    pub to_user_name: String,
    pub to_user_login: String,
    pub whisper_id: WhisperId,
    pub whisper: Whisper,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Whisper {
    pub text: String,
}
