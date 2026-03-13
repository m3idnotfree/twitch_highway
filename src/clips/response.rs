use asknothingx2_util::serde::{deserialize_empty_object_as_none, serialize_none_as_empty_object};
use serde::{Deserialize, Serialize};

use crate::{
    clips::{Clip, ClipDownload, NewClip},
    types::Pagination,
};

#[derive(Debug, Clone, Deserialize)]
pub struct NewClipResponse {
    pub data: Vec<NewClip>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipsInfoResponse {
    pub data: Vec<Clip>,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ClipsDownloadResponse {
    pub data: Vec<ClipDownload>,
}
