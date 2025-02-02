use asknothingx2_util::api::Method;
use request::{DropEntitlementRequest, UpdateEntitlementsRequest};
use response::{DropsEntitlementsResponse, UpdateDropEntitlementsResponse};

use crate::{
    base::TwitchAPIBase,
    request::{EmptyBody, EndpointType, TwitchAPIRequest},
    types::PaginationQuery,
    TwitchAPI,
};

pub mod request;
pub mod response;
pub mod types;

#[cfg_attr(docsrs, doc(cfg(feature = "entitlements")))]
pub trait EntitlementsAPI: TwitchAPIBase {
    /// <https://dev.twitch.tv/docs/api/reference/#get-drops-entitlements>
    fn get_drops_entitlements(
        &self,
        opts: Option<DropEntitlementRequest>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, DropsEntitlementsResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#update-drops-entitlements>
    fn update_drops_entitlements(
        &self,
        opts: Option<UpdateEntitlementsRequest>,
    ) -> TwitchAPIRequest<UpdateEntitlementsRequest, UpdateDropEntitlementsResponse>;
}
impl EntitlementsAPI for TwitchAPI {
    fn get_drops_entitlements(
        &self,
        opts: Option<DropEntitlementRequest>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, DropsEntitlementsResponse> {
        let mut url = self.build_url();
        url.path(["entitlements", "drops"])
            .query_opt_pairs(opts)
            .query_opt_pairs(pagination);

        TwitchAPIRequest::new(
            EndpointType::GetDropsEntitlements,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn update_drops_entitlements(
        &self,
        opts: Option<UpdateEntitlementsRequest>,
    ) -> TwitchAPIRequest<UpdateEntitlementsRequest, UpdateDropEntitlementsResponse> {
        let mut url = self.build_url();
        url.path(["entitlements", "drops"]);

        let mut headers = self.build_headers();
        headers.json();

        TwitchAPIRequest::new(
            EndpointType::UpdateDropsEntitlements,
            url.build(),
            Method::PATCH,
            headers.build(),
            opts.unwrap_or_default(),
        )
    }
}
