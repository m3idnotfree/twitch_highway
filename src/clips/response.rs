use asknothingx2_util::serde::{deserialize_empty_object_as_none, serialize_none_as_empty_object};
use serde::{Deserialize, Serialize};

use crate::types::Pagination;

use super::types::CreateClip;

#[derive(Debug, Deserialize)]
pub struct CreateClipsResponse {
    pub data: Vec<CreateClip>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClipsInfoResponse {
    pub data: Vec<CreateClip>,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
}
