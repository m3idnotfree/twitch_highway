use asknothingx2_util::serde::{deserialize_empty_object_as_none, serialize_none_as_empty_object};
use serde::{Deserialize, Serialize};

use crate::{
    channels::{ChannelEditor, ChannelFollower, ChannelInfo, FollowedChannel},
    types::Pagination,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelInfoResponse {
    pub data: Vec<ChannelInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelEditorsResponse {
    pub data: Vec<ChannelEditor>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FollowerdChannelsResponse {
    pub data: Option<Vec<FollowedChannel>>,
    pub total: u64,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelFollowersResponse {
    pub data: Option<Vec<ChannelFollower>>,
    pub total: u64,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
}
