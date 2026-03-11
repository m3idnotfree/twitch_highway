use crate::{
    subscriptions::BroadcasterSubscriptionResponse,
    types::{
        constants::{AFTER, BEFORE, BROADCASTER_ID, FIRST, SUBSCRIPTIONS, USER_ID},
        BroadcasterId, UserId,
    },
    Client, Error,
};

#[derive(Debug)]
pub struct GetBroadcasterSubscriptionsBuilder<'a> {
    client: &'a Client,
    broadcaster_id: &'a BroadcasterId,
    user_ids: Option<&'a [UserId]>,
    first: Option<u8>,
    after: Option<&'a str>,
    before: Option<&'a str>,
}

impl<'a> GetBroadcasterSubscriptionsBuilder<'a> {
    pub fn new(client: &'a Client, broadcaster_id: &'a BroadcasterId) -> Self {
        Self {
            client,
            broadcaster_id,
            user_ids: None,
            first: None,
            after: None,
            before: None,
        }
    }

    pub fn user_ids(mut self, value: &'a [UserId]) -> Self {
        self.user_ids = Some(value);
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

    pub fn before(mut self, value: &'a str) -> Self {
        self.before = Some(value);
        self
    }

    pub async fn send(self) -> Result<BroadcasterSubscriptionResponse, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut().unwrap().push(SUBSCRIPTIONS);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, self.broadcaster_id);

        if let Some(ids) = self.user_ids {
            url.query_pairs_mut()
                .extend_pairs(ids.iter().map(|id| (USER_ID, id)));
        }
        if let Some(val) = self.first {
            url.query_pairs_mut().append_pair(FIRST, &val.to_string());
        }
        if let Some(val) = self.after {
            url.query_pairs_mut().append_pair(AFTER, val);
        }
        if let Some(val) = self.before {
            url.query_pairs_mut().append_pair(BEFORE, val);
        }

        let req = self.client.http_client().get(url);
        self.client.json(req).await
    }
}
