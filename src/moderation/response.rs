use asknothingx2_util::serde::{deserialize_empty_object_as_none, serialize_none_as_empty_object};
use serde::{Deserialize, Serialize};

use crate::types::Pagination;

use super::types::{
    AutoModSetting, AutoModStatus, BanUser, BannedUser, BlockedTerm, ModeratedChannel, Moderator,
    ShieldModeStatus, UnbanRequest, WarnChatUserff,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckAutoModStatusResponse {
    pub data: Vec<AutoModStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AutoModSettingsResponse {
    pub data: Vec<AutoModSetting>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GetBannedUsersResponse {
    pub data: Vec<BannedUser>,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BanUsersResponse {
    pub data: Vec<BanUser>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnbanRequestResponse {
    pub data: Vec<UnbanRequest>,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockedTermsResponse {
    pub data: Vec<BlockedTerm>,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModeratedChannelResponse {
    pub data: Vec<ModeratedChannel>,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModeratorsResponse {
    pub data: Vec<Moderator>,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShieldModeStatusResponse {
    pub data: Vec<ShieldModeStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WarnChatUsersResponse {
    pub data: Vec<WarnChatUserff>,
}
