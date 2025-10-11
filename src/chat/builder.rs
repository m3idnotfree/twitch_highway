use serde::Serialize;

use crate::{
    chat::{
        AnnouncementColor, ChatSettingResponse, ChattersResponse, EmotesResponse,
        SendChatMessageResponse,
    },
    request::{NoContent, TwitchAPIRequest},
    types::{
        constants::{
            AFTER, ANNOUNCEMENTS, BROADCASTER_ID, CHAT, CHATTERS, EMOTES, FIRST, MESSAGES,
            MODERATOR_ID, SETTINGS, USER, USER_ID,
        },
        BroadcasterId, ModeratorId, UserId,
    },
    TwitchAPI,
};

define_request_builder! {
    #[derive(Debug)]
    GetChattersBuilder<'a> {
        req: {
            broadcaster_id: &'a BroadcasterId [key = BROADCASTER_ID],
            moderator_id: &'a ModeratorId [key = MODERATOR_ID]
        },
        opts: {
            first: u8 [key = FIRST, convert = to_string],
            after: &'a str [key = AFTER]
        }
    } -> ChattersResponse;
    endpoint_type: GetChatters,
    method: GET,
    path: [CHAT, CHATTERS],
}

define_request_builder! {
    #[derive(Debug)]
    GetChatSettingsBuilder<'a> {
        req: {broadcaster_id: &'a BroadcasterId [key = BROADCASTER_ID]},
        opts: {moderator_id: &'a ModeratorId [key = MODERATOR_ID]}
    } -> ChatSettingResponse;
    endpoint_type: GetChatSettings,
    method: GET,
    path: [CHAT, SETTINGS],

}

define_request_builder! {
    #[derive(Debug)]
    GetUserEmotesBuilder<'a> {
        req: {user_id: &'a UserId [key = USER_ID]},
        opts: {
            broadcaster_id: &'a BroadcasterId [key = BROADCASTER_ID],
            after: &'a str [key = AFTER],
        }
    } -> EmotesResponse;
    endpoint_type: GetUserEmotes,
    method: GET,
    path: [CHAT, EMOTES, USER],
}

#[derive(Debug, Serialize)]
pub struct UpdateChatSettingsBuilder<'a> {
    #[serde(skip)]
    api: &'a TwitchAPI,
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

impl<'a> UpdateChatSettingsBuilder<'a> {
    pub fn new(
        api: &'a TwitchAPI,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
    ) -> Self {
        Self {
            api,
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

    pub fn build(self) -> TwitchAPIRequest<ChatSettingResponse> {
        let mut url = self.api.build_url();

        url.path_segments_mut().unwrap().extend(&[CHAT, SETTINGS]);

        let mut query = url.query_pairs_mut();

        query.append_pair(BROADCASTER_ID, self.broadcaster_id);
        query.append_pair(MODERATOR_ID, self.moderator_id);

        drop(query);

        let body = serde_json::to_string(&self).ok();

        TwitchAPIRequest::new(
            crate::request::EndpointType::UpdateChatSettings,
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

    pub async fn json(self) -> Result<ChatSettingResponse, crate::Error> {
        self.build().json().await
    }
}

#[derive(Debug, Serialize)]
pub struct SendChatAnnouncementBuilder<'a> {
    #[serde(skip)]
    api: &'a TwitchAPI,
    #[serde(skip)]
    broadcaster_id: &'a BroadcasterId,
    #[serde(skip)]
    moderator_id: &'a ModeratorId,

    message: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<AnnouncementColor>,
}

impl<'a> SendChatAnnouncementBuilder<'a> {
    pub fn new(
        api: &'a TwitchAPI,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
        message: &'a str,
    ) -> Self {
        Self {
            api,
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

    pub fn build(self) -> TwitchAPIRequest<NoContent> {
        let mut url = self.api.build_url();

        url.path_segments_mut()
            .unwrap()
            .extend(&[CHAT, ANNOUNCEMENTS]);

        let mut query = url.query_pairs_mut();

        query.append_pair(BROADCASTER_ID, self.broadcaster_id);
        query.append_pair(MODERATOR_ID, self.moderator_id);

        drop(query);

        let body = serde_json::to_string(&self).ok();

        TwitchAPIRequest::new(
            crate::request::EndpointType::SendChatAnnouncement,
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

    pub async fn json(self) -> Result<NoContent, crate::Error> {
        self.build().json().await
    }
}

#[derive(Debug, Serialize)]
pub struct SendChatMessageBuilder<'a> {
    #[serde(skip)]
    api: &'a TwitchAPI,

    broadcaster_id: &'a BroadcasterId,
    sender_id: &'a UserId,
    message: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_source_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_parent_message_id: Option<&'a str>,
}

impl<'a> SendChatMessageBuilder<'a> {
    pub fn new(
        api: &'a TwitchAPI,

        broadcaster_id: &'a BroadcasterId,
        sender_id: &'a UserId,
        message: &'a str,
    ) -> Self {
        Self {
            api,
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
    pub fn build(self) -> TwitchAPIRequest<SendChatMessageResponse> {
        let mut url = self.api.build_url();

        url.path_segments_mut().unwrap().extend(&[CHAT, MESSAGES]);

        let body = serde_json::to_string(&self).ok();

        TwitchAPIRequest::new(
            crate::request::EndpointType::SendChatMessage,
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

    pub async fn json(self) -> Result<SendChatMessageResponse, crate::Error> {
        self.build().json().await
    }
}
