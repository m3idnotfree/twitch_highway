use asknothingx2_util::api::Method;
use request::{DropEntitlementRequest, UpdateEntitlementsRequest};
use response::{DropsEntitlementsResponse, UpdateDropEntitlementsResponse};

use crate::{
    request::{EndpointType, TwitchAPIRequest},
    types::PaginationQuery,
    TwitchAPI,
};

pub mod request;
pub mod response;
pub mod types;

twitch_api_trait! {
    #[cfg_attr(docsrs, doc(cfg(feature = "entitlements")))]
    trait EntitlementsAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#get-drops-entitlements>
        fn get_drops_entitlements(
            &self,
            opts: Option<DropEntitlementRequest>,
            pagination: Option<PaginationQuery>,
        ) -> DropsEntitlementsResponse;
        /// <https://dev.twitch.tv/docs/api/reference/#update-drops-entitlements>
        fn update_drops_entitlements(
            &self,
            opts: Option<UpdateEntitlementsRequest>,
        ) -> UpdateDropEntitlementsResponse;
    }
    impl {
        get_drops_entitlements => {
            endpoint_type: EndpointType::GetDropsEntitlements,
            method: Method::GET,
            path: ["entitlements", "drops"],
            query_params: {
                opt_into_query(opts),
                pagination(pagination)
            }
        }
        update_drops_entitlements => {
            endpoint_type: EndpointType::UpdateDropsEntitlements,
            method: Method::PATCH,
            path: ["entitlements", "drops"],
            headers:[json],
            body: opts.and_then(|o| o.into_json())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        entitlements::{
            request::{DropEntitlementRequest, FulfillmentStatus, UpdateEntitlementsRequest},
            EntitlementsAPI,
        },
        test_utils::TwitchApiTest,
        types::{GameId, Id, PaginationQuery, UserId},
    };

    #[tokio::test]
    pub(crate) async fn get_drops_entitlements() {
        let suite = TwitchApiTest::new().await;

        suite.mock_entitlements_success().await;

        let request = DropEntitlementRequest::new()
            .user_id(UserId::new("user789"))
            .game_id(GameId::new("game012"))
            .fulfillment_status(FulfillmentStatus::CLAIMED);

        let pagination = PaginationQuery::new().first(20);

        let response = suite
            .execute("/entitlements/drops", |api| {
                api.get_drops_entitlements(Some(request), Some(pagination))
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 2);
        assert!(response.pagination.is_some());

        let first_entitlement = &response.data[0];
        assert_eq!(first_entitlement.id.as_str(), "entitlement123");
        assert_eq!(first_entitlement.user_id.as_str(), "user789");
        assert_eq!(first_entitlement.game_id.as_str(), "game012");
        assert_eq!(first_entitlement.fulfillment_status.as_str(), "CLAIMED");

        let second_entitlement = &response.data[1];
        assert_eq!(second_entitlement.id.as_str(), "entitlement456");
        assert_eq!(second_entitlement.fulfillment_status.as_str(), "FULFILLED");
    }

    #[tokio::test]
    async fn get_drops_entitlements_with_ids_endpoint() {
        let suite = TwitchApiTest::new().await;

        suite.mock_entitlements_success().await;

        let request = DropEntitlementRequest::new()
            .id(vec![Id::new("specific_ent_1"), Id::new("specific_ent_2")]);

        let response = suite
            .execute("/entitlements/drops", |api| {
                api.get_drops_entitlements(Some(request), None)
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 2);
        assert!(response.pagination.is_none());

        let first_entitlement = &response.data[0];
        assert_eq!(first_entitlement.id.as_str(), "specific_ent_1");
        assert_eq!(first_entitlement.fulfillment_status.as_str(), "CLAIMED");

        let second_entitlement = &response.data[1];
        assert_eq!(second_entitlement.id.as_str(), "specific_ent_2");
        assert_eq!(second_entitlement.fulfillment_status.as_str(), "FULFILLED");
    }

    #[tokio::test]
    async fn get_drops_entitlements_no_filters_endpoint() {
        let suite = TwitchApiTest::new().await;

        suite.mock_entitlements_success().await;

        let response = suite
            .execute("/entitlements/drops", |api| {
                api.get_drops_entitlements(None, None)
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 1);
        assert!(response.pagination.is_none());

        let entitlement = &response.data[0];
        assert_eq!(entitlement.id.as_str(), "all_ent_1");
        assert_eq!(entitlement.benefit_id, "all_benefit_1");
    }

    #[tokio::test]
    pub(crate) async fn update_drops_entitlements() {
        let suite = TwitchApiTest::new().await;

        suite.mock_entitlements_success().await;

        let request = UpdateEntitlementsRequest::new()
            .entitlement_ids(vec![
                "update_ent_1".to_string(),
                "update_ent_2".to_string(),
                "update_ent_3".to_string(),
            ])
            .fulfillment_status(FulfillmentStatus::FULFILLED);

        let response = suite
            .execute("/entitlements/drops", |api| {
                api.update_drops_entitlements(Some(request))
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 2);

        let first_result = &response.data[0];
        assert_eq!(first_result.ids.len(), 2);
        assert_eq!(first_result.ids[0].as_str(), "update_ent_1");
        assert_eq!(first_result.ids[1].as_str(), "update_ent_2");

        let second_result = &response.data[1];
        assert_eq!(second_result.ids.len(), 1);
        assert_eq!(second_result.ids[0].as_str(), "update_ent_3");
    }

    #[tokio::test]
    async fn update_drops_entitlements_no_request_endpoint() {
        let suite = TwitchApiTest::new().await;

        suite.mock_entitlements_extra().await;

        let response = suite
            .execute("/entitlements/drops", |api| {
                api.update_drops_entitlements(None)
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 0);
    }

    #[tokio::test]
    async fn entitlements_api_error_response() {
        let suite = TwitchApiTest::new().await;

        suite.mock_entitlements_failure().await;

        let response = suite
            .execute("/entitlements/drops", |api| {
                api.get_drops_entitlements(None, None)
            })
            .send()
            .await;

        match response {
            Ok(response) => {
                panic!("Expected Error, got: {response:?}")
            }
            Err(e) => {
                assert!(e.is_api());
            }
        }
    }
}
