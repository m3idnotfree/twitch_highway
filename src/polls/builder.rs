use serde::Serialize;

use crate::{
    polls::PollsResponse,
    types::constants::{AFTER, BROADCASTER_ID, FIRST, ID, POLLS},
    types::{BroadcasterId, PollId, Title},
    Client, Error,
};

#[derive(Debug)]
pub struct GetPolls<'a> {
    client: &'a Client,
    broadcaster_id: &'a BroadcasterId,
    ids: Option<&'a [PollId]>,
    first: Option<u8>,
    after: Option<&'a str>,
}

impl<'a> GetPolls<'a> {
    pub fn new(client: &'a Client, broadcaster_id: &'a BroadcasterId) -> Self {
        Self {
            client,
            broadcaster_id,
            ids: None,
            first: None,
            after: None,
        }
    }

    pub fn ids(mut self, value: &'a [PollId]) -> Self {
        self.ids = Some(value);
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

    pub async fn send(self) -> Result<PollsResponse, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut().unwrap().push(POLLS);
        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, self.broadcaster_id);
        if let Some(ids) = self.ids {
            url.query_pairs_mut()
                .extend_pairs(ids.iter().map(|id| (ID, id)));
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
pub struct CreatePoll<'a> {
    #[serde(skip)]
    client: &'a Client,
    broadcaster_id: &'a BroadcasterId,
    title: &'a str,
    choices: &'a [Title],
    /// min 15 max 1800
    duration: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_points_voting_enabled: Option<bool>,
    /// max 1,000,000
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_points_per_vote: Option<u32>,
}

impl<'a> CreatePoll<'a> {
    pub fn new(
        client: &'a Client,
        broadcaster_id: &'a BroadcasterId,
        title: &'a str,
        choices: &'a [Title],
        duration: u16,
    ) -> Self {
        Self {
            client,
            broadcaster_id,
            title,
            choices,
            duration,
            channel_points_voting_enabled: None,
            channel_points_per_vote: None,
        }
    }

    pub fn channel_points_voting_enabled(mut self, value: bool) -> Self {
        self.channel_points_voting_enabled = Some(value);
        self
    }
    pub fn channel_points_per_vote(mut self, value: u32) -> Self {
        self.channel_points_per_vote = Some(value);
        self
    }

    pub async fn send(self) -> Result<PollsResponse, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut().unwrap().push(POLLS);

        let req = self.client.http_client().post(url).json(&self);
        self.client.json(req).await
    }
}
