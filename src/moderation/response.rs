use asknothingx2_util::serde::{
    deserialize_empty_array_as_none, deserialize_empty_object_as_none,
    serialize_none_as_empty_array, serialize_none_as_empty_object,
};
use serde::{Deserialize, Serialize};

use crate::{
    moderation::{
        AutoModSetting, AutoModStatus, BanUser, BannedUser, BlockedTerm, ModeratedChannel,
        Moderator, ShieldModeStatus, SuspiciousUser, UnbanRequest, WarnChatUser,
    },
    types::Pagination,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckAutoModStatusResponse {
    pub data: Vec<AutoModStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoModSettingsResponse {
    pub data: Vec<AutoModSetting>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBannedUsersResponse {
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_array",
        deserialize_with = "deserialize_empty_array_as_none"
    )]
    pub data: Option<Vec<BannedUser>>,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BanUsersResponse {
    pub data: Vec<BanUser>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnbanRequestResponse {
    pub data: Vec<UnbanRequest>,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockedTermsResponse {
    pub data: Vec<BlockedTerm>,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModeratedChannelResponse {
    pub data: Vec<ModeratedChannel>,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModeratorsResponse {
    pub data: Vec<Moderator>,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShieldModeStatusResponse {
    pub data: Vec<ShieldModeStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarnChatUsersResponse {
    pub data: Vec<WarnChatUser>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuspiciousResponse {
    pub data: Vec<SuspiciousUser>,
}
