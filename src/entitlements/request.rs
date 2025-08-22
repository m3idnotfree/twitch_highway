use serde::{Deserialize, Serialize};

use crate::types::{GameId, Id, UserId};

define_request!(
    #[derive(Serialize)]
    DropEntitlementRequest<'a> {
        opts: {
            id: &'a [Id] => ID ; vec,
            user_id: &'a UserId => USER_ID,
            game_id: &'a GameId => GAME_ID,
            fulfillment_status: FulfillmentStatus,
        };
        into_query
    }
);

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
    #[derive(Default, Serialize)]
    UpdateEntitlementsRequest<'a>{
        opts: {
            entitlement_ids: &'a [&'a str],
            fulfillment_status: FulfillmentStatus
        };
        into_json
    }
);

#[cfg(test)]
mod tests {
    use crate::entitlements::request::FulfillmentStatus;

    #[test]
    fn fulfillment_status_enum() {
        let statuses = vec![
            (FulfillmentStatus::CLAIMED, "CLAIMED"),
            (FulfillmentStatus::FULFILLED, "FULFILLED"),
        ];

        for (status, expected_str) in statuses {
            assert_eq!(status.as_str(), expected_str);
            assert_eq!(status.as_ref(), expected_str);

            let status_string: String = status.into();
            assert_eq!(status_string, expected_str);

            let serialized = serde_json::to_string(&status).unwrap();
            assert_eq!(serialized, format!("\"{}\"", expected_str));

            let deserialized: FulfillmentStatus = serde_json::from_str(&serialized).unwrap();
            assert_eq!(deserialized.as_str(), expected_str);
        }
    }
}
