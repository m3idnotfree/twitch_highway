use asknothingx2_util::api::Method;
use request::{
    AddBlockedTermRequest, AutoModAction, BanUserRequest, BanUsersRequest, CheckAutoMod,
    CheckAutoModStatusRequest, ManageHeldAutoModMeussageRequest, UpdateAutoModSettingsRequest,
    UpdateShieldModeStatusRequest, WarnChatUser, WarnChatUserRequest,
};
use response::{
    AutoModSettingsResponse, BanUsersResponse, BlockedTermsResponse, CheckAutoModStatusResponse,
    GetBannedUsersResponse, ModeratedChannelResponse, ModeratorsResponse, ShieldModeStatusResponse,
    UnbanRequestResponse, WarnChatUsersResponse,
};
use types::UnbanRequestStatus;

use crate::{
    request::{EndpointType, NoContent, TwitchAPIRequest},
    types::{
        constants::{BROADCASTER_ID, CHANNELS, CHAT, ID, MODERATOR_ID, SETTINGS, USER_ID},
        BroadcasterId, Id, ModeratorId, PaginationQuery, UserId,
    },
    TwitchAPI,
};

pub mod request;
pub mod response;
pub mod types;

const MODERATION: &str = "moderation";

twitch_api_trait! {
    #[cfg_attr(docsrs, doc(cfg(feature = "moderation")))]
    trait ModerationAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#check-automod-status>
        fn check_automod_status(
            &self,
            broadcaster_id: BroadcasterId,
            data: &[CheckAutoMod],
        ) -> CheckAutoModStatusResponse;
        /// <https://dev.twitch.tv/docs/api/reference/#manage-held-automod-messages>
        fn manage_held_automod_messages(
            &self,
            user_id: UserId,
            msg_id: &str,
            action: AutoModAction,
        ) -> NoContent;
        /// <https://dev.twitch.tv/docs/api/reference/#get-automod-settings>
        fn get_auto_mod_settings(
            &self,
            broadcaster_id: BroadcasterId,
            moderator_id: ModeratorId,
        ) -> AutoModSettingsResponse;
        /// <https://dev.twitch.tv/docs/api/reference/#update-automod-settings>
        fn update_auto_mod_settings(
            &self,
            broadcaster_id: BroadcasterId,
            moderator_id: ModeratorId,
            opts: Option<UpdateAutoModSettingsRequest>,
        ) -> AutoModSettingsResponse;
        /// <https://dev.twitch.tv/docs/api/reference/#get-banned-users>
        fn get_banned_users(
            &self,
            broadcaster_id: BroadcasterId,
            user_id: Option<&[UserId]>,
            pagination: Option<PaginationQuery>,
        ) -> GetBannedUsersResponse;
        /// <https://dev.twitch.tv/docs/api/reference/#ban-user>
        fn ban_users(
            &self,
            broadcaster_id: BroadcasterId,
            moderator_id: ModeratorId,
            data: BanUserRequest,
        ) -> BanUsersResponse;
        /// <https://dev.twitch.tv/docs/api/reference/#unban-user>
        fn unban_user(
            &self,
            broadcaster_id: BroadcasterId,
            moderator_id: ModeratorId,
            user_id: UserId,
        ) -> NoContent;
        /// <https://dev.twitch.tv/docs/api/reference/#get-unban-requests>
        fn get_unban_requests(
            &self,
            broadcaster_id: BroadcasterId,
            moderator_id: ModeratorId,
            status: UnbanRequestStatus,
            user_id: Option<UserId>,
            pagination: Option<PaginationQuery>,
        ) -> UnbanRequestResponse;
        /// <https://dev.twitch.tv/docs/api/reference/#resolve-unban-requests>
        fn resolve_unban_requests(
            &self,
            broadcaster_id: BroadcasterId,
            moderator_id: ModeratorId,
            unban_request_id: &str,
            status: UnbanRequestStatus,
            resolution_text: Option<&str>,
        ) -> UnbanRequestResponse;
        /// <https://dev.twitch.tv/docs/api/reference/#get-blocked-terms>
        fn get_blocked_terms(
            &self,
            broadcaster_id: BroadcasterId,
            moderator_id: ModeratorId,
            pagination: Option<PaginationQuery>,
        ) -> BlockedTermsResponse;
        /// <https://dev.twitch.tv/docs/api/reference/#add-blocked-term>
        fn add_blocked_term(
            &self,
            broadcaster_id: BroadcasterId,
            moderator_id: ModeratorId,
            text: &str,
        ) -> BlockedTermsResponse;
        /// <https://dev.twitch.tv/docs/api/reference/#remove-blocked-term>
        fn remove_blocked_term(
            &self,
            broadcaster_id: BroadcasterId,
            moderator_id: ModeratorId,
            id: Id,
        ) -> NoContent;
        /// <https://dev.twitch.tv/docs/api/reference/#delete-chat-messages>
        fn delete_chat_messages(
            &self,
            broadcaster_id: BroadcasterId,
            moderator_id: ModeratorId,
            message_id: Option<&str>,
        ) -> NoContent;
        /// <https://dev.twitch.tv/docs/api/reference/#get-moderated-channels>
        fn get_moderated_channels(
            &self,
            user_id: UserId,
            pagination: Option<PaginationQuery>,
        ) -> ModeratedChannelResponse;
        /// <https://dev.twitch.tv/docs/api/reference/#get-moderators>
        fn get_moderators(
            &self,
            broadcaster_id: BroadcasterId,
            user_id: Option<UserId>,
            pagination: Option<PaginationQuery>,
        ) -> ModeratorsResponse;
        /// <https://dev.twitch.tv/docs/api/reference/#add-channel-moderator>
        fn add_channel_moderator(
            &self,
            broadcaster_id: BroadcasterId,
            user_id: UserId,
        ) -> NoContent;
        /// <https://dev.twitch.tv/docs/api/reference/#remove-channel-moderator>
        fn remove_channel_moderator(
            &self,
            broadcaster_id: BroadcasterId,
            user_id: UserId,
        ) -> NoContent;
        /// <https://dev.twitch.tv/docs/api/reference/#get-vips>
        fn get_vips(
            &self,
            user_ids: Option<&[UserId]>,
            broadcaster_id: BroadcasterId,
            pagination: Option<PaginationQuery>,
        ) -> ModeratorsResponse;
        /// <https://dev.twitch.tv/docs/api/reference/#add-channel-vip>
        fn add_channel_vip(
            &self,
            user_id: UserId,
            broadcaster_id: BroadcasterId,
        ) -> NoContent;
        /// <https://dev.twitch.tv/docs/api/reference/#remove-channel-vip>
        fn remove_channel_vip(
            &self,
            user_id: UserId,
            broadcaster_id: BroadcasterId,
        ) -> NoContent;
        /// <https://dev.twitch.tv/docs/api/reference/#update-shield-mode-status>
        fn update_shield_mode_status(
            &self,
            broadcaster_id: BroadcasterId,
            moderator_id: ModeratorId,
            is_active: bool,
        ) -> ShieldModeStatusResponse;
        /// <https://dev.twitch.tv/docs/api/reference/#get-shield-mode-status>
        fn get_shield_mode_status(
            &self,
            broadcaster_id: BroadcasterId,
            moderator_id: ModeratorId,
        ) -> ShieldModeStatusResponse;
        /// <https://dev.twitch.tv/docs/api/reference/#warn-chat-user>
        fn warm_chat_user(
            &self,
            broadcaster_id: BroadcasterId,
            moderator_id: ModeratorId,
            data: &[WarnChatUser],
        ) -> WarnChatUsersResponse;
    }
    impl {
        check_automod_status => {
            endpoint_type: EndpointType::CheckAutoModStatus,
            method: Method::GET,
            path: [MODERATION, "enforcements", "status"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id)
            },
            headers: [json],
            body: CheckAutoModStatusRequest::new(data).into_json()
        }
        manage_held_automod_messages => {
            endpoint_type: EndpointType::ManageHeldAutoModMessages,
            method: Method::GET,
            path: [MODERATION, "automod", "message"],
            headers: [json],
            body:ManageHeldAutoModMeussageRequest::new(user_id, msg_id, action).into_json()
        }
        get_auto_mod_settings => {
            endpoint_type: EndpointType::GetAutoModSettings,
            method: Method::GET,
            path: [MODERATION, "automod", SETTINGS],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(MODERATOR_ID, moderator_id)
            }
        }
        update_auto_mod_settings => {
            endpoint_type: EndpointType::UpdateAutoModSettings,
            method: Method::PUT,
            path: [MODERATION, "automod", SETTINGS],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(MODERATOR_ID, moderator_id)
            },
            headers: [json],
            body: opts.and_then(|o| o.into_json())
        }
        get_banned_users => {
            endpoint_type: EndpointType::GetBannedUsers,
            method: Method::GET,
            path: [MODERATION, "banned"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                opt_extend(user_id.map(|ids| ids.iter().map(|id| (USER_ID, id)))),
                pagination(pagination)
            },
            headers: [json]
        }
        ban_users => {
            endpoint_type: EndpointType::BanUsers,
            method: Method::POST,
            path: [MODERATION, "bans"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(MODERATOR_ID, moderator_id)
            },
            headers: [json],
            body: BanUsersRequest::new(data).into_json()
        }
        unban_user => {
            endpoint_type: EndpointType::BanUsers,
            method: Method::DELETE,
            path: [MODERATION, "bans"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(MODERATOR_ID, moderator_id),
                query(USER_ID, user_id)
            }
        }
        get_unban_requests => {
            endpoint_type: EndpointType::GetUnbanRequests,
            method: Method::GET,
            path: [MODERATION, "unban_requests"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(MODERATOR_ID, moderator_id),
                query("status", status),
                opt(USER_ID, user_id),
                pagination(pagination)
            }
        }
        resolve_unban_requests => {
            endpoint_type: EndpointType::ResolveUnbanRequests,
            method: Method::PATCH,
            path: [MODERATION, "unban_requests"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(MODERATOR_ID, moderator_id),
                query("unban_request_id", unban_request_id),
                query("status", status),
                opt("resolution_text", resolution_text)
            }
        }
        get_blocked_terms => {
            endpoint_type: EndpointType::GetBlockedTerms,
            method: Method::GET,
            path: [MODERATION, "blocked_terms"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(MODERATOR_ID, moderator_id),
                pagination(pagination)
            }
        }
        add_blocked_term => {
            endpoint_type: EndpointType::AddBlockedTerm,
            method: Method::POST,
            path: [MODERATION, "blocked_terms"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(MODERATOR_ID, moderator_id)
            },
            headers: [json],
            body: AddBlockedTermRequest::new(text).into_json()
        }
        remove_blocked_term => {
            endpoint_type: EndpointType::RemoveBlockedTerm,
            method: Method::DELETE,
            path: [MODERATION, "blocked_terms"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(MODERATOR_ID, moderator_id),
                query(ID, id)
            }
        }
        delete_chat_messages => {
            endpoint_type: EndpointType::DeleteChatMessages,
            method: Method::DELETE,
            path: [MODERATION, CHAT],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(MODERATOR_ID, moderator_id),
                opt("message_id", message_id)
            }
        }
        get_moderated_channels => {
            endpoint_type: EndpointType::GetModeratedChannels,
            method: Method::GET,
            path: [MODERATION, CHANNELS],
            query_params: {
                query(USER_ID, user_id),
                pagination(pagination)
            }
        }
        get_moderators => {
            endpoint_type: EndpointType::GetModerators,
            method: Method::GET,
            path: [MODERATION, "moderators"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                opt(USER_ID, user_id),
                pagination(pagination)
            }
        }
        add_channel_moderator => {
            endpoint_type: EndpointType::AddChannelModerator,
            method: Method::POST,
            path: [MODERATION, "moderators"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(USER_ID, user_id)
            }
        }
        remove_channel_moderator => {
            endpoint_type: EndpointType::RemoveChannelModerator,
            method: Method::DELETE,
            path: [MODERATION, "moderators"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(USER_ID, user_id)
            }
        }
        get_vips => {
            endpoint_type: EndpointType::GetVIPs,
            method: Method::GET,
            path: [CHANNELS, "vips"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                opt_extend(user_ids.map(|ids| ids.iter().map(|id| (USER_ID, id)))),
                pagination(pagination)
            }
        }
        add_channel_vip => {
            endpoint_type: EndpointType::AddChannelVIP,
            method: Method::POST,
            path: [CHANNELS, "vips"],
            query_params: {
                query(USER_ID, user_id),
                query(BROADCASTER_ID, broadcaster_id)
            }
        }
        remove_channel_vip => {
            endpoint_type: EndpointType::RemoveChannelVIP,
            method: Method::DELETE,
            path: [CHANNELS, "vips"],
            query_params: {
                query(USER_ID, user_id),
                query(BROADCASTER_ID, broadcaster_id)
            }
        }
        update_shield_mode_status => {
            endpoint_type: EndpointType::UpdateShieldModeStatus,
            method: Method::PUT,
            path: [MODERATION, "shield_mode"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(MODERATOR_ID, moderator_id)
            },
            headers: [json],
            body: UpdateShieldModeStatusRequest::new(is_active).into_json()
        }
        get_shield_mode_status => {
            endpoint_type: EndpointType::GetShieldModeStatus,
            method: Method::GET,
            path: [MODERATION, "shield_mode"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(MODERATOR_ID, moderator_id)
            }
        }
        warm_chat_user => {
            endpoint_type: EndpointType::WarnChatUser,
            method: Method::GET,
            path: [MODERATION, "warnings"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(MODERATOR_ID, moderator_id)
            },
            body: WarnChatUserRequest::new(data).into_json()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        moderation::{
            request::{AutoModAction, BanUserRequest, CheckAutoMod, UpdateAutoModSettingsRequest},
            types::UnbanRequestStatus,
            ModerationAPI,
        },
        test_utils::TwitchApiTest,
        types::{BroadcasterId, Id, ModeratorId, PaginationQuery, UserId},
    };

    #[tokio::test]
    pub(crate) async fn check_automod_status() {
        let suite = TwitchApiTest::new().await;

        suite.mock_moderation_success().await;

        let check_data = vec![
            CheckAutoMod::new("test_msg_1".to_string(), "Hello everyone!".to_string()),
            CheckAutoMod::new("test_msg_2".to_string(), "Bad word here".to_string()),
        ];

        let response = suite
            .execute("/moderation/enforcements/status", |api| {
                api.check_automod_status(BroadcasterId::new("123456789"), &check_data)
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 2);

        let first_result = &response.data[0];
        assert_eq!(first_result.msg_id, "test_msg_1");
        assert!(first_result.is_permitted);

        let second_result = &response.data[1];
        assert_eq!(second_result.msg_id, "test_msg_2");
        assert!(!second_result.is_permitted);
    }

    #[tokio::test]
    pub(crate) async fn get_banned_users() {
        let suite = TwitchApiTest::new().await;

        suite.mock_moderation_success().await;

        let pagination = PaginationQuery::new().first(20);

        let response = suite
            .execute("/moderation/banned", |api| {
                api.get_banned_users(BroadcasterId::new("123456789"), None, Some(pagination))
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 1);

        let banned_user = &response.data[0];
        assert_eq!(banned_user.user_id.as_str(), "banned_user_123");
        assert_eq!(banned_user.user_login, "troublemaker");
        assert_eq!(banned_user.reason, "Harassment");
        assert_eq!(banned_user.moderator_id.as_str(), "mod456");
    }

    #[tokio::test]
    pub(crate) async fn manage_held_automod_messages() {
        let suite = TwitchApiTest::new().await;

        suite.mock_moderation_success().await;

        let response = suite
            .execute("/moderation/automod/message", |api| {
                api.manage_held_automod_messages(
                    UserId::new("123456789"),
                    "msg_123",
                    AutoModAction::ALLOW,
                )
            })
            .send()
            .await;

        assert!(response.is_ok());
        let response = response.unwrap();
        assert_eq!(response.status().as_u16(), 204);
    }

    #[tokio::test]
    pub(crate) async fn get_auto_mod_settings() {
        let suite = TwitchApiTest::new().await;

        suite.mock_moderation_success().await;

        let response = suite
            .execute("/moderation/automod/settings", |api| {
                api.get_auto_mod_settings(
                    BroadcasterId::new("123456789"),
                    ModeratorId::new("987654321"),
                )
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 1);

        let settings = &response.data[0];
        assert_eq!(settings.broadcaster_id.as_str(), "123456789");
        assert_eq!(settings.moderator_id, "987654321");
        assert_eq!(settings.overall_level, Some(2));
        assert_eq!(settings.aggression, 2);
    }

    #[tokio::test]
    pub(crate) async fn update_auto_mod_settings() {
        let suite = TwitchApiTest::new().await;

        suite.mock_moderation_success().await;

        let update_request = UpdateAutoModSettingsRequest::new()
            .overall_level(3)
            .aggression(3)
            .bullying(2)
            .swearing(4);

        let response = suite
            .execute("/moderation/automod/settings", |api| {
                api.update_auto_mod_settings(
                    BroadcasterId::new("123456789"),
                    ModeratorId::new("987654321"),
                    Some(update_request),
                )
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 1);

        let settings = &response.data[0];
        assert_eq!(settings.overall_level, Some(3));
        assert_eq!(settings.aggression, 3);
    }

    #[tokio::test]
    pub(crate) async fn ban_users() {
        let suite = TwitchApiTest::new().await;

        suite.mock_moderation_success().await;

        let ban_request = BanUserRequest::new(UserId::new("troublemaker123"))
            .duration(86400)
            .reason("Harassment");

        let response = suite
            .execute("/moderation/bans", |api| {
                api.ban_users(
                    BroadcasterId::new("123456789"),
                    ModeratorId::new("987654321"),
                    ban_request,
                )
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 1);

        let ban_result = &response.data[0];
        assert_eq!(ban_result.broadcaster_id.as_str(), "123456789");
        assert_eq!(ban_result.moderator_id.as_str(), "987654321");
        assert_eq!(ban_result.user_id.as_str(), "troublemaker123");
    }

    #[tokio::test]
    pub(crate) async fn unban_user() {
        let suite = TwitchApiTest::new().await;

        suite.mock_moderation_success().await;

        let response = suite
            .execute("/moderation/bans", |api| {
                api.unban_user(
                    BroadcasterId::new("123456789"),
                    ModeratorId::new("987654321"),
                    UserId::new("unbanned_user_123"),
                )
            })
            .send()
            .await;

        assert!(response.is_ok());
        let response = response.unwrap();
        assert_eq!(response.status().as_u16(), 204);
    }

    #[tokio::test]
    pub(crate) async fn get_unban_requests() {
        let suite = TwitchApiTest::new().await;

        suite.mock_moderation_success().await;

        let response = suite
            .execute("/moderation/unban_requests", |api| {
                api.get_unban_requests(
                    BroadcasterId::new("123456789"),
                    ModeratorId::new("987654321"),
                    UnbanRequestStatus::Pending,
                    None,
                    None,
                )
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 1);

        let unban_request = &response.data[0];
        assert_eq!(unban_request.id.as_str(), "unban_req_123");
        assert_eq!(unban_request.user_id.as_str(), "requesting_user");
        assert_eq!(unban_request.text, "I promise to behave better");
    }

    #[tokio::test]
    pub(crate) async fn resolve_unban_requests() {
        let suite = TwitchApiTest::new().await;

        suite.mock_moderation_success().await;

        let response = suite
            .execute("/moderation/unban_requests", |api| {
                api.resolve_unban_requests(
                    BroadcasterId::new("123456789"),
                    ModeratorId::new("987654321"),
                    "unban_req_123",
                    UnbanRequestStatus::Approved,
                    Some("Request approved"),
                )
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 1);

        let resolved_request = &response.data[0];
        assert_eq!(resolved_request.id.as_str(), "unban_req_123");
        assert_eq!(
            resolved_request.resolution_text,
            Some("Request approved".to_string())
        );
    }

    #[tokio::test]
    pub(crate) async fn get_blocked_terms() {
        let suite = TwitchApiTest::new().await;

        suite.mock_moderation_success().await;

        let response = suite
            .execute("/moderation/blocked_terms", |api| {
                api.get_blocked_terms(
                    BroadcasterId::new("123456789"),
                    ModeratorId::new("987654321"),
                    None,
                )
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 1);

        let blocked_term = &response.data[0];
        assert_eq!(blocked_term.text, "badword");
        assert_eq!(blocked_term.broadcaster_id.as_str(), "123456789");
    }

    #[tokio::test]
    pub(crate) async fn add_blocked_term() {
        let suite = TwitchApiTest::new().await;

        suite.mock_moderation_success().await;

        let response = suite
            .execute("/moderation/blocked_terms", |api| {
                api.add_blocked_term(
                    BroadcasterId::new("123456789"),
                    ModeratorId::new("987654321"),
                    "newbadword",
                )
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 1);

        let new_blocked_term = &response.data[0];
        assert_eq!(new_blocked_term.text, "newbadword");
    }

    #[tokio::test]
    pub(crate) async fn remove_blocked_term() {
        let suite = TwitchApiTest::new().await;

        suite.mock_moderation_success().await;

        let response = suite
            .execute("/moderation/blocked_terms", |api| {
                api.remove_blocked_term(
                    BroadcasterId::new("123456789"),
                    ModeratorId::new("987654321"),
                    Id::new("blocked_term_123"),
                )
            })
            .send()
            .await;

        assert!(response.is_ok());
        let response = response.unwrap();
        assert_eq!(response.status().as_u16(), 204);
    }

    #[tokio::test]
    pub(crate) async fn delete_chat_messages() {
        let suite = TwitchApiTest::new().await;

        suite.mock_moderation_success().await;

        let response = suite
            .execute("/moderation/chat", |api| {
                api.delete_chat_messages(
                    BroadcasterId::new("123456789"),
                    ModeratorId::new("987654321"),
                    Some("msg_to_delete_123"),
                )
            })
            .send()
            .await;

        assert!(response.is_ok());
        let response = response.unwrap();
        assert_eq!(response.status().as_u16(), 204);
    }

    #[tokio::test]
    pub(crate) async fn delete_all_chat_messages() {
        let suite = TwitchApiTest::new().await;

        suite.mock_moderation_success().await;

        let response = suite
            .execute("/moderation/chat", |api| {
                api.delete_chat_messages(
                    BroadcasterId::new("123456789"),
                    ModeratorId::new("987654321"),
                    None,
                )
            })
            .send()
            .await;

        assert!(response.is_ok());
        let response = response.unwrap();
        assert_eq!(response.status().as_u16(), 204);
    }

    #[tokio::test]
    pub(crate) async fn get_moderated_channels() {
        let suite = TwitchApiTest::new().await;

        suite.mock_moderation_success().await;

        let response = suite
            .execute("/moderation/channels", |api| {
                api.get_moderated_channels(UserId::new("123456789"), None)
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 1);

        let moderated_channel = &response.data[0];
        assert_eq!(moderated_channel.broadcaster_id.as_str(), "987654321");
        assert_eq!(moderated_channel.broadcaster_login, "streamerchannel");
        assert_eq!(moderated_channel.broadcaster_name, "StreamerChannel");
    }

    #[tokio::test]
    pub(crate) async fn get_moderators() {
        let suite = TwitchApiTest::new().await;

        suite.mock_moderation_success().await;

        let response = suite
            .execute("/moderation/moderators", |api| {
                api.get_moderators(BroadcasterId::new("123456789"), None, None)
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 1);

        let moderator = &response.data[0];
        assert_eq!(moderator.user_id.as_str(), "mod_user_123");
        assert_eq!(moderator.user_login, "headmoderator");
        assert_eq!(moderator.user_name, "HeadModerator");
    }

    #[tokio::test]
    pub(crate) async fn add_channel_moderator() {
        let suite = TwitchApiTest::new().await;

        suite.mock_moderation_success().await;

        let response = suite
            .execute("/moderation/moderators", |api| {
                api.add_channel_moderator(
                    BroadcasterId::new("123456789"),
                    UserId::new("new_mod_123"),
                )
            })
            .send()
            .await;

        assert!(response.is_ok());
        let response = response.unwrap();
        assert_eq!(response.status().as_u16(), 204);
    }

    #[tokio::test]
    pub(crate) async fn remove_channel_moderator() {
        let suite = TwitchApiTest::new().await;

        suite.mock_moderation_success().await;

        let response = suite
            .execute("/moderation/moderators", |api| {
                api.remove_channel_moderator(
                    BroadcasterId::new("123456789"),
                    UserId::new("old_mod_123"),
                )
            })
            .send()
            .await;

        assert!(response.is_ok());
        let response = response.unwrap();
        assert_eq!(response.status().as_u16(), 204);
    }

    #[tokio::test]
    pub(crate) async fn get_vips() {
        let suite = TwitchApiTest::new().await;

        suite.mock_moderation_success().await;

        let response = suite
            .execute("/channels/vips", |api| {
                api.get_vips(None, BroadcasterId::new("123456789"), None)
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 1);

        let vip = &response.data[0];
        assert_eq!(vip.user_id.as_str(), "vip_user_123");
        assert_eq!(vip.user_login, "vipuser");
        assert_eq!(vip.user_name, "VIPUser");
    }

    #[tokio::test]
    pub(crate) async fn add_channel_vip() {
        let suite = TwitchApiTest::new().await;

        suite.mock_moderation_success().await;

        let response = suite
            .execute("/channels/vips", |api| {
                api.add_channel_vip(UserId::new("new_vip_123"), BroadcasterId::new("123456789"))
            })
            .send()
            .await;

        assert!(response.is_ok());
        let response = response.unwrap();
        assert_eq!(response.status().as_u16(), 204);
    }

    #[tokio::test]
    pub(crate) async fn remove_channel_vip() {
        let suite = TwitchApiTest::new().await;

        suite.mock_moderation_success().await;

        let response = suite
            .execute("/channels/vips", |api| {
                api.remove_channel_vip(UserId::new("old_vip_123"), BroadcasterId::new("123456789"))
            })
            .send()
            .await;

        assert!(response.is_ok());
        let response = response.unwrap();
        assert_eq!(response.status().as_u16(), 204);
    }

    #[tokio::test]
    pub(crate) async fn update_shield_mode_status() {
        let suite = TwitchApiTest::new().await;

        suite.mock_moderation_success().await;

        let response = suite
            .execute("/moderation/shield_mode", |api| {
                api.update_shield_mode_status(
                    BroadcasterId::new("123456789"),
                    ModeratorId::new("987654321"),
                    true,
                )
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 1);

        let shield_status = &response.data[0];
        assert!(shield_status.is_active);
        assert_eq!(shield_status.moderator_id.as_str(), "987654321");
        assert_eq!(shield_status.moderator_name, "HeadModerator");
    }

    #[tokio::test]
    pub(crate) async fn get_shield_mode_status() {
        let suite = TwitchApiTest::new().await;

        suite.mock_moderation_success().await;

        let response = suite
            .execute("/moderation/shield_mode", |api| {
                api.get_shield_mode_status(
                    BroadcasterId::new("123456789"),
                    ModeratorId::new("987654321"),
                )
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 1);

        let shield_status = &response.data[0];
        assert!(!shield_status.is_active);
        assert_eq!(shield_status.moderator_id.as_str(), "987654321");
    }

    #[tokio::test]
    pub(crate) async fn warm_chat_user() {
        let suite = TwitchApiTest::new().await;

        suite.mock_moderation_success().await;

        let warn_data = vec![
            super::request::WarnChatUser::new(
                UserId::new("warned_user_1"),
                "Inappropriate language",
            ),
            super::request::WarnChatUser::new(UserId::new("warned_user_2"), "Spam"),
        ];

        let response = suite
            .execute("/moderation/warnings", |api| {
                api.warm_chat_user(
                    BroadcasterId::new("123456789"),
                    ModeratorId::new("987654321"),
                    &warn_data,
                )
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 2);

        let first_warning = &response.data[0];
        assert_eq!(first_warning.broadcaster_id.as_str(), "123456789");
        assert_eq!(first_warning.user_id.as_str(), "warned_user_1");
        assert_eq!(first_warning.reason, "Inappropriate language");

        let second_warning = &response.data[1];
        assert_eq!(second_warning.user_id.as_str(), "warned_user_2");
        assert_eq!(second_warning.reason, "Spam");
    }
}
