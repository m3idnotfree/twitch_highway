use asknothingx2_util::api::Method;
use request::{ChatColor, SendChatMessageRequest, UpdateChatSettingsRequest};

use crate::{
    base::{TwitchAPI, TwitchAPIBase},
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
        broadcaster_id: &str,
        moderator_id: &str,
        first: Option<u64>,
        after: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody>;
    /// https://dev.twitch.tv/docs/api/reference/#get-channel-emotes
    fn channel_emotes(&self, broadcaster_id: &str) -> TwitchAPIRequest<EmptyBody>;
    /// https://dev.twitch.tv/docs/api/reference/#get-global-emotes
    fn global_emotes(&self) -> TwitchAPIRequest<EmptyBody>;
    /// https://dev.twitch.tv/docs/api/reference/#get-emote-sets
    fn emote_sets<'a, T: IntoIterator<Item = &'a str>>(
        &self,
        emote_set_ids: T,
    ) -> TwitchAPIRequest<EmptyBody>;
    fn channel_badge(&self, broadcaster_id: &str) -> TwitchAPIRequest<EmptyBody>;
    fn global_badge(&self) -> TwitchAPIRequest<EmptyBody>;
    /// https://dev.twitch.tv/docs/api/reference/#get-chat-settings
    fn get_chat_settings(
        &self,
        broadcaster_id: &str,
        moderator_id: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody>;
    /// https://dev.twitch.tv/docs/api/reference/#get-shared-chat-session
    fn get_shared_chat_session(&self, broadcaster_id: &str) -> TwitchAPIRequest<EmptyBody>;
    fn user_emotes(
        &self,
        user_id: &str,
        after: Option<&str>,
        broadcaster_id: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody>;
    fn update_chat_settings(
        &self,
        broadcaster_id: &str,
        moderator_id: &str,
        configuration: UpdateChatSettingsRequest,
    ) -> TwitchAPIRequest<UpdateChatSettingsRequest>;
    fn send_chat_announcement(&self) {
        unimplemented!()
    }
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
        broadcaster_id: &str,
        sender_id: &str,
        message: &str,
    ) -> TwitchAPIRequest<SendChatMessageRequest>;
    fn user_chat_color<'a, T: IntoIterator<Item = &'a str>>(
        &self,
        user_id: T,
    ) -> TwitchAPIRequest<EmptyBody>;
    fn update_user_chat_color(
        &self,
        user_id: &str,
        color: ChatColor,
    ) -> TwitchAPIRequest<EmptyBody>;
}

impl ChatAPI for TwitchAPI {
    fn get_chatters(
        &self,
        broadcaster_id: &str,
        moderator_id: &str,
        first: Option<u64>,
        after: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody> {
        TwitchAPIRequest::new(
            EndpointType::GetChatters,
            self.build_url()
                .path(["chat", "chatters"])
                .query([
                    ("broadcaster_id", broadcaster_id),
                    ("moderator_id", moderator_id),
                ])
                .query_option("first", first.map(|x| x.to_string()))
                .query_option("after", after)
                .build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn channel_emotes(&self, broadcaster_id: &str) -> TwitchAPIRequest<EmptyBody> {
        TwitchAPIRequest::new(
            EndpointType::GetChannelEmotes,
            self.build_url()
                .path(["chat", "emotes"])
                .query([("broadcaster_id", broadcaster_id)])
                .build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }

    fn global_emotes(&self) -> TwitchAPIRequest<EmptyBody> {
        TwitchAPIRequest::new(
            EndpointType::GetGlobalEmotes,
            self.build_url().path(["chat", "emotes", "global"]).build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }

    fn emote_sets<'a, T: IntoIterator<Item = &'a str>>(
        &self,
        emote_set_ids: T,
    ) -> TwitchAPIRequest<EmptyBody> {
        TwitchAPIRequest::new(
            EndpointType::GetEmoteSets,
            self.build_url()
                .path(["chat", "emotes", "set"])
                .query(emote_set_ids.into_iter().map(|x| ("emote_set_id", x)))
                .build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn channel_badge(&self, broadcaster_id: &str) -> TwitchAPIRequest<EmptyBody> {
        TwitchAPIRequest::new(
            EndpointType::GetChannelChatBadges,
            self.build_url()
                .path(["chat", "badges"])
                .query([("broadcaster_id", broadcaster_id)])
                .build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn global_badge(&self) -> TwitchAPIRequest<EmptyBody> {
        TwitchAPIRequest::new(
            EndpointType::GetGlobalChatBadges,
            self.build_url().path(["chat", "badges", "global"]).build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }

    fn get_chat_settings(
        &self,
        broadcaster_id: &str,
        moderator_id: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody> {
        TwitchAPIRequest::new(
            EndpointType::GetChatSettings,
            self.build_url()
                .path(["chat", "settings"])
                .query([("broadcaster_id", broadcaster_id)])
                .query_option("moderator_id", moderator_id)
                .build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }

    fn get_shared_chat_session(&self, broadcaster_id: &str) -> TwitchAPIRequest<EmptyBody> {
        TwitchAPIRequest::new(
            EndpointType::GetChatSettings,
            self.build_url()
                .path(["shared_chat", "session"])
                .query([("broadcaster_id", broadcaster_id)])
                .build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }

    fn user_emotes(
        &self,
        user_id: &str,
        after: Option<&str>,
        broadcaster_id: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody> {
        TwitchAPIRequest::new(
            EndpointType::GetUserEmotes,
            self.build_url()
                .path(["chat", "emotes", "user"])
                .query([("user_id", user_id)])
                .query_option("after", after)
                .query_option("broadcaster_id", broadcaster_id)
                .build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }

    fn update_chat_settings(
        &self,
        broadcaster_id: &str,
        moderator_id: &str,
        configuration: UpdateChatSettingsRequest,
    ) -> TwitchAPIRequest<UpdateChatSettingsRequest> {
        TwitchAPIRequest::new(
            EndpointType::UpdateChatSettings,
            self.build_url()
                .path(["chat", "settings"])
                .query([("broadcaster_id", broadcaster_id)])
                .query([("moderator_id", moderator_id)])
                .build(),
            Method::PATCH,
            self.build_headers().json().build(),
            configuration,
        )
    }

    fn chat_write(
        &self,
        broadcaster_id: &str,
        sender_id: &str,
        message: &str,
    ) -> TwitchAPIRequest<SendChatMessageRequest> {
        TwitchAPIRequest::new(
            EndpointType::SendChatMessage,
            self.build_url().path(["chat", "messages"]).build(),
            Method::POST,
            self.build_headers().json().build(),
            SendChatMessageRequest::new(broadcaster_id, sender_id, message),
        )
    }

    fn user_chat_color<'a, T: IntoIterator<Item = &'a str>>(
        &self,
        user_ids: T,
    ) -> TwitchAPIRequest<EmptyBody> {
        TwitchAPIRequest::new(
            EndpointType::GetUserChatColor,
            self.build_url()
                .path(["chat", "color"])
                .query(user_ids.into_iter().map(|x| ("user_id", x)))
                .build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }

    fn update_user_chat_color(
        &self,
        user_id: &str,
        color: ChatColor,
    ) -> TwitchAPIRequest<EmptyBody> {
        TwitchAPIRequest::new(
            EndpointType::UpdateUserChatColor,
            self.build_url()
                .path(["chat", "color"])
                .query([("user_id", user_id), ("color", color.as_str())])
                .build(),
            Method::PUT,
            self.build_headers().build(),
            EmptyBody,
        )
    }
}
