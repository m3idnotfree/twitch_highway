use serde::Serialize;

use crate::{
    Client, Error,
    channels::{ChannelFollowersResponse, ContentClassificationLabel, FollowerdChannelsResponse},
    types::{
        BroadcasterId, GameId, UserId,
        constants::{AFTER, BROADCASTER_ID, CHANNELS, FIRST, FOLLOWED, FOLLOWERS, USER_ID},
    },
};

#[derive(Debug, Serialize)]
pub struct ModifyChannelInfo<'a> {
    #[serde(skip)]
    client: &'a Client,
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

impl<'a> ModifyChannelInfo<'a> {
    pub fn new(client: &'a Client, broadcaster_id: &'a BroadcasterId) -> Self {
        Self {
            client,
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

    pub async fn send(self) -> Result<(), Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut().unwrap().push(CHANNELS);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, self.broadcaster_id);

        let req = self.client.http_client().patch(url).json(&self);
        self.client.no_content(req).await
    }
}

#[derive(Debug)]
pub struct GetFollowedChannels<'a> {
    client: &'a Client,
    user_id: &'a UserId,
    broadcaster_id: Option<&'a BroadcasterId>,
    first: Option<u8>,
    after: Option<&'a str>,
}

impl<'a> GetFollowedChannels<'a> {
    pub fn new(client: &'a Client, user_id: &'a UserId) -> Self {
        Self {
            client,
            user_id,
            broadcaster_id: None,
            first: None,
            after: None,
        }
    }

    pub fn broadcaster_id(mut self, value: &'a BroadcasterId) -> Self {
        self.broadcaster_id = Some(value);
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

    pub async fn send(self) -> Result<FollowerdChannelsResponse, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([CHANNELS, FOLLOWED]);

        url.query_pairs_mut().append_pair(USER_ID, self.user_id);
        if let Some(val) = self.broadcaster_id {
            url.query_pairs_mut().append_pair(BROADCASTER_ID, val);
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
#[derive(Debug)]
pub struct GetChannelFollowers<'a> {
    client: &'a Client,
    broadcaster_id: &'a BroadcasterId,
    user_id: Option<&'a UserId>,
    first: Option<u8>,
    after: Option<&'a str>,
}

impl<'a> GetChannelFollowers<'a> {
    pub fn new(client: &'a Client, broadcaster_id: &'a BroadcasterId) -> Self {
        Self {
            client,
            broadcaster_id,
            user_id: None,
            first: None,
            after: None,
        }
    }

    pub fn user_id(mut self, value: &'a UserId) -> Self {
        self.user_id = Some(value);
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

    pub async fn send(self) -> Result<ChannelFollowersResponse, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([CHANNELS, FOLLOWERS]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, self.broadcaster_id);
        if let Some(val) = self.user_id {
            url.query_pairs_mut().append_pair(USER_ID, val);
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
