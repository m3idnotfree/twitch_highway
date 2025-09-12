use serde::Serialize;

use crate::{
    entitlements::FulfillmentStatus,
    types::{GameId, Id, UserId},
};

define_request!(
    #[derive(Debug, Clone, Serialize)]
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

define_request!(
    #[derive(Debug, Clone, Copy, Default, Serialize)]
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
    use crate::entitlements::FulfillmentStatus;

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
