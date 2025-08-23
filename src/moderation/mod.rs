pub mod request;
pub mod response;
pub mod types;

use asknothingx2_util::api::Method;
use request::{
    AddBlockedTermRequest, AutoModAction, BanUserRequest, CheckAutoMod, CheckAutoModStatusRequest,
    ManageHeldAutoModMeussageRequest, UpdateAutoModSettingsRequest, UpdateShieldModeStatusRequest,
    WarnChatUser, WarnChatUserRequest,
};
use response::{
    AutoModSettingsResponse, BanUsersResponse, BlockedTermsResponse, CheckAutoModStatusResponse,
    GetBannedUsersResponse, ModeratedChannelResponse, ModeratorsResponse, ShieldModeStatusResponse,
    UnbanRequestResponse, WarnChatUsersResponse,
};
use types::UnbanRequestStatus;

use crate::{
    moderation::request::BanUserRequestWrapper,
    request::{EndpointType, NoContent},
    types::{
        constants::{BROADCASTER_ID, CHANNELS, CHAT, ID, MODERATOR_ID, SETTINGS, USER_ID},
        BroadcasterId, Id, ModeratorId, PaginationQuery, UserId,
    },
};

const MODERATION: &str = "moderation";

endpoints! {
    ModerationAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#check-automod-status>
        fn check_automod_status(
            &self,
            broadcaster_id: &BroadcasterId,
            data: &[CheckAutoMod],
        ) -> CheckAutoModStatusResponse {
            endpoint_type: EndpointType::CheckAutoModStatus,
            method: Method::POST,
            path: [MODERATION, "enforcements", "status"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id)
            },
            headers: [json],
            body: CheckAutoModStatusRequest::new(data).into_json()
        }

        /// <https://dev.twitch.tv/docs/api/reference/#manage-held-automod-messages>
        fn manage_held_automod_messages(
            &self,
            user_id: &UserId,
            msg_id: &str,
            action: AutoModAction,
        ) -> NoContent {
            endpoint_type: EndpointType::ManageHeldAutoModMessages,
            method: Method::POST,
            path: [MODERATION, "automod", "message"],
            headers: [json],
            body:ManageHeldAutoModMeussageRequest::new(user_id, msg_id, action).into_json()
        }

        /// <https://dev.twitch.tv/docs/api/reference/#get-automod-settings>
        fn get_automod_settings(
            &self,
            broadcaster_id: &BroadcasterId,
            moderator_id: &ModeratorId,
        ) -> AutoModSettingsResponse {
            endpoint_type: EndpointType::GetAutoModSettings,
            method: Method::GET,
            path: [MODERATION, "automod", SETTINGS],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(MODERATOR_ID, moderator_id)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#update-automod-settings>
        fn update_automod_settings(
            &self,
            broadcaster_id: &BroadcasterId,
            moderator_id: &ModeratorId,
            opts: Option<UpdateAutoModSettingsRequest>,
        ) -> AutoModSettingsResponse {
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

        /// <https://dev.twitch.tv/docs/api/reference/#get-banned-users>
        fn get_banned_users(
            &self,
            broadcaster_id: &BroadcasterId,
            user_id: Option<&[UserId]>,
            pagination: Option<PaginationQuery>,
        ) -> GetBannedUsersResponse {
            endpoint_type: EndpointType::GetBannedUsers,
            method: Method::GET,
            path: [MODERATION, "banned"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                opt_extend(user_id.map(|ids| ids.iter().map(|id| (USER_ID, id)))),
                pagination(pagination)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#ban-user>
        fn ban_user(
            &self,
            broadcaster_id: &BroadcasterId,
            moderator_id: &ModeratorId,
            data: BanUserRequest,
        ) -> BanUsersResponse {
            endpoint_type: EndpointType::BanUsers,
            method: Method::POST,
            path: [MODERATION, "bans"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(MODERATOR_ID, moderator_id)
            },
            headers: [json],
            body: BanUserRequestWrapper::new(data).into_json()
        }

        /// <https://dev.twitch.tv/docs/api/reference/#unban-user>
        fn unban_user(
            &self,
            broadcaster_id: &BroadcasterId,
            moderator_id: &ModeratorId,
            user_id: &UserId,
        ) -> NoContent {
            endpoint_type: EndpointType::BanUsers,
            method: Method::DELETE,
            path: [MODERATION, "bans"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(MODERATOR_ID, moderator_id),
                query(USER_ID, user_id)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#get-unban-requests>
        fn get_unban_requests(
            &self,
            broadcaster_id: &BroadcasterId,
            moderator_id: &ModeratorId,
            status: UnbanRequestStatus,
            user_id: Option<&UserId>,
            pagination: Option<PaginationQuery>,
        ) -> UnbanRequestResponse {
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

        /// <https://dev.twitch.tv/docs/api/reference/#resolve-unban-requests>
        fn resolve_unban_request(
            &self,
            broadcaster_id: &BroadcasterId,
            moderator_id: &ModeratorId,
            unban_request_id: &str,
            status: UnbanRequestStatus,
            resolution_text: Option<&str>,
        ) -> UnbanRequestResponse {
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

        /// <https://dev.twitch.tv/docs/api/reference/#get-blocked-terms>
        fn get_blocked_terms(
            &self,
            broadcaster_id: &BroadcasterId,
            moderator_id: &ModeratorId,
            pagination: Option<PaginationQuery>,
        ) -> BlockedTermsResponse {
            endpoint_type: EndpointType::GetBlockedTerms,
            method: Method::GET,
            path: [MODERATION, "blocked_terms"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(MODERATOR_ID, moderator_id),
                pagination(pagination)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#add-blocked-term>
        fn add_blocked_term(
            &self,
            broadcaster_id: &BroadcasterId,
            moderator_id: &ModeratorId,
            text: &str,
        ) -> BlockedTermsResponse {
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

        /// <https://dev.twitch.tv/docs/api/reference/#remove-blocked-term>
        fn remove_blocked_term(
            &self,
            broadcaster_id: &BroadcasterId,
            moderator_id: &ModeratorId,
            id: &Id,
        ) -> NoContent {
            endpoint_type: EndpointType::RemoveBlockedTerm,
            method: Method::DELETE,
            path: [MODERATION, "blocked_terms"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(MODERATOR_ID, moderator_id),
                query(ID, id)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#delete-chat-messages>
        fn delete_chat_messages(
            &self,
            broadcaster_id: &BroadcasterId,
            moderator_id: &ModeratorId,
            message_id: Option<&str>,
        ) -> NoContent {
            endpoint_type: EndpointType::DeleteChatMessages,
            method: Method::DELETE,
            path: [MODERATION, CHAT],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(MODERATOR_ID, moderator_id),
                opt("message_id", message_id)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#get-moderated-channels>
        fn get_moderated_channels(
            &self,
            user_id: &UserId,
            pagination: Option<PaginationQuery>,
        ) -> ModeratedChannelResponse {
            endpoint_type: EndpointType::GetModeratedChannels,
            method: Method::GET,
            path: [MODERATION, CHANNELS],
            query_params: {
                query(USER_ID, user_id),
                pagination(pagination)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#get-moderators>
        fn get_moderators(
            &self,
            broadcaster_id: &BroadcasterId,
            user_id: Option<&UserId>,
            pagination: Option<PaginationQuery>,
        ) -> ModeratorsResponse {
            endpoint_type: EndpointType::GetModerators,
            method: Method::GET,
            path: [MODERATION, "moderators"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                opt(USER_ID, user_id),
                pagination(pagination)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#add-channel-moderator>
        fn add_channel_moderator(
            &self,
            broadcaster_id: &BroadcasterId,
            user_id: &UserId,
        ) -> NoContent {
            endpoint_type: EndpointType::AddChannelModerator,
            method: Method::POST,
            path: [MODERATION, "moderators"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(USER_ID, user_id)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#remove-channel-moderator>
        fn remove_channel_moderator(
            &self,
            broadcaster_id: &BroadcasterId,
            user_id: &UserId,
        ) -> NoContent {
            endpoint_type: EndpointType::RemoveChannelModerator,
            method: Method::DELETE,
            path: [MODERATION, "moderators"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(USER_ID, user_id)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#get-vips>
        fn get_vips(
            &self,
            broadcaster_id: &BroadcasterId,
            user_ids: Option<&[UserId]>,
            pagination: Option<PaginationQuery>,
        ) -> ModeratorsResponse {
            endpoint_type: EndpointType::GetVIPs,
            method: Method::GET,
            path: [CHANNELS, "vips"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                opt_extend(user_ids.map(|ids| ids.iter().map(|id| (USER_ID, id)))),
                pagination(pagination)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#add-channel-vip>
        fn add_channel_vip(
            &self,
            user_id: &UserId,
            broadcaster_id: &BroadcasterId,
        ) -> NoContent {
            endpoint_type: EndpointType::AddChannelVIP,
            method: Method::POST,
            path: [CHANNELS, "vips"],
            query_params: {
                query(USER_ID, user_id),
                query(BROADCASTER_ID, broadcaster_id)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#remove-channel-vip>
        fn remove_channel_vip(
            &self,
            user_id: &UserId,
            broadcaster_id: &BroadcasterId,
        ) -> NoContent {
            endpoint_type: EndpointType::RemoveChannelVIP,
            method: Method::DELETE,
            path: [CHANNELS, "vips"],
            query_params: {
                query(USER_ID, user_id),
                query(BROADCASTER_ID, broadcaster_id)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#update-shield-mode-status>
        fn update_shield_mode_status(
            &self,
            broadcaster_id: &BroadcasterId,
            moderator_id: &ModeratorId,
            is_active: bool,
        ) -> ShieldModeStatusResponse {
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

        /// <https://dev.twitch.tv/docs/api/reference/#get-shield-mode-status>
        fn get_shield_mode_status(
            &self,
            broadcaster_id: &BroadcasterId,
            moderator_id: &ModeratorId,
        ) -> ShieldModeStatusResponse {
            endpoint_type: EndpointType::GetShieldModeStatus,
            method: Method::GET,
            path: [MODERATION, "shield_mode"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(MODERATOR_ID, moderator_id)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#warn-chat-user>
        fn warn_chat_user(
            &self,
            broadcaster_id: &BroadcasterId,
            moderator_id: &ModeratorId,
            data: WarnChatUser,
        ) -> WarnChatUsersResponse {
            endpoint_type: EndpointType::WarnChatUser,
            method: Method::POST,
            path: [MODERATION, "warnings"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(MODERATOR_ID, moderator_id)
            },
            headers: [json],
            body: WarnChatUserRequest::new(data).into_json()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        moderation::{
            request::{
                AutoModAction, BanUserRequest, CheckAutoMod, UpdateAutoModSettingsRequest,
                WarnChatUser,
            },
            types::UnbanRequestStatus,
            ModerationAPI,
        },
        test_utils::TwitchApiTest,
        types::{BroadcasterId, Id, ModeratorId, PaginationQuery, UserId},
    };

    api_test!(
        check_automod_status,
        [
            &BroadcasterId::from("12345"),
            &[
                CheckAutoMod::new("123", "Hello World!"),
                CheckAutoMod::new("393", "Boooooo!"),
            ]
        ]
    );

    api_test!(
        manage_held_automod_messages,
        [&UserId::from("9327994"), "836013710", AutoModAction::ALLOW,]
    );

    api_test!(
        get_automod_settings,
        [&BroadcasterId::from("1234"), &ModeratorId::from("5678"),]
    );

    api_test!(
        update_automod_settings,
        [
            &BroadcasterId::from("1234"),
            &ModeratorId::from("5678"),
            Some(UpdateAutoModSettingsRequest::new().overall_level(3)),
        ]
    );

    api_test!(
        get_banned_users,
        [&BroadcasterId::from("198704263"), None, None]
    );

    api_test!(
        ban_user,
        [
            &BroadcasterId::from("1234"),
            &ModeratorId::from("5678"),
            BanUserRequest::new(&UserId::from("9876")).reason("no reason"),
        ]
    );

    api_test!(
        unban_user,
        [
            &BroadcasterId::from("1234"),
            &ModeratorId::from("5678"),
            &UserId::from("5432"),
        ]
    );

    api_test!(
        get_unban_requests,
        [
            &BroadcasterId::from("274637212"),
            &ModeratorId::from("274637212"),
            UnbanRequestStatus::Pending,
            None,
            None,
        ]
    );

    api_test!(
        resolve_unban_request,
        [
            &BroadcasterId::from("274637212"),
            &ModeratorId::from("987654321"),
            "92af127c-7326-4483-a52b-b0daa0be61c01",
            UnbanRequestStatus::Approved,
            None
        ]
    );

    api_test!(
        get_blocked_terms,
        [
            &BroadcasterId::from("1234"),
            &ModeratorId::from("5678"),
            Some(PaginationQuery::new().first(10)),
        ]
    );

    api_test!(
        add_blocked_term,
        [
            &BroadcasterId::from("1234"),
            &ModeratorId::from("5678"),
            "A phrase I’m not fond of",
        ]
    );

    api_test!(
        remove_blocked_term,
        [
            &BroadcasterId::from("1234"),
            &ModeratorId::from("5678"),
            &Id::from("c9fc79b8-0f63-4ef7-9d38-efd811e74ac2"),
        ]
    );

    api_test!(
        delete_chat_messages,
        [
            &BroadcasterId::from("11111"),
            &ModeratorId::from("44444"),
            Some("abc-123-def"),
        ]
    );

    api_test!(get_moderated_channels, [&UserId::from("931931"), None]);

    api_test!(
        get_moderators,
        [&BroadcasterId::from("198704263"), None, None]
    );

    api_test!(
        add_channel_moderator,
        [&BroadcasterId::from("11111"), &UserId::from("44444"),]
    );

    api_test!(
        remove_channel_moderator,
        [&BroadcasterId::from("11111"), &UserId::from("44444"),]
    );

    api_test!(get_vips, [&BroadcasterId::from("123"), None, None]);

    api_test!(
        add_channel_vip,
        [&UserId::from("456"), &BroadcasterId::from("123")]
    );

    api_test!(
        remove_channel_vip,
        [&UserId::from("456"), &BroadcasterId::from("123")]
    );

    api_test!(
        update_shield_mode_status,
        [
            &BroadcasterId::from("12345"),
            &ModeratorId::from("98765"),
            false,
        ]
    );

    api_test!(
        get_shield_mode_status,
        [&BroadcasterId::from("12345"), &ModeratorId::from("98765"),]
    );

    api_test!(
        warn_chat_user,
        [
            &BroadcasterId::from("404040"),
            &ModeratorId::from("404041"),
            WarnChatUser::new(&UserId::from("9876"), "stop doing that!"),
        ]
    );

    #[tokio::test]
    async fn ban_user2() {
        let suite = TwitchApiTest::new().await;

        suite.ban_user2().await;

        let _ = suite
            .execute(|api| {
                api.ban_user(
                    &BroadcasterId::from("1234"),
                    &ModeratorId::from("5678"),
                    BanUserRequest::new(&UserId::from("9876"))
                        .duration(300)
                        .reason("no reason"),
                )
            })
            .json()
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn ban_user_error() {
        let suite = TwitchApiTest::new().await;

        suite.ban_user_error().await;

        let respnose = suite
            .execute(|api| {
                api.ban_user(
                    &BroadcasterId::from("1234"),
                    &ModeratorId::from("5678"),
                    BanUserRequest::new(&UserId::from("9876"))
                        .duration(300)
                        .reason("no reason"),
                )
            })
            .json()
            .await;

        assert!(respnose.is_err());
    }

    #[tokio::test]
    async fn unban_user_error() {
        let suite = TwitchApiTest::new().await;

        suite.unban_user_error().await;

        let respnose = suite
            .execute(|api| {
                api.unban_user(
                    &BroadcasterId::from("1234"),
                    &ModeratorId::from("5678"),
                    &UserId::from("5432"),
                )
            })
            .json()
            .await;

        assert!(respnose.is_err());
    }

    #[tokio::test]
    async fn add_blocked_term2() {
        let suite = TwitchApiTest::new().await;

        suite.add_blocked_term2().await;

        suite
            .execute(|api| {
                api.add_blocked_term(
                    &BroadcasterId::from("1234"),
                    &ModeratorId::from("5678"),
                    "crac*",
                )
            })
            .json()
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn delete_chat_messages2() {
        let suite = TwitchApiTest::new().await;

        suite.delete_chat_messages2().await;

        suite
            .execute(|api| {
                api.delete_chat_messages(
                    &BroadcasterId::from("11111"),
                    &ModeratorId::from("44444"),
                    Some("abc-123-def"),
                )
            })
            .json()
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn get_vips2() {
        let suite = TwitchApiTest::new().await;

        suite.get_vips2().await;

        suite
            .execute(|api| {
                api.get_vips(
                    &BroadcasterId::from("123"),
                    Some(&[UserId::from("456"), UserId::from("678")]),
                    None,
                )
            })
            .json()
            .await
            .unwrap();
    }
}
