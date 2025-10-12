use asknothingx2_util::serde::{deserialize_empty_object_as_none, serialize_none_as_empty_object};
use serde::{Deserialize, Serialize};

use crate::{
    types::{Pagination, VideoId},
    videos::Video,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideosResponse {
    pub data: Vec<Video>,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteVideosResponse {
    pub data: Vec<VideoId>,
}
