use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::types::{GameId, Id, UserId};

use super::request::FulfillmentStatus;

#[derive(Debug, Serialize, Deserialize)]
pub struct DropEntitlement {
    pub id: Id,
    pub benefit_id: String,
    pub timestamp: DateTime<FixedOffset>,
    pub user_id: UserId,
    pub game_id: GameId,
    pub fulfillment_status: FulfillmentStatus,
    pub last_updated: DateTime<FixedOffset>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDropEntitlement {
    pub status: DropEntitlementStatus,
    pub ids: Vec<Id>,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
pub enum DropEntitlementStatus {
    INVALID_ID,
    NOT_FOUND,
    SUCCESS,
    UNAUTHORIZED,
    UPDATE_FAILED,
}

#[cfg(test)]
mod tests {
    use crate::entitlements::types::DropEntitlementStatus;

    #[test]
    fn drop_entitlement_status_enum() {
        let statuses = vec![
            DropEntitlementStatus::INVALID_ID,
            DropEntitlementStatus::NOT_FOUND,
            DropEntitlementStatus::SUCCESS,
            DropEntitlementStatus::UNAUTHORIZED,
            DropEntitlementStatus::UPDATE_FAILED,
        ];

        for status in statuses {
            let serialized = serde_json::to_string(&status).unwrap();
            let deserialized: DropEntitlementStatus = serde_json::from_str(&serialized).unwrap();

            let re_serialized = serde_json::to_string(&deserialized).unwrap();
            assert_eq!(serialized, re_serialized);
        }
    }
}
