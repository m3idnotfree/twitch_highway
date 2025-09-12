use asknothingx2_util::serde::{deserialize_empty_object_as_none, serialize_none_as_empty_object};
use serde::{Deserialize, Serialize};

use crate::{subscriptions::Subscription, types::Pagination};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSubscriptionResponse {
    pub data: Vec<Subscription>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcasterSubscriptionResponse {
    pub data: Vec<Subscription>,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
    pub total: u64,
    pub points: u64,
}
