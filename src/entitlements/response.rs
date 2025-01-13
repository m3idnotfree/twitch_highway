use asknothingx2_util::serde::{deserialize_empty_object_as_none, serialize_none_as_empty_object};
use serde::{Deserialize, Serialize};

use crate::types::Pagination;

use super::types::{DropEntitlement, UpdateDropEntitlement};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DropsEntitlementsResponse {
    pub data: Vec<DropEntitlement>,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDropEntitlementsResponse {
    pub data: Vec<UpdateDropEntitlement>,
}
