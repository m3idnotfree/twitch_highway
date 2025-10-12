use serde::Serialize;

use crate::{
    polls::PollsResponse,
    request::TwitchAPIRequest,
    types::constants::{AFTER, BROADCASTER_ID, FIRST, ID, POLLS},
    types::{BroadcasterId, PollId, Title},
    TwitchAPI,
};

define_request_builder! {
    #[derive(Debug)]
    GetPollsBuilder<'a> {
        req: {broadcaster_id: &'a BroadcasterId [key = BROADCASTER_ID]},
        opts: {
            ids: &'a [PollId] [key = ID, convert = extend],
            first: u8 [key = FIRST, convert = to_string],
            after: &'a str [key = AFTER]
        }
    } -> PollsResponse;
    endpoint_type: GetPolls,
    method: GET,
    path: [POLLS],
}

#[derive(Debug, Serialize)]
pub struct CreatePollBuilder<'a> {
    #[serde(skip)]
    api: &'a TwitchAPI,
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

impl<'a> CreatePollBuilder<'a> {
    pub fn new(
        api: &'a TwitchAPI,
        broadcaster_id: &'a BroadcasterId,
        title: &'a str,
        choices: &'a [Title],
        duration: u16,
    ) -> Self {
        Self {
            api,
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

    pub fn build(self) -> TwitchAPIRequest<PollsResponse> {
        let mut url = self.api.build_url();

        url.path_segments_mut().unwrap().extend(&[POLLS]);

        let body = serde_json::to_string(&self).ok();

        TwitchAPIRequest::new(
            crate::request::EndpointType::CreatePoll,
            url,
            reqwest::Method::POST,
            self.api.header_json(),
            body,
            self.api.client.clone(),
        )
    }

    pub async fn send(self) -> Result<reqwest::Response, crate::Error> {
        self.build().send().await
    }

    pub async fn json(self) -> Result<PollsResponse, crate::Error> {
        self.build().json().await
    }
}
