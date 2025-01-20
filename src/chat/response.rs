use asknothingx2_util::serde::{deserialize_empty_object_as_none, serialize_none_as_empty_object};
use serde::{Deserialize, Serialize};

use crate::types::Pagination;

use super::types::{
    Badge, ChatSetting, Chatter, Emote, MessageResponse, SharedChatSession, UserColor,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct BadgesResponse {
    pub data: Vec<Badge>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatSettingResponse {
    pub data: Vec<ChatSetting>,
}

/// <https://dev.twitch.tv/docs/api/reference/#get-chatters>
/// spec paginarion must include event empty.
/// but mock server remove paginiation field
#[derive(Debug, Serialize, Deserialize)]
pub struct ChattersResponse {
    pub data: Vec<Chatter>,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
    pub total: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmotesResponse {
    pub data: Vec<Emote>,
    pub template: String,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendChatMessageResponse {
    pub data: Vec<MessageResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SharedChatSessionResponse {
    pub data: Vec<SharedChatSession>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersColorResponse {
    data: Vec<UserColor>,
}
