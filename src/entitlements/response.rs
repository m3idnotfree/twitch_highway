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

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::entitlements::response::{
        DropsEntitlementsResponse, UpdateDropEntitlementsResponse,
    };

    #[test]
    fn drops_entitlements_response_deserialization() {
        let json_data = json!({
            "data": [
                {
                    "id": "entitlement123",
                    "benefit_id": "benefit456",
                    "timestamp": "2024-01-15T10:30:00Z",
                    "user_id": "user789",
                    "game_id": "game012",
                    "fulfillment_status": "CLAIMED",
                    "last_updated": "2024-01-15T11:00:00Z"
                },
                {
                    "id": "entitlement456",
                    "benefit_id": "benefit789",
                    "timestamp": "2024-01-15T09:15:00Z",
                    "user_id": "user123",
                    "game_id": "game345",
                    "fulfillment_status": "FULFILLED",
                    "last_updated": "2024-01-15T10:30:00Z"
                }
            ],
            "pagination": {
                "cursor": "eyJiI..."
            }
        });

        let response: DropsEntitlementsResponse = serde_json::from_value(json_data).unwrap();

        assert_eq!(response.data.len(), 2);
        assert!(response.pagination.is_some());
        assert_eq!(response.pagination.unwrap().cursor, "eyJiI...");

        let first_entitlement = &response.data[0];
        assert_eq!(first_entitlement.id.as_str(), "entitlement123");
        assert_eq!(first_entitlement.benefit_id, "benefit456");
        assert_eq!(first_entitlement.user_id.as_str(), "user789");
        assert_eq!(first_entitlement.game_id.as_str(), "game012");
        assert_eq!(first_entitlement.fulfillment_status.as_str(), "CLAIMED");

        let second_entitlement = &response.data[1];
        assert_eq!(second_entitlement.id.as_str(), "entitlement456");
        assert_eq!(second_entitlement.benefit_id, "benefit789");
        assert_eq!(second_entitlement.user_id.as_str(), "user123");
        assert_eq!(second_entitlement.game_id.as_str(), "game345");
        assert_eq!(second_entitlement.fulfillment_status.as_str(), "FULFILLED");
    }

    #[test]
    fn update_drop_entitlements_response_deserialization() {
        let json_data = json!({
            "data": [
                {
                    "status": "SUCCESS",
                    "ids": ["ent1", "ent2", "ent3"]
                },
                {
                    "status": "NOT_FOUND",
                    "ids": ["ent4", "ent5"]
                },
                {
                    "status": "INVALID_ID",
                    "ids": ["invalid_ent"]
                }
            ]
        });

        let response: UpdateDropEntitlementsResponse = serde_json::from_value(json_data).unwrap();

        assert_eq!(response.data.len(), 3);

        let first_result = &response.data[0];
        assert_eq!(first_result.ids.len(), 3);
        assert_eq!(first_result.ids[0].as_str(), "ent1");
        assert_eq!(first_result.ids[1].as_str(), "ent2");
        assert_eq!(first_result.ids[2].as_str(), "ent3");

        let second_result = &response.data[1];
        assert_eq!(second_result.ids.len(), 2);
        assert_eq!(second_result.ids[0].as_str(), "ent4");
        assert_eq!(second_result.ids[1].as_str(), "ent5");

        let third_result = &response.data[2];
        assert_eq!(third_result.ids.len(), 1);
        assert_eq!(third_result.ids[0].as_str(), "invalid_ent");
    }

    #[test]
    fn drops_entitlements_response_empty_data() {
        let json_data = json!({
            "data": []
        });

        let response: DropsEntitlementsResponse = serde_json::from_value(json_data).unwrap();
        assert_eq!(response.data.len(), 0);
        assert!(response.pagination.is_none());
    }

    #[test]
    fn drops_entitlements_response_empty_pagination() {
        let json_data = json!({
            "data": [
                {
                    "id": "single_entitlement",
                    "benefit_id": "single_benefit",
                    "timestamp": "2024-01-15T10:30:00Z",
                    "user_id": "single_user",
                    "game_id": "single_game",
                    "fulfillment_status": "CLAIMED",
                    "last_updated": "2024-01-15T11:00:00Z"
                }
            ],
            "pagination": {}
        });

        let response: DropsEntitlementsResponse = serde_json::from_value(json_data).unwrap();
        assert_eq!(response.data.len(), 1);
        assert!(response.pagination.is_none());
    }
}
