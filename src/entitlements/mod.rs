pub mod request;
pub mod response;
pub mod types;

use request::{DropEntitlementRequest, UpdateEntitlementsRequest};
use response::{DropsEntitlementsResponse, UpdateDropEntitlementsResponse};

use crate::types::PaginationQuery;

endpoints! {
    EntitlementsAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#get-drops-entitlements>
        fn get_drops_entitlements(
            &self,
            opts: Option<DropEntitlementRequest>,
            pagination: Option<PaginationQuery>,
        ) -> DropsEntitlementsResponse {
            endpoint_type: GetDropsEntitlements,
            method: GET,
            path: ["entitlements", "drops"],
            query_params: {
                opt_into_query(opts),
                pagination(pagination)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#update-drops-entitlements>
        fn update_drops_entitlements(
            &self,
            opts: Option<UpdateEntitlementsRequest>,
        ) -> UpdateDropEntitlementsResponse {
            endpoint_type: UpdateDropsEntitlements,
            method: PATCH,
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
        types::{GameId, UserId},
    };

    api_test!(
        get_drops_entitlements,
        [
            Some(
                DropEntitlementRequest::new()
                    .user_id(&UserId::from("25009227"))
                    .game_id(&GameId::from("33214"))
            ),
            None
        ]
    );
    api_test!(
        update_drops_entitlements,
        [Some(
            UpdateEntitlementsRequest::new()
                .entitlement_ids(&[
                    "fb78259e-fb81-4d1b-8333-34a06ffc24c0",
                    "862750a5-265e-4ab6-9f0a-c64df3d54dd0",
                    "d8879baa-3966-4d10-8856-15fdd62cce02",
                    "9a290126-7e3b-4f66-a9ae-551537893b65"
                ])
                .fulfillment_status(FulfillmentStatus::FULFILLED)
        )]
    );
}
