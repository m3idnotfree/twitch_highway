use asknothingx2_util::serde::{deserialize_empty_object_as_none, serialize_none_as_empty_object};
use serde::{Deserialize, Serialize};

use crate::types::Pagination;

use super::types::{Marker, Stream, StreamKey, StreamMarker};

#[derive(Debug, Deserialize)]
pub struct StreamKeyResponse {
    pub data: Vec<StreamKey>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamsResponse {
    pub data: Vec<Stream>,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
}
#[derive(Debug, Deserialize)]
pub struct CreateStreamMarkerResponse {
    pub data: Vec<Marker>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetStreamMarkersResponse {
    pub data: Vec<StreamMarker>,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
}
