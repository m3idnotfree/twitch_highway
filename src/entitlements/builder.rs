use serde::Serialize;

use crate::{
    Client, Error,
    entitlements::{DropsEntitlementsResponse, FulfillmentStatus, UpdateDropEntitlementsResponse},
    types::{
        EntitlementId, GameId, UserId,
        constants::{AFTER, DROPS, ENTITLEMENTS, FIRST, GAME_ID, ID, USER_ID},
    },
};

#[derive(Debug)]
pub struct GetDropsEntitlements<'a> {
    client: &'a Client,
    ids: Option<&'a [EntitlementId]>,
    user_id: Option<&'a UserId>,
    game_id: Option<&'a GameId>,
    fulfillment_status: Option<FulfillmentStatus>,
    first: Option<u8>,
    after: Option<&'a str>,
}

impl<'a> GetDropsEntitlements<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self {
            client,
            ids: None,
            user_id: None,
            game_id: None,
            fulfillment_status: None,
            first: None,
            after: None,
        }
    }

    pub fn ids(mut self, value: &'a [EntitlementId]) -> Self {
        self.ids = Some(value);
        self
    }

    pub fn user_id(mut self, value: &'a UserId) -> Self {
        self.user_id = Some(value);
        self
    }

    pub fn game_id(mut self, value: &'a GameId) -> Self {
        self.game_id = Some(value);
        self
    }

    pub fn fulfillment_status(mut self, value: FulfillmentStatus) -> Self {
        self.fulfillment_status = Some(value);
        self
    }

    pub fn first(mut self, value: u8) -> Self {
        self.first = Some(value);
        self
    }

    pub fn after(mut self, value: &'a str) -> Self {
        self.after = Some(value);
        self
    }

    pub async fn send(self) -> Result<DropsEntitlementsResponse, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([ENTITLEMENTS, DROPS]);

        if let Some(ids) = self.ids {
            url.query_pairs_mut()
                .extend_pairs(ids.iter().map(|id| (ID, id)));
        }
        if let Some(val) = self.user_id {
            url.query_pairs_mut().append_pair(USER_ID, val);
        }
        if let Some(val) = self.game_id {
            url.query_pairs_mut().append_pair(GAME_ID, val);
        }
        if let Some(val) = self.fulfillment_status {
            url.query_pairs_mut()
                .append_pair("fulfillment_status", val.as_ref());
        }
        if let Some(val) = self.first {
            url.query_pairs_mut().append_pair(FIRST, &val.to_string());
        }
        if let Some(val) = self.after {
            url.query_pairs_mut().append_pair(AFTER, val);
        }

        let req = self.client.http_client().get(url);
        self.client.json(req).await
    }
}

#[derive(Debug, Serialize)]
pub struct UpdateDropsEntitlements<'a> {
    #[serde(skip)]
    client: &'a Client,
    entitlement_ids: Option<&'a [EntitlementId]>,
    fulfillment_status: Option<FulfillmentStatus>,
}

impl<'a> UpdateDropsEntitlements<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self {
            client,
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

    pub async fn send(self) -> Result<UpdateDropEntitlementsResponse, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([ENTITLEMENTS, DROPS]);

        let req = self.client.http_client().patch(url).json(&self);
        self.client.json(req).await
    }
}
