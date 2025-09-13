use asknothingx2_util::serde::{
    deserialize_empty_array_as_none, deserialize_empty_object_as_none,
    serialize_none_as_empty_array, serialize_none_as_empty_object,
};
use serde::{Deserialize, Serialize};

use crate::{subscriptions::Subscription, types::Pagination};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSubscriptionResponse {
    pub data: Vec<Subscription>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcasterSubscriptionResponse {
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_array",
        deserialize_with = "deserialize_empty_array_as_none"
    )]
    pub data: Option<Vec<Subscription>>,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
    pub total: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub points: Option<u64>,
}
