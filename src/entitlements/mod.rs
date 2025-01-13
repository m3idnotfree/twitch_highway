use crate::{base::TwitchAPIBase, EmptyBody, EndpointType, TwitchAPI, TwitchAPIRequest};
use asknothingx2_util::api::Method;
use request::{DropEntitlementRequest, UpdateEntitlementsRequest};

pub mod request;
pub mod response;
pub mod types;

pub trait EntitlementsAPI: TwitchAPIBase {
    fn get_drops_entitlements(
        &self,
        request: DropEntitlementRequest,
    ) -> TwitchAPIRequest<EmptyBody>;
    fn update_drops_entitlements(
        &self,
        request: UpdateEntitlementsRequest,
    ) -> TwitchAPIRequest<UpdateEntitlementsRequest>;
}
impl EntitlementsAPI for TwitchAPI {
    fn get_drops_entitlements(
        &self,
        request: DropEntitlementRequest,
    ) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path(["entitlements", "drops"]).query_pairs(request);

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
        request: UpdateEntitlementsRequest,
    ) -> TwitchAPIRequest<UpdateEntitlementsRequest> {
        let mut url = self.build_url();
        url.path(["entitlements", "drops"]);

        let mut headers = self.build_headers();
        headers.json();

        TwitchAPIRequest::new(
            EndpointType::UpdateDropsEntitlements,
            url.build(),
            Method::PATCH,
            headers.build(),
            request,
        )
    }
}
