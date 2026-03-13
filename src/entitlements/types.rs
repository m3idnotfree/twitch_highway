use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::types::{EntitlementId, GameId, UserId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropEntitlement {
    pub id: EntitlementId,
    pub benefit_id: String,
    pub timestamp: DateTime<FixedOffset>,
    pub user_id: UserId,
    pub game_id: GameId,
    pub fulfillment_status: FulfillmentStatus,
    pub last_updated: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDropEntitlement {
    pub status: DropEntitlementStatus,
    pub ids: Vec<EntitlementId>,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DropEntitlementStatus {
    INVALID_ID,
    NOT_FOUND,
    SUCCESS,
    UNAUTHORIZED,
    UPDATE_FAILED,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FulfillmentStatus {
    CLAIMED,
    FULFILLED,
}

impl FulfillmentStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::CLAIMED => "CLAIMED",
            Self::FULFILLED => "FULFILLED",
        }
    }
}

impl From<FulfillmentStatus> for String {
    fn from(value: FulfillmentStatus) -> Self {
        value.as_str().to_string()
    }
}

impl AsRef<str> for FulfillmentStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
