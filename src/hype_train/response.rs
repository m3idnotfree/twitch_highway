use asknothingx2_util::serde::{deserialize_empty_object_as_none, serialize_none_as_empty_object};
use serde::{Deserialize, Serialize};

use crate::{
    hype_train::{HypeTrain, HypeTrainStatus},
    types::Pagination,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HypeTrainResponse {
    pub data: Vec<HypeTrain>,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HypeTrainStatusResponse {
    pub data: Vec<HypeTrainStatus>,
}
