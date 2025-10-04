mod request;
mod response;
mod types;

pub use request::{DropEntitlementRequest, UpdateEntitlementsRequest};
pub use response::{DropsEntitlementsResponse, UpdateDropEntitlementsResponse};
pub use types::{DropEntitlement, DropEntitlementStatus, FulfillmentStatus, UpdateDropEntitlement};

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
