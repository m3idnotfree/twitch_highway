use serde::Serialize;

use crate::{
    entitlements::{DropsEntitlementsResponse, FulfillmentStatus, UpdateDropEntitlementsResponse},
    request::TwitchAPIRequest,
    types::{
        constants::{AFTER, DROPS, ENTITLEMENTS, FIRST, GAME_ID, ID, USER_ID},
        EntitlementId, GameId, UserId,
    },
    TwitchAPI,
};

define_request_builder! {
    #[derive(Debug)]
    GetDropsEntitlementsBuilder<'a> {
        ids: &'a [EntitlementId] [key = ID, convert = extend],
        user_id: &'a UserId [key = USER_ID],
        game_id: &'a GameId [key = GAME_ID],
        fulfillment_status: FulfillmentStatus [convert = as_ref],
        first: u8 [key = FIRST, convert = to_string],
        after: &'a str [key = AFTER]
    } -> DropsEntitlementsResponse;
            endpoint_type: GetDropsEntitlements,
            method: GET,
            path: [ENTITLEMENTS, DROPS],
}

#[derive(Debug, Serialize)]
pub struct UpdateDropsEntitlementsBuilder<'a> {
    #[serde(skip)]
    api: &'a TwitchAPI,
    entitlement_ids: Option<&'a [EntitlementId]>,
    fulfillment_status: Option<FulfillmentStatus>,
}

impl<'a> UpdateDropsEntitlementsBuilder<'a> {
    pub fn new(api: &'a TwitchAPI) -> Self {
        Self {
            api,
            entitlement_ids: None,
            fulfillment_status: None,
        }
    }
    pub fn entitlement_ids(mut self, value: &'a [EntitlementId]) -> Self {
        self.entitlement_ids = Some(value);
        self
    }

    pub fn fulfillment_status(mut self, value: FulfillmentStatus) -> Self {
        self.fulfillment_status = Some(value);
        self
    }

    pub fn build(self) -> TwitchAPIRequest<UpdateDropEntitlementsResponse> {
        let mut url = self.api.build_url();

        url.path_segments_mut()
            .unwrap()
            .extend(&[ENTITLEMENTS, DROPS]);

        let body = serde_json::to_string(&self).ok();

        TwitchAPIRequest::new(
            crate::request::EndpointType::UpdateDropsEntitlements,
            url,
            reqwest::Method::PATCH,
            self.api.header_json(),
            body,
            self.api.client.clone(),
        )
    }

    pub async fn send(self) -> Result<reqwest::Response, crate::Error> {
        self.build().send().await
    }

    pub async fn json(self) -> Result<UpdateDropEntitlementsResponse, crate::Error> {
        self.build().json().await
    }
}
