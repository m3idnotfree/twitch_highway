use serde::Serialize;

use crate::{
    Client, Error,
    chat::{
        AnnouncementColor, ChatSettingResponse, ChattersResponse, EmotesResponse,
        SendChatMessageResponse,
    },
    types::{
        BroadcasterId, ModeratorId, UserId,
        constants::{
            AFTER, ANNOUNCEMENTS, BROADCASTER_ID, CHAT, CHATTERS, EMOTES, FIRST, MESSAGES,
            MODERATOR_ID, SETTINGS, USER, USER_ID,
        },
    },
};

#[derive(Debug)]
pub struct GetChatters<'a> {
    client: &'a Client,
    broadcaster_id: &'a BroadcasterId,
    moderator_id: &'a ModeratorId,
    first: Option<u8>,
    after: Option<&'a str>,
}

impl<'a> GetChatters<'a> {
    pub fn new(
        client: &'a Client,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
    ) -> Self {
        Self {
            client,
            broadcaster_id,
            moderator_id,
            first: None,
            after: None,
        }
    }

    pub fn first(mut self, value: u8) -> Self {
        self.first = Some(value);
        self
    }

    pub fn after(mut self, value: &'a str) -> Self {
        self.after = Some(value);
        self
    }
    pub async fn send(self) -> Result<ChattersResponse, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut().unwrap().extend([CHAT, CHATTERS]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, self.broadcaster_id)
            .append_pair(MODERATOR_ID, self.moderator_id);
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
pub struct GetChatSettings<'a> {
    client: &'a Client,
    broadcaster_id: &'a BroadcasterId,
    moderator_id: Option<&'a ModeratorId>,
}

impl<'a> GetChatSettings<'a> {
    pub fn new(client: &'a Client, broadcaster_id: &'a BroadcasterId) -> Self {
        Self {
            client,
            broadcaster_id,
            moderator_id: None,
        }
    }

    pub fn moderator_id(mut self, value: &'a ModeratorId) -> Self {
        self.moderator_id = Some(value);
        self
    }

    pub async fn send(self) -> Result<ChatSettingResponse, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut().unwrap().extend([CHAT, SETTINGS]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, self.broadcaster_id);
        if let Some(val) = self.moderator_id {
            url.query_pairs_mut().append_pair(MODERATOR_ID, val);
        }

        let req = self.client.http_client().get(url);
        self.client.json(req).await
    }
}

#[derive(Debug)]
pub struct GetUserEmotes<'a> {
    client: &'a Client,
    user_id: &'a UserId,
    broadcaster_id: Option<&'a BroadcasterId>,
    after: Option<&'a str>,
}

impl<'a> GetUserEmotes<'a> {
    pub fn new(client: &'a Client, user_id: &'a UserId) -> Self {
        Self {
            client,
            user_id,
            broadcaster_id: None,
            after: None,
        }
    }

    pub fn broadcaster_id(mut self, value: &'a BroadcasterId) -> Self {
        self.broadcaster_id = Some(value);
        self
    }

    pub fn after(mut self, value: &'a str) -> Self {
        self.after = Some(value);
        self
    }

    pub async fn send(self) -> Result<EmotesResponse, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([CHAT, EMOTES, USER]);

        url.query_pairs_mut().append_pair(USER_ID, self.user_id);
        if let Some(val) = self.broadcaster_id {
            url.query_pairs_mut().append_pair(BROADCASTER_ID, val);
        }
        if let Some(val) = self.after {
            url.query_pairs_mut().append_pair(AFTER, val);
        }

        let req = self.client.http_client().get(url);
        self.client.json(req).await
    }
}

#[derive(Debug, Serialize)]
pub struct UpdateChatSettings<'a> {
    #[serde(skip)]
    client: &'a Client,
    #[serde(skip)]
    broadcaster_id: &'a BroadcasterId,
    #[serde(skip)]
    moderator_id: &'a ModeratorId,

    #[serde(skip_serializing_if = "Option::is_none")]
    emote_mode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    follower_mode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    follower_mode_duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    non_moderator_chat_delay: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    non_moderator_chat_delay_duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slow_mode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slow_mode_wait_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscriber_mode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unique_chat_mode: Option<bool>,
}

impl<'a> UpdateChatSettings<'a> {
    pub fn new(
        client: &'a Client,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
    ) -> Self {
        Self {
            client,
            broadcaster_id,
            moderator_id,
            emote_mode: None,
            follower_mode: None,
            follower_mode_duration: None,
            non_moderator_chat_delay: None,
            non_moderator_chat_delay_duration: None,
            slow_mode: None,
            slow_mode_wait_time: None,
            subscriber_mode: None,
            unique_chat_mode: None,
        }
    }

    pub fn emote_mode(mut self, value: bool) -> Self {
        self.emote_mode = Some(value);
        self
    }

    pub fn follower_mode(mut self, value: bool) -> Self {
        self.follower_mode = Some(value);
        self
    }

    pub fn follower_mode_duration(mut self, value: u64) -> Self {
        self.follower_mode_duration = Some(value);
        self
    }

    pub fn non_moderator_chat_delay(mut self, value: bool) -> Self {
        self.non_moderator_chat_delay = Some(value);
        self
    }

    pub fn non_moderator_chat_delay_duration(mut self, value: u64) -> Self {
        self.non_moderator_chat_delay_duration = Some(value);
        self
    }

    pub fn slow_mode(mut self, value: bool) -> Self {
        self.slow_mode = Some(value);
        self
    }

    pub fn slow_mode_wait_time(mut self, value: u64) -> Self {
        self.slow_mode_wait_time = Some(value);
        self
    }

    pub fn subscriber_mode(mut self, value: bool) -> Self {
        self.subscriber_mode = Some(value);
        self
    }

    pub fn unique_chat_mode(mut self, value: bool) -> Self {
        self.unique_chat_mode = Some(value);
        self
    }

    pub async fn send(self) -> Result<ChatSettingResponse, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut().unwrap().extend([CHAT, SETTINGS]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, self.broadcaster_id)
            .append_pair(MODERATOR_ID, self.moderator_id);

        let req = self.client.http_client().patch(url).json(&self);
        self.client.json(req).await
    }
}

#[derive(Debug, Serialize)]
pub struct SendChatAnnouncement<'a> {
    #[serde(skip)]
    client: &'a Client,
    #[serde(skip)]
    broadcaster_id: &'a BroadcasterId,
    #[serde(skip)]
    moderator_id: &'a ModeratorId,

    message: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<AnnouncementColor>,
}

impl<'a> SendChatAnnouncement<'a> {
    pub fn new(
        client: &'a Client,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
        message: &'a str,
    ) -> Self {
        Self {
            client,
            broadcaster_id,
            moderator_id,
            message,
            color: None,
        }
    }

    pub fn color(mut self, value: AnnouncementColor) -> Self {
        self.color = Some(value);
        self
    }

    pub async fn send(self) -> Result<(), Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([CHAT, ANNOUNCEMENTS]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, self.broadcaster_id)
            .append_pair(MODERATOR_ID, self.moderator_id);

        let req = self.client.http_client().post(url).json(&self);
        self.client.no_content(req).await
    }
}

#[derive(Debug, Serialize)]
pub struct SendChatMessage<'a> {
    #[serde(skip)]
    client: &'a Client,
    broadcaster_id: &'a BroadcasterId,
    sender_id: &'a UserId,
    message: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_source_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_parent_message_id: Option<&'a str>,
}

impl<'a> SendChatMessage<'a> {
    pub fn new(
        client: &'a Client,
        broadcaster_id: &'a BroadcasterId,
        sender_id: &'a UserId,
        message: &'a str,
    ) -> Self {
        Self {
            client,
            broadcaster_id,
            sender_id,
            message,
            for_source_only: None,
            reply_parent_message_id: None,
        }
    }

    pub fn for_source_only(mut self, value: bool) -> Self {
        self.for_source_only = Some(value);
        self
    }

    pub fn reply_parent_message_id(mut self, value: &'a str) -> Self {
        self.reply_parent_message_id = Some(value);
        self
    }

    pub async fn send(self) -> Result<SendChatMessageResponse, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut().unwrap().extend([CHAT, MESSAGES]);

        let req = self.client.http_client().post(url).json(&self);
        self.client.json(req).await
    }
}
