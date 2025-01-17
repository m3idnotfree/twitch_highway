use asknothingx2_util::api::Method;
use request::{ChatColor, SendChatMessageRequest, UpdateChatSettingsRequest};
use response::{
    BadgesResponse, ChatSettingResponse, ChattersResponse, EmotesResponse, SendChatMessageResponse,
    SharedChatSessionResponse, UsersColorResponse,
};

use crate::{
    base::{TwitchAPI, TwitchAPIBase},
    types::{
        constants::{AFTER, BROADCASTER_ID, CHAT, EMOTES, MODERATOR_ID, SETTINGS, USER_ID},
        BroadcasterId, ModeratorId, PaginationQuery, UserId,
    },
    EmptyBody, EndpointType, TwitchAPIRequest,
};

pub mod request;
pub mod response;
pub mod types;

pub trait ChatAPI: TwitchAPIBase {
    /// https://dev.twitch.tv/docs/api/reference/#get-chatters
    /// Authorization
    /// Requires a user access token that includes the moderator:read:chatters scope.
    fn get_chatters(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, ChattersResponse>;
    /// https://dev.twitch.tv/docs/api/reference/#get-channel-emotes
    fn channel_emotes(
        &self,
        broadcaster_id: BroadcasterId,
    ) -> TwitchAPIRequest<EmptyBody, EmotesResponse>;
    /// https://dev.twitch.tv/docs/api/reference/#get-global-emotes
    fn global_emotes(&self) -> TwitchAPIRequest<EmptyBody, EmotesResponse>;
    /// https://dev.twitch.tv/docs/api/reference/#get-emote-sets
    fn emote_sets(&self, emote_set_ids: &[&str]) -> TwitchAPIRequest<EmptyBody, EmotesResponse>;
    /// https://dev.twitch.tv/docs/api/reference/#get-channel-chat-badges
    fn channel_badge(
        &self,
        broadcaster_id: BroadcasterId,
    ) -> TwitchAPIRequest<EmptyBody, BadgesResponse>;
    /// https://dev.twitch.tv/docs/api/reference/#get-global-chat-badges
    fn global_badge(&self) -> TwitchAPIRequest<EmptyBody, BadgesResponse>;
    /// https://dev.twitch.tv/docs/api/reference/#get-chat-settings
    fn get_chat_settings(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: Option<ModeratorId>,
    ) -> TwitchAPIRequest<EmptyBody, ChatSettingResponse>;
    /// https://dev.twitch.tv/docs/api/reference/#get-shared-chat-session
    fn get_shared_chat_session(
        &self,
        broadcaster_id: BroadcasterId,
    ) -> TwitchAPIRequest<EmptyBody, SharedChatSessionResponse>;
    /// https://dev.twitch.tv/docs/api/reference/#get-user-emotes
    fn user_emotes(
        &self,
        user_id: UserId,
        after: Option<&str>,
        broadcaster_id: Option<BroadcasterId>,
    ) -> TwitchAPIRequest<EmptyBody, EmotesResponse>;
    /// https://dev.twitch.tv/docs/api/reference/#update-chat-settings
    fn update_chat_settings(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        opts: Option<UpdateChatSettingsRequest>,
    ) -> TwitchAPIRequest<UpdateChatSettingsRequest, ChatSettingResponse>;
    /// https://dev.twitch.tv/docs/api/reference/#send-chat-announcement
    fn send_chat_announcement(&self) {
        unimplemented!()
    }
    /// https://dev.twitch.tv/docs/api/reference/#send-a-shoutout
    fn send_a_shoutout(&self) {
        unimplemented!()
    }
    /// https://dev.twitch.tv/docs/api/reference/#send-chat-message
    /// Requires an app access token or user access token that includes the user:write:chat scope.
    /// If app access token used,
    /// then additionally requires user:bot scope from chatting user,
    /// and either channel:bot scope from broadcaster or moderator status.
    /// The message is limited to a maximum of 500 characters.
    /// Chat messages can also include emoticons.
    /// To include emoticons,
    /// use the name of the emote.
    /// The names are case sensitive.
    /// Don’t include colons around the name (e.g., :bleedPurple:).
    /// If Twitch recognizes the name,
    /// Twitch converts the name to the emote before writing the chat message to the chat room
    fn chat_write(
        &self,
        broadcaster_id: BroadcasterId,
        sender_id: String,
        message: String,
    ) -> TwitchAPIRequest<SendChatMessageRequest, SendChatMessageResponse>;
    /// https://dev.twitch.tv/docs/api/reference/#get-user-chat-color
    fn user_chat_color(
        &self,
        user_id: &[UserId],
    ) -> TwitchAPIRequest<EmptyBody, UsersColorResponse>;
    /// https://dev.twitch.tv/docs/api/reference/#update-user-chat-color
    fn update_user_chat_color(
        &self,
        user_id: UserId,
        color: ChatColor,
    ) -> TwitchAPIRequest<EmptyBody, EmptyBody>;
}

impl ChatAPI for TwitchAPI {
    fn get_chatters(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, ChattersResponse> {
        let mut url = self.build_url();
        url.path([CHAT, "chatters"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(MODERATOR_ID, moderator_id)
            .query_opt_pairs(pagination);

        TwitchAPIRequest::new(
            EndpointType::GetChatters,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn channel_emotes(
        &self,
        broadcaster_id: BroadcasterId,
    ) -> TwitchAPIRequest<EmptyBody, EmotesResponse> {
        let mut url = self.build_url();
        url.path([CHAT, EMOTES])
            .query(BROADCASTER_ID, broadcaster_id);

        TwitchAPIRequest::new(
            EndpointType::GetChannelEmotes,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }

    fn global_emotes(&self) -> TwitchAPIRequest<EmptyBody, EmotesResponse> {
        let mut url = self.build_url();
        url.path([CHAT, EMOTES, "global"]);

        TwitchAPIRequest::new(
            EndpointType::GetGlobalEmotes,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }

    fn emote_sets(&self, emote_set_ids: &[&str]) -> TwitchAPIRequest<EmptyBody, EmotesResponse> {
        let mut url = self.build_url();
        url.path([CHAT, EMOTES, "set"])
            .query_extend(emote_set_ids.iter().map(|x| ("emote_set_id", x)));

        TwitchAPIRequest::new(
            EndpointType::GetEmoteSets,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn channel_badge(
        &self,
        broadcaster_id: BroadcasterId,
    ) -> TwitchAPIRequest<EmptyBody, BadgesResponse> {
        let mut url = self.build_url();
        url.path([CHAT, "badges"])
            .query(BROADCASTER_ID, broadcaster_id);

        TwitchAPIRequest::new(
            EndpointType::GetChannelChatBadges,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn global_badge(&self) -> TwitchAPIRequest<EmptyBody, BadgesResponse> {
        let mut url = self.build_url();
        url.path([CHAT, "badges", "global"]);

        TwitchAPIRequest::new(
            EndpointType::GetGlobalChatBadges,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }

    fn get_chat_settings(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: Option<ModeratorId>,
    ) -> TwitchAPIRequest<EmptyBody, ChatSettingResponse> {
        let mut url = self.build_url();
        url.path([CHAT, SETTINGS])
            .query(BROADCASTER_ID, broadcaster_id)
            .query_opt(MODERATOR_ID, moderator_id);

        TwitchAPIRequest::new(
            EndpointType::GetChatSettings,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }

    fn get_shared_chat_session(
        &self,
        broadcaster_id: BroadcasterId,
    ) -> TwitchAPIRequest<EmptyBody, SharedChatSessionResponse> {
        let mut url = self.build_url();
        url.path(["shared_chat", "session"])
            .query(BROADCASTER_ID, broadcaster_id);

        TwitchAPIRequest::new(
            EndpointType::GetShardChatSession,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }

    fn user_emotes(
        &self,
        user_id: UserId,
        after: Option<&str>,
        broadcaster_id: Option<BroadcasterId>,
    ) -> TwitchAPIRequest<EmptyBody, EmotesResponse> {
        let mut url = self.build_url();
        url.path([CHAT, EMOTES, "user"])
            .query(USER_ID, user_id)
            .query_opt(AFTER, after)
            .query_opt(BROADCASTER_ID, broadcaster_id);

        TwitchAPIRequest::new(
            EndpointType::GetUserEmotes,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }

    fn update_chat_settings(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        configuration: Option<UpdateChatSettingsRequest>,
    ) -> TwitchAPIRequest<UpdateChatSettingsRequest, ChatSettingResponse> {
        let mut url = self.build_url();
        url.path([CHAT, SETTINGS])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(MODERATOR_ID, moderator_id);

        let mut headers = self.build_headers();
        headers.json();

        TwitchAPIRequest::new(
            EndpointType::UpdateChatSettings,
            url.build(),
            Method::PATCH,
            headers.build(),
            configuration.unwrap_or_default(),
        )
    }

    fn chat_write(
        &self,
        broadcaster_id: BroadcasterId,
        sender_id: String,
        message: String,
    ) -> TwitchAPIRequest<SendChatMessageRequest, SendChatMessageResponse> {
        let mut url = self.build_url();
        url.path([CHAT, "messages"]);

        let mut headers = self.build_headers();
        headers.json();

        TwitchAPIRequest::new(
            EndpointType::SendChatMessage,
            url.build(),
            Method::POST,
            headers.build(),
            SendChatMessageRequest::new(broadcaster_id, sender_id, message),
        )
    }

    fn user_chat_color(
        &self,
        user_ids: &[UserId],
    ) -> TwitchAPIRequest<EmptyBody, UsersColorResponse> {
        let mut url = self.build_url();
        url.path([CHAT, "color"])
            .query_extend(user_ids.iter().map(|x| (USER_ID, x)));

        TwitchAPIRequest::new(
            EndpointType::GetUserChatColor,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }

    fn update_user_chat_color(
        &self,
        user_id: UserId,
        color: ChatColor,
    ) -> TwitchAPIRequest<EmptyBody, EmptyBody> {
        let mut url = self.build_url();
        url.path([CHAT, "color"])
            .query(USER_ID, user_id)
            .query("color", color.as_str());

        TwitchAPIRequest::new(
            EndpointType::UpdateUserChatColor,
            url.build(),
            Method::PUT,
            self.build_headers().build(),
            EmptyBody,
        )
    }
}
