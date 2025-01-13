use asknothingx2_util::serde::{deserialize_empty_object_as_none, serialize_none_as_empty_object};
use serde::{Deserialize, Serialize};

use crate::types::Pagination;

use super::types::{
    BitsProductExtension, ConfigurationSegment, Extension, LiveChannel, SecretData,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigurationSegmentResponse {
    pub data: Vec<ConfigurationSegment>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionLiveChannelsRespnose {
    pub data: Vec<LiveChannel>,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionSecretsResponse {
    pub data: Vec<SecretData>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionsResponse {
    pub data: Vec<Extension>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionsBitsProductsResponse {
    pub data: Vec<BitsProductExtension>,
}
