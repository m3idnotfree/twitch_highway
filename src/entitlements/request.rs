use serde::{Deserialize, Serialize};

use crate::{
    base::{IntoQueryPairs, QueryParams},
    types::{
        constants::{GAME_ID, ID, USER_ID},
        GameId, Id, UserId,
    },
    IntoRequestBody,
};

request_struct!(
    #[derive(Serialize, Deserialize)]
    DropEntitlementRequest {
        game_id: GameId,
        id: Vec<Id>,
        user_id: UserId,
        fulfillment_status: FulfillmentStatus,
    }
);

impl IntoQueryPairs for DropEntitlementRequest {
    fn into_query_pairs(self) -> Vec<(&'static str, String)> {
        let mut params = QueryParams::new();
        params
            .extend_opt(self.id.map(|id| id.into_iter().map(|p| (ID, p))))
            .push_opt(USER_ID, self.user_id)
            .push_opt(GAME_ID, self.game_id)
            .push_opt("fulfillment_status", self.fulfillment_status);

        params.build()
    }
}

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

request_struct!(
    #[derive(Serialize, Deserialize)]
    UpdateEntitlementsRequest{
        entitlement_ids: Vec<String>,
        fulfillment_status: FulfillmentStatus
    }
);

impl IntoRequestBody for UpdateEntitlementsRequest {
    fn as_body(&self) -> Option<String> {
        Some(serde_json::to_string(&self).unwrap())
    }
}
