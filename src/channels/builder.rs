use serde::Serialize;

use crate::{
    channels::{ChannelFollowersResponse, ContentClassificationLabel, FollowerdChannelsResponse},
    request::{NoContent, TwitchAPIRequest},
    types::{
        constants::{AFTER, BROADCASTER_ID, CHANNELS, FIRST, FOLLOWED, FOLLOWERS, USER_ID},
        BroadcasterId, GameId, UserId,
    },
    TwitchAPI,
};

#[derive(Debug, Serialize)]
pub struct ModifyChannelInfoBuilder<'a> {
    #[serde(skip)]
    api: &'a TwitchAPI,
    #[serde(skip)]
    broadcaster_id: &'a BroadcasterId,

    #[serde(skip_serializing_if = "Option::is_none")]
    broadcaster_language: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    game_id: Option<&'a GameId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delay: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_classification_labels: Option<&'a [ContentClassificationLabel]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_branded_content: Option<bool>,
}

impl<'a> ModifyChannelInfoBuilder<'a> {
    pub fn new(api: &'a TwitchAPI, broadcaster_id: &'a BroadcasterId) -> Self {
        Self {
            api,
            broadcaster_id,
            broadcaster_language: None,
            title: None,
            game_id: None,
            delay: None,
            tags: None,
            content_classification_labels: None,
            is_branded_content: None,
        }
    }
    pub fn broadcaster_language(mut self, value: &'a str) -> Self {
        self.broadcaster_language = Some(value);
        self
    }
    pub fn title(mut self, value: &'a str) -> Self {
        self.title = Some(value);
        self
    }
    pub fn game_id(mut self, value: &'a GameId) -> Self {
        self.game_id = Some(value);
        self
    }
    pub fn delay(mut self, value: u64) -> Self {
        self.delay = Some(value);
        self
    }
    pub fn tags(mut self, value: &'a [&'a str]) -> Self {
        self.tags = Some(value);
        self
    }
    pub fn content_classification_labels(
        mut self,
        value: &'a [ContentClassificationLabel],
    ) -> Self {
        self.content_classification_labels = Some(value);
        self
    }
    pub fn is_branded_content(mut self, value: bool) -> Self {
        self.is_branded_content = Some(value);
        self
    }

    pub fn build(self) -> TwitchAPIRequest<NoContent> {
        let mut url = self.api.build_url();

        url.path_segments_mut().unwrap().extend(&[CHANNELS]);

        let mut query = url.query_pairs_mut();

        query.append_pair(BROADCASTER_ID, self.broadcaster_id);

        drop(query);

        let body = serde_json::to_string(&self).ok();

        TwitchAPIRequest::new(
            crate::request::EndpointType::ModifyChannelInformation,
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

    pub async fn json(self) -> Result<NoContent, crate::Error> {
        self.build().json().await
    }
}

define_request_builder! {
    #[derive(Debug)]
    GetFollowedChannels<'a> {
        req: { user_id: &'a UserId [key = USER_ID] },
        opts: {
            broadcaster_id: &'a BroadcasterId [key = BROADCASTER_ID],
            first: u8 [key = FIRST, convert = to_string],
            after: &'a str [key = AFTER]
        }
    } -> FollowerdChannelsResponse;
    endpoint_type: GetFollowedChannels,
    method: GET,
    path: [CHANNELS, FOLLOWED],
}

define_request_builder! {
    #[derive(Debug)]
    GetChannelFollowersRequest<'a> {
        req: { broadcaster_id: &'a BroadcasterId [key = BROADCASTER_ID]},
        opts: {
            user_id: &'a UserId [key = USER_ID],
            first: u8 [key = FIRST, convert = to_string],
            after: &'a str [key = AFTER]
        }
    } -> ChannelFollowersResponse;
    endpoint_type: GetChannelFollowers,
    method: GET,
    path: [CHANNELS, FOLLOWERS],
}
