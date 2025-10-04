mod request;
mod response;
mod types;

pub use request::{
    AnnouncementColor, ChatAnnouncementBody, ChatColor, SendChatMessageRequest,
    UpdateChatSettingsRequest,
};
pub use response::{
    BadgesResponse, ChatSettingResponse, ChattersResponse, EmotesResponse, SendChatMessageResponse,
    SharedChatSessionResponse, UsersColorResponse,
};
pub use types::{
    Badge, BroadcasterIdField, ChatSetting, Chatter, DropReason, Emote, EmoteType, Format,
    MessageResponse, Scale, SharedChatSession, ThemeMode, UserColor, Version,
};

use crate::{
    request::NoContent,
    types::{
        constants::{AFTER, BROADCASTER_ID, CHAT, MODERATOR_ID, SETTINGS, USER_ID},
        BroadcasterId, ModeratorId, PaginationQuery, UserId,
    },
};

const EMOTES: &str = "emotes";

endpoints! {
    ChatAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#get-chatters>
        fn get_chatters(
            &self,
            broadcaster_id: &BroadcasterId,
            moderator_id: &ModeratorId,
            pagination: Option<PaginationQuery>,
        ) -> ChattersResponse {
            endpoint_type: GetChatters,
            method: GET,
            path: [CHAT, "chatters"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(MODERATOR_ID, moderator_id),
                pagination(pagination)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#get-channel-emotes>
        fn get_channel_emotes(
            &self,
            broadcaster_id: &BroadcasterId,
        ) -> EmotesResponse {
            endpoint_type: GetChannelEmotes,
            method: GET,
            path: [CHAT, EMOTES],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#get-global-emotes>
        fn get_global_emotes(&self) -> EmotesResponse {
            endpoint_type: GetGlobalEmotes,
            method: GET,
            path: [CHAT, EMOTES, "global"]
        }

        /// <https://dev.twitch.tv/docs/api/reference/#get-emote-sets>
        fn get_emote_sets(
            &self,
            emote_set_ids: &[&str],
        ) -> EmotesResponse {
            endpoint_type: GetEmoteSets,
            method: GET,
            path: [CHAT, EMOTES, "set"],
            query_params: {
                extend(emote_set_ids.iter().map(|x| ("emote_set_id", *x)))
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#get-channel-chat-badges>
        fn get_channel_chat_badges(
            &self,
            broadcaster_id: &BroadcasterId,
        ) -> BadgesResponse {
            endpoint_type: GetChannelChatBadges,
            method: GET,
            path: [CHAT, "badges"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#get-global-chat-badges>
        fn get_global_chat_badges(&self) -> BadgesResponse {
            endpoint_type: GetGlobalChatBadges,
            method: GET,
            path: [CHAT, "badges", "global"]
        }

        /// <https://dev.twitch.tv/docs/api/reference/#get-chat-settings>
        fn get_chat_settings(
            &self,
            broadcaster_id: &BroadcasterId,
            moderator_id: Option<&ModeratorId>,
        ) -> ChatSettingResponse {
            endpoint_type: GetChatSettings,
            method: GET,
            path: [CHAT, SETTINGS],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                opt(MODERATOR_ID, moderator_id)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#get-shared-chat-session>
        fn get_shared_chat_session(
            &self,
            broadcaster_id: &BroadcasterId,
        ) -> SharedChatSessionResponse {
            endpoint_type: GetShardChatSession,
            method: GET,
            path: ["shared_chat", "session"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#get-user-emotes>
        fn get_user_emotes(
            &self,
            user_id: &UserId,
            after: Option<&str>,
            broadcaster_id: Option<BroadcasterId>,
        ) -> EmotesResponse {
            endpoint_type: GetUserEmotes,
            method: GET,
            path: [CHAT, EMOTES, "user"],
            query_params: {
                query(USER_ID, user_id),
                opt(AFTER, after),
                opt(BROADCASTER_ID, broadcaster_id)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#update-chat-settings>
        fn update_chat_settings(
            &self,
            broadcaster_id: &BroadcasterId,
            moderator_id: &ModeratorId,
            update: UpdateChatSettingsRequest,
        ) -> ChatSettingResponse {
            endpoint_type: UpdateChatSettings,
            method: PATCH,
            path: [CHAT, SETTINGS],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(MODERATOR_ID, moderator_id)
            },
            headers: [json],
            body: update.into_json()
        }

        /// <https://dev.twitch.tv/docs/api/reference/#send-chat-announcement>
        fn send_chat_announcement(
            &self,
            broadcaster_id: &BroadcasterId,
            moderator_id: &ModeratorId,
            message: &str,
            color: Option<AnnouncementColor>,
        ) -> NoContent {
            endpoint_type: SendChatAnnouncement,
            method: POST,
            path: [CHAT, "announcements"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(MODERATOR_ID, moderator_id)
            },
            headers: [json],
            body: ChatAnnouncementBody::new(message, color).into_json()
        }

        /// <https://dev.twitch.tv/docs/api/reference/#send-a-shoutout>
        fn send_a_shoutout(
            &self,
            from_broadcaster_id: &BroadcasterId,
            to_broadcaster_id: &BroadcasterId,
            moderator_id: &ModeratorId,
        ) -> NoContent {
            endpoint_type: SendAShoutout,
            method: POST,
            path: [CHAT, "shoutouts"],
            query_params: {
                query("from_broadcaster_id", from_broadcaster_id),
                query("to_broadcaster_id", to_broadcaster_id),
                query(MODERATOR_ID, moderator_id)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#send-chat-message>
        fn send_chat_message(
            &self,
            req: SendChatMessageRequest,
        ) -> SendChatMessageResponse {
            endpoint_type: SendChatMessage,
            method: POST,
            path: [CHAT, "messages"],
            headers: [json],
            body: req.into_json()
        }

        /// <https://dev.twitch.tv/docs/api/reference/#get-user-chat-color>
        fn get_user_chat_color(
            &self,
            user_id: &[UserId],
        ) -> UsersColorResponse {
            endpoint_type: GetUserChatColor,
            method: GET,
            path: [CHAT, "color"],
            query_params: {
                extend(user_id.iter().map(|x| (USER_ID, x)))
            }
        }
        /// <https://dev.twitch.tv/docs/api/reference/#update-user-chat-color>
        fn update_user_chat_color(
            &self,
            user_id: &UserId,
            color: ChatColor,
        ) -> NoContent {
            endpoint_type: UpdateUserChatColor,
            method: PUT,
            path: [CHAT, "color"],
            query_params: {
                query(USER_ID, user_id),
                query("color", color.as_str())
            }
        }
    }
}
