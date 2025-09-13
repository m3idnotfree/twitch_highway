use asknothingx2_util::serde::{
    deserialize_empty_array_as_none, deserialize_empty_object_as_none,
    serialize_none_as_empty_array, serialize_none_as_empty_object,
};
use serde::{Deserialize, Serialize};

use crate::{
    streams::{Marker, Stream, StreamKey, StreamMarker},
    types::Pagination,
};

#[derive(Debug, Clone, Deserialize)]
pub struct StreamKeyResponse {
    pub data: Vec<StreamKey>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamsResponse {
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_array",
        deserialize_with = "deserialize_empty_array_as_none"
    )]
    pub data: Option<Vec<Stream>>,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateStreamMarkerResponse {
    pub data: Vec<Marker>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetStreamMarkersResponse {
    pub data: Vec<StreamMarker>,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
}
