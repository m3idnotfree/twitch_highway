mod builder;
mod response;
mod types;

pub use response::{
    BadgesResponse, ChatSettingResponse, ChattersResponse, EmotesResponse, SendChatMessageResponse,
    SharedChatSessionResponse, UsersColorResponse,
};
pub use types::{
    AnnouncementColor, Badge, BroadcasterIdField, ChatColor, ChatSetting, Chatter, DropReason,
    Emote, EmoteType, Format, MessageResponse, Scale, SharedChatSession, ThemeMode, UserColor,
    Version,
};

pub use builder::{
    GetChatSettings, GetChatters, GetUserEmotes, SendChatAnnouncement, SendChatMessage,
    UpdateChatSettings,
};

use std::future::Future;

use crate::{
    types::{
        constants::{
            BADGES, BROADCASTER_ID, CHAT, COLOR, EMOTES, EMOTE_SET_ID, FROM_BROADCASTER_ID, GLOBAL,
            MODERATOR_ID, SESSION, SET, SHARED_CHAT, SHOUTOUTS, TO_BROADCASTER_ID, USER_ID,
        },
        BroadcasterId, ModeratorId, UserId,
    },
    Client, Error,
};

pub trait ChatAPI {
    /// See <https://dev.twitch.tv/docs/api/reference/#get-chatters>
    fn get_chatters<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
    ) -> GetChatters<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#get-channel-emotes>
    fn get_channel_emotes(
        &self,
        broadcaster_id: &BroadcasterId,
    ) -> impl Future<Output = Result<EmotesResponse, Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#get-global-emotes>
    fn get_global_emotes(&self) -> impl Future<Output = Result<EmotesResponse, Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#get-emote-sets>
    fn get_emote_sets(
        &self,
        emote_set_ids: &[&str],
    ) -> impl Future<Output = Result<EmotesResponse, Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#get-channel-chat-badges>
    fn get_channel_chat_badges(
        &self,
        broadcaster_id: &BroadcasterId,
    ) -> impl Future<Output = Result<BadgesResponse, Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#get-global-chat-badges>
    fn get_global_chat_badges(&self) -> impl Future<Output = Result<BadgesResponse, Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#get-chat-settings>
    fn get_chat_settings<'a>(&'a self, broadcaster_id: &'a BroadcasterId) -> GetChatSettings<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#get-shared-chat-session>
    fn get_shared_chat_session(
        &self,
        broadcaster_id: &BroadcasterId,
    ) -> impl Future<Output = Result<SharedChatSessionResponse, Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#get-user-emotes>
    fn get_user_emotes<'a>(&'a self, user_id: &'a UserId) -> GetUserEmotes<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#update-chat-settings>
    fn update_chat_settings<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
    ) -> UpdateChatSettings<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#send-chat-announcement>
    fn send_chat_announcement<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
        message: &'a str,
    ) -> SendChatAnnouncement<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#send-a-shoutout>
    fn send_a_shoutout(
        &self,
        from_broadcaster_id: &BroadcasterId,
        to_broadcaster_id: &BroadcasterId,
        moderator_id: &ModeratorId,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#send-chat-message>
    fn send_chat_message<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        sender_id: &'a UserId,
        message: &'a str,
    ) -> SendChatMessage<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#get-user-chat-color>
    fn get_user_chat_color(
        &self,
        user_ids: &[UserId],
    ) -> impl Future<Output = Result<UsersColorResponse, Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#update-user-chat-color>
    fn update_user_chat_color(
        &self,
        user_id: &UserId,
        color: ChatColor,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

impl ChatAPI for Client {
    fn get_chatters<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
    ) -> GetChatters<'a> {
        GetChatters::new(self, broadcaster_id, moderator_id)
    }

    async fn get_channel_emotes(
        &self,
        broadcaster_id: &BroadcasterId,
    ) -> Result<EmotesResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().extend([CHAT, EMOTES]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id);

        self.json(self.http_client().get(url)).await
    }

    async fn get_global_emotes(&self) -> Result<EmotesResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([CHAT, EMOTES, GLOBAL]);

        self.json(self.http_client().get(url)).await
    }

    async fn get_emote_sets(&self, emote_set_ids: &[&str]) -> Result<EmotesResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().extend([CHAT, EMOTES, SET]);

        url.query_pairs_mut()
            .extend_pairs(emote_set_ids.iter().map(|id| (EMOTE_SET_ID, id)));

        self.json(self.http_client().get(url)).await
    }

    /// <https://dev.twitch.tv/docs/api/reference/#get-channel-chat-badges>
    async fn get_channel_chat_badges(
        &self,
        broadcaster_id: &BroadcasterId,
    ) -> Result<BadgesResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().extend([CHAT, BADGES]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id);

        self.json(self.http_client().get(url)).await
    }

    async fn get_global_chat_badges(&self) -> Result<BadgesResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([CHAT, BADGES, GLOBAL]);

        self.json(self.http_client().get(url)).await
    }

    fn get_chat_settings<'a>(&'a self, broadcaster_id: &'a BroadcasterId) -> GetChatSettings<'a> {
        GetChatSettings::new(self, broadcaster_id)
    }

    async fn get_shared_chat_session(
        &self,
        broadcaster_id: &BroadcasterId,
    ) -> Result<SharedChatSessionResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([SHARED_CHAT, SESSION]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id);

        self.json(self.http_client().get(url)).await
    }

    fn get_user_emotes<'a>(&'a self, user_id: &'a UserId) -> GetUserEmotes<'a> {
        GetUserEmotes::new(self, user_id)
    }

    fn update_chat_settings<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
    ) -> UpdateChatSettings<'a> {
        UpdateChatSettings::new(self, broadcaster_id, moderator_id)
    }

    fn send_chat_announcement<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
        message: &'a str,
    ) -> SendChatAnnouncement<'a> {
        SendChatAnnouncement::new(self, broadcaster_id, moderator_id, message)
    }

    async fn send_a_shoutout(
        &self,
        from_broadcaster_id: &BroadcasterId,
        to_broadcaster_id: &BroadcasterId,
        moderator_id: &ModeratorId,
    ) -> Result<(), Error> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().extend([CHAT, SHOUTOUTS]);

        url.query_pairs_mut()
            .append_pair(FROM_BROADCASTER_ID, from_broadcaster_id)
            .append_pair(TO_BROADCASTER_ID, to_broadcaster_id)
            .append_pair(MODERATOR_ID, moderator_id);

        self.no_content(self.http_client().post(url)).await
    }

    fn send_chat_message<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        sender_id: &'a UserId,
        message: &'a str,
    ) -> SendChatMessage<'a> {
        SendChatMessage::new(self, broadcaster_id, sender_id, message)
    }

    async fn get_user_chat_color(&self, user_ids: &[UserId]) -> Result<UsersColorResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().extend([CHAT, COLOR]);

        url.query_pairs_mut()
            .extend_pairs(user_ids.iter().map(|id| (USER_ID, id)));

        self.json(self.http_client().get(url)).await
    }

    async fn update_user_chat_color(
        &self,
        user_id: &UserId,
        color: ChatColor,
    ) -> Result<(), Error> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().extend([CHAT, COLOR]);

        url.query_pairs_mut()
            .append_pair(USER_ID, user_id)
            .append_pair(COLOR, color.as_ref());

        self.no_content(self.http_client().put(url)).await
    }
}
