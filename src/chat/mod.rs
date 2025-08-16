use asknothingx2_util::api::Method;
use request::{
    AnnouncementColor, ChatAnnouncementBody, ChatColor, SendChatMessageRequest,
    UpdateChatSettingsRequest,
};
use response::{
    BadgesResponse, ChatSettingResponse, ChattersResponse, EmotesResponse, SendChatMessageResponse,
    SharedChatSessionResponse, UsersColorResponse,
};

use crate::{
    request::{EndpointType, NoContent, TwitchAPIRequest},
    types::{
        constants::{AFTER, BROADCASTER_ID, CHAT, EMOTES, MODERATOR_ID, SETTINGS, USER_ID},
        BroadcasterId, ModeratorId, PaginationQuery, UserId,
    },
    TwitchAPI,
};

pub mod request;
pub mod response;
pub mod types;

twitch_api_trait! {
    #[cfg_attr(docsrs, doc(cfg(feature = "chat")))]
    trait ChatAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#get-chatters>
        fn get_chatters(
            &self,
            broadcaster_id: BroadcasterId,
            moderator_id: ModeratorId,
            pagination: Option<PaginationQuery>,
        ) -> ChattersResponse;
        /// <https://dev.twitch.tv/docs/api/reference/#get-channel-emotes>
        fn channel_emotes(
            &self,
            broadcaster_id: BroadcasterId,
        ) -> EmotesResponse;
        /// <https://dev.twitch.tv/docs/api/reference/#get-global-emotes>
        fn global_emotes(&self) -> EmotesResponse;
        /// <https://dev.twitch.tv/docs/api/reference/#get-emote-sets>
        fn emote_sets(
            &self,
            emote_set_ids: &[&str],
        ) -> EmotesResponse;
        /// <https://dev.twitch.tv/docs/api/reference/#get-channel-chat-badges>
        fn channel_badge(
            &self,
            broadcaster_id: BroadcasterId,
        ) -> BadgesResponse;
        /// <https://dev.twitch.tv/docs/api/reference/#get-global-chat-badges>
        fn global_badge(&self) -> BadgesResponse;
        /// <https://dev.twitch.tv/docs/api/reference/#get-chat-settings>
        fn get_chat_settings(
            &self,
            broadcaster_id: BroadcasterId,
            moderator_id: Option<ModeratorId>,
        ) -> ChatSettingResponse;
        /// <https://dev.twitch.tv/docs/api/reference/#get-shared-chat-session>
        fn get_shared_chat_session(
            &self,
            broadcaster_id: BroadcasterId,
        ) -> SharedChatSessionResponse;
        /// <https://dev.twitch.tv/docs/api/reference/#get-user-emotes>
        fn user_emotes(
            &self,
            user_id: UserId,
            after: Option<&str>,
            broadcaster_id: Option<BroadcasterId>,
        ) -> EmotesResponse;
        /// <https://dev.twitch.tv/docs/api/reference/#update-chat-settings>
        fn update_chat_settings(
            &self,
            broadcaster_id: BroadcasterId,
            moderator_id: ModeratorId,
            opts: Option<UpdateChatSettingsRequest>,
        ) -> ChatSettingResponse;
        /// <https://dev.twitch.tv/docs/api/reference/#send-chat-announcement>
        fn send_chat_announcement(
            &self,
            broadcaster_id: BroadcasterId,
            moderator_id: ModeratorId,
            message: &str,
            color: Option<AnnouncementColor>,
        ) -> NoContent;
        /// <https://dev.twitch.tv/docs/api/reference/#send-a-shoutout>
        fn send_a_shoutout(
            &self,
            from_broadcaster_id: BroadcasterId,
            to_broadcaster_id: BroadcasterId,
            moderator_id: ModeratorId,
        ) -> NoContent;
        /// <https://dev.twitch.tv/docs/api/reference/#send-chat-message>
        fn chat_write(
            &self,
            broadcaster_id: BroadcasterId,
            sender_id: &str,
            message: &str,
        ) -> SendChatMessageResponse;
        /// <https://dev.twitch.tv/docs/api/reference/#get-user-chat-color>
        fn user_chat_color(
            &self,
            user_id: &[UserId],
        ) -> UsersColorResponse;
        /// <https://dev.twitch.tv/docs/api/reference/#update-user-chat-color>
        fn update_user_chat_color(
            &self,
            user_id: UserId,
            color: ChatColor,
        ) -> NoContent;
    }
    impl {
        get_chatters => {
            endpoint_type: EndpointType::GetChatters,
            method: Method::GET,
            path: [CHAT, "chatters"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(MODERATOR_ID, moderator_id),
                pagination(pagination)
            }
        }
        channel_emotes => {
            endpoint_type: EndpointType::GetChannelEmotes,
            method: Method::GET,
            path: [CHAT, EMOTES],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id)
            }
        }
        global_emotes => {
            endpoint_type: EndpointType::GetGlobalEmotes,
            method: Method::GET,
            path: [CHAT, EMOTES, "global"]
        }
        emote_sets => {
            endpoint_type: EndpointType::GetEmoteSets,
            method: Method::GET,
            path: [CHAT, EMOTES, "set"],
            query_params: {
                extend(emote_set_ids.iter().map(|x| ("emote_set_id", *x)))
            }
        }
        channel_badge => {
            endpoint_type: EndpointType::GetChannelChatBadges,
            method: Method::GET,
            path: [CHAT, "badges"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id)
            }
        }
        global_badge => {
            endpoint_type: EndpointType::GetGlobalChatBadges,
            method: Method::GET,
            path: [CHAT, "badges", "global"]
        }
        get_chat_settings => {
            endpoint_type: EndpointType::GetChatSettings,
            method: Method::GET,
            path: [CHAT, SETTINGS],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                opt(MODERATOR_ID, moderator_id)
            }
        }
        get_shared_chat_session => {
            endpoint_type: EndpointType::GetShardChatSession,
            method: Method::GET,
            path: ["shared_chat", "session"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id)
            }
        }
        user_emotes => {
            endpoint_type: EndpointType::GetUserEmotes,
            method: Method::GET,
            path: [CHAT, EMOTES, "user"],
            query_params: {
                query(USER_ID, user_id),
                opt(AFTER, after),
                opt(BROADCASTER_ID, broadcaster_id)
            }
        }
        update_chat_settings => {
            endpoint_type: EndpointType::UpdateChatSettings,
            method: Method::PATCH,
            path: [CHAT, SETTINGS],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(MODERATOR_ID, moderator_id)
            },
            headers: [json],
            body: opts.and_then(|o| o.into_json())
        }
        send_chat_announcement => {
            endpoint_type: EndpointType::SendChatAnnouncement,
            method: Method::POST,
            path: [CHAT, "announcements"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(MODERATOR_ID, moderator_id)
            },
            headers: [json],
            body: ChatAnnouncementBody::new(message, color).into_json()
        }
        send_a_shoutout => {
            endpoint_type: EndpointType::SendAShoutout,
            method: Method::POST,
            path: [CHAT, "shoutouts"],
            query_params: {
                query("from_broadcaster_id", from_broadcaster_id),
                query("to_broadcaster_id", to_broadcaster_id),
                query(MODERATOR_ID, moderator_id)
            }
        }
        chat_write => {
            endpoint_type: EndpointType::SendChatMessage,
            method: Method::POST,
            path: [CHAT, "messages"],
            headers: [json],
            body: SendChatMessageRequest::new(broadcaster_id, sender_id, message).into_json()
        }
        user_chat_color => {
            endpoint_type: EndpointType::GetUserChatColor,
            method: Method::GET,
            path: [CHAT, "color"],
            query_params: {
                extend(user_id.iter().map(|x| (USER_ID, x)))
            }
        }
        update_user_chat_color => {
            endpoint_type: EndpointType::UpdateUserChatColor,
            method: Method::PUT,
            path: [CHAT, "color"],
            query_params: {
                query(USER_ID, user_id),
                query("color", color.as_str())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        chat::ChatAPI,
        test_utils::TwitchApiTest,
        types::{BroadcasterId, ModeratorId, PaginationQuery, UserId},
    };

    #[tokio::test]
    async fn get_chatters_endpoint() {
        let suite = TwitchApiTest::new().await;

        suite.mock_chat_success().await;

        let pagination = PaginationQuery::new().first(100);

        let response = suite
            .execute("/chat/chatters", |api| {
                api.get_chatters(
                    BroadcasterId::new("123456789"),
                    ModeratorId::new("987654321"),
                    Some(pagination),
                )
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 2);
        assert_eq!(response.total, 150);
        assert!(response.pagination.is_some());

        let first_chatter = &response.data[0];
        assert_eq!(first_chatter.user_id.as_str(), "123456789");
        assert_eq!(first_chatter.user_login, "activeuser1");

        let second_chatter = &response.data[1];
        assert_eq!(second_chatter.user_id.as_str(), "987654321");
        assert_eq!(second_chatter.user_login, "activeuser2");
    }

    #[tokio::test]
    async fn channel_emotes_endpoint() {
        let suite = TwitchApiTest::new().await;

        suite.mock_chat_success().await;

        let response = suite
            .execute("/chat/emotes", |api| {
                api.channel_emotes(BroadcasterId::new("123456789"))
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 1);
        assert!(response.template.contains("{{id}}"));

        let emote = &response.data[0];
        assert_eq!(emote.id.as_str(), "emotesv2_123");
        assert_eq!(emote.name, "testEmote1");
        assert!(emote.images.is_some());
        assert_eq!(emote.tier, Some("1000".to_string()));
    }

    #[tokio::test]
    async fn send_chat_message_endpoint() {
        let suite = TwitchApiTest::new().await;

        suite.mock_chat_success().await;

        let response = suite
            .execute("/chat/messages", |api| {
                api.chat_write(BroadcasterId::new("123456789"), "987654321", "Hello, chat!")
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 1);

        let message = &response.data[0];
        assert_eq!(message.message_id, "msg123456789");
        assert!(message.is_sent);
        assert!(message.drop_reason.is_none());
    }

    #[tokio::test]
    async fn get_chat_settings_endpoint() {
        let suite = TwitchApiTest::new().await;

        suite.mock_chat_success().await;

        let response = suite
            .execute("/chat/settings", |api| {
                api.get_chat_settings(
                    BroadcasterId::new("123456789"),
                    Some(ModeratorId::new("987654321")),
                )
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 1);

        let settings = &response.data[0];
        assert_eq!(settings.broadcaster_id.as_str(), "123456789");
        assert!(settings.follower_mode);
        assert_eq!(settings.follower_mode_duration, Some(300));
        assert!(!settings.slow_mode);
    }

    #[tokio::test]
    async fn user_chat_color_endpoint() {
        let suite = TwitchApiTest::new().await;

        suite.mock_chat_success().await;

        let user_ids = [UserId::new("123456789")];
        let response = suite
            .execute("/chat/color", |api| api.user_chat_color(&user_ids))
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 1);

        let user_color = &response.data[0];
        assert_eq!(user_color.user_id.as_str(), "123456789");
        assert_eq!(user_color.user_name, "TestUser");
        assert_eq!(user_color.color, "#FF0000");
    }

    #[tokio::test]
    async fn chat_api_error_response() {
        let suite = TwitchApiTest::new().await;

        suite.mock_chat_success().await;

        let response = suite
            .execute("/chat/chatters", |api| {
                api.get_chatters(
                    BroadcasterId::new("123456789"),
                    ModeratorId::new("987654321"),
                    None,
                )
            })
            .send()
            .await;

        match response {
            Ok(response) => {
                panic!("Expected Error, got: {response:?}")
            }
            Err(e) => {
                assert!(e.is_api());
            }
        }
    }
}
