use serde::{Deserialize, Serialize};

use crate::{
    base::{IntoQueryPairs, QueryParams},
    types::{Id, UserId, AFTER, FIRST, ID, USER_ID},
    RequestBody,
};

request_struct!(
    #[derive(Debug, Default, Serialize, Deserialize)]
    DropEntitlementRequest {
        id: Vec<Id>,
        user_id: UserId,
        game_id: String,
        fulfillment_status: FulfillmentStatus,
        after: String,
        first: u64
    }
);

impl IntoQueryPairs for DropEntitlementRequest {
    fn into_query_pairs(self) -> Vec<(&'static str, String)> {
        let mut params = QueryParams::new();
        params
            .extend_opt(self.id.map(|id| id.into_iter().map(|p| (ID, p))))
            .push_opt(USER_ID, self.user_id)
            .push_opt("game_id", self.game_id)
            .push_opt("fulfillment_status", self.fulfillment_status)
            .push_opt(AFTER, self.after)
            .push_opt(FIRST, self.first.map(|x| x.to_string()));

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
#[derive(Debug, Default,Serialize, Deserialize)]
UpdateEntitlementsRequest{
    entitlement_ids: Vec<String>,
    fulfillment_status: FulfillmentStatus
}
);

impl RequestBody for UpdateEntitlementsRequest {
    fn as_body(&self) -> Option<String> {
        Some(serde_json::to_string(&self).unwrap())
    }
}
