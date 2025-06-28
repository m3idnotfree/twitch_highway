use serde::{Deserialize, Serialize};

use crate::types::{GameId, Id, UserId};

define_request!(
    #[derive(Serialize, Deserialize)]
    DropEntitlementRequest {
        opts: {
            id: Vec<Id> => ID ; vec,
            user_id: UserId => USER_ID,
            game_id: GameId => GAME_ID,
            fulfillment_status: FulfillmentStatus,
        };
        apply_to_url
    }
);

#[derive(Debug, Serialize, Deserialize)]
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
    fn from(status: FulfillmentStatus) -> Self {
        match status {
            FulfillmentStatus::CLAIMED => "CLAIMED".to_string(),
            FulfillmentStatus::FULFILLED => "FULFILLED".to_string(),
        }
    }
}

impl AsRef<str> for FulfillmentStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

define_request!(
    #[derive(Default, Serialize, Deserialize)]
    UpdateEntitlementsRequest{
        opts: {
            entitlement_ids: Vec<String>,
            fulfillment_status: FulfillmentStatus
        };
        to_json
    }
);
