use asknothingx2_util::serde::{deserialize_empty_object_as_none, serialize_none_as_empty_object};
use serde::{Deserialize, Serialize};

use crate::types::Pagination;

use super::types::{ChannelEditor, ChannelInfo, FollowedChannel};

#[derive(Debug, Deserialize)]
pub struct ChannelInfoResponse {
    pub data: Vec<ChannelInfo>,
}

#[derive(Debug, Deserialize)]
pub struct ChannelEditorsResponse {
    pub data: Vec<ChannelEditor>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelFollowRequestResponse {
    pub data: Option<Vec<FollowedChannel>>,
    pub total: u64,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
}
