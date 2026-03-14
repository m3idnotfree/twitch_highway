use serde::{Deserialize, Serialize};

use crate::BroadcasterId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelUpdate {
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub title: String,
    pub language: String,
    pub category_id: String,
    pub category_name: String,
    pub content_classification_labels: Vec<String>,
}
