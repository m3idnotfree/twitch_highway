use asknothingx2_util::serde::{deserialize_empty_object_as_none, serialize_none_as_empty_object};
use serde::Deserialize;

use crate::types::Pagination;

use super::types::{AutoModSetting, AutoModStatus, BanUser, BannedUser};

#[derive(Debug, Deserialize)]
pub struct CheckAutoModStatusResponse {
    pub data: Vec<AutoModStatus>,
}

#[derive(Debug, Deserialize)]
pub struct AutoModSettingsResponse {
    pub data: Vec<AutoModSetting>,
}
#[derive(Debug, Deserialize)]
pub struct GetBannedUsersResponse {
    pub data: Vec<BannedUser>,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Deserialize)]
pub struct BanUsersResponse {
    pub data: Vec<BanUser>,
}
