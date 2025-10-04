mod request;
mod response;
mod types;

pub use request::{
    AddBlockedTermRequest, AutoModAction, BanUserRequest, BanUserRequestWrapper, CheckAutoMod,
    CheckAutoModStatusRequest, ManageHeldAutoModMeussageRequest, UpdateAutoModSettingsRequest,
    UpdateShieldModeStatusRequest, WarnChatUserBody, WarnChatUserRequest,
};
pub use response::{
    AutoModSettingsResponse, BanUsersResponse, BlockedTermsResponse, CheckAutoModStatusResponse,
    GetBannedUsersResponse, ModeratedChannelResponse, ModeratorsResponse, ShieldModeStatusResponse,
    UnbanRequestResponse, WarnChatUsersResponse,
};
pub use types::{
    AutoModSetting, AutoModStatus, BanUser, BannedUser, BlockedTerm, ModeratedChannel, Moderator,
    ShieldModeStatus, UnbanRequest, UnbanRequestStatus, WarnChatUser,
};

use crate::{
    request::NoContent,
    types::{
        constants::{BROADCASTER_ID, CHANNELS, CHAT, ID, MODERATOR_ID, SETTINGS, USER_ID},
        BroadcasterId, Id, ModeratorId, PaginationQuery, UserId,
    },
};

const MODERATION: &str = "moderation";
const AUTOMOD: &str = "automod";
const MODERATORS: &str = "moderators";
const BLOCKED_TERMS: &str = "blocked_terms";
const VIPS: &str = "vips";
const SHIELD_MODE: &str = "shield_mode";

endpoints! {
    ModerationAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#check-automod-status>
        fn check_automod_status(
            &self,
            broadcaster_id: &BroadcasterId,
            data: &[CheckAutoMod],
        ) -> CheckAutoModStatusResponse {
            endpoint_type: CheckAutoModStatus,
            method: POST,
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
            endpoint_type: ManageHeldAutoModMessages,
            method: POST,
            path: [MODERATION,AUTOMOD, "message"],
            headers: [json],
            body:ManageHeldAutoModMeussageRequest::new(user_id, msg_id, action).into_json()
        }

        /// <https://dev.twitch.tv/docs/api/reference/#get-automod-settings>
        fn get_automod_settings(
            &self,
            broadcaster_id: &BroadcasterId,
            moderator_id: &ModeratorId,
        ) -> AutoModSettingsResponse {
            endpoint_type: GetAutoModSettings,
            method: GET,
            path: [MODERATION, AUTOMOD, SETTINGS],
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
            endpoint_type: UpdateAutoModSettings,
            method: PUT,
            path: [MODERATION, AUTOMOD, SETTINGS],
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
            endpoint_type: GetBannedUsers,
            method: GET,
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
            endpoint_type: BanUsers,
            method: POST,
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
            endpoint_type: BanUsers,
            method: DELETE,
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
            endpoint_type: GetUnbanRequests,
            method: GET,
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
            endpoint_type: ResolveUnbanRequests,
            method: PATCH,
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
            endpoint_type: GetBlockedTerms,
            method: GET,
            path: [MODERATION, BLOCKED_TERMS],
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
            endpoint_type: AddBlockedTerm,
            method: POST,
            path: [MODERATION, BLOCKED_TERMS],
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
            endpoint_type: RemoveBlockedTerm,
            method: DELETE,
            path: [MODERATION, BLOCKED_TERMS],
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
            endpoint_type: DeleteChatMessages,
            method: DELETE,
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
            endpoint_type: GetModeratedChannels,
            method: GET,
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
            endpoint_type: GetModerators,
            method: GET,
            path: [MODERATION, MODERATORS],
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
            endpoint_type: AddChannelModerator,
            method: POST,
            path: [MODERATION, MODERATORS],
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
            endpoint_type: RemoveChannelModerator,
            method: DELETE,
            path: [MODERATION, MODERATORS],
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
            endpoint_type: GetVIPs,
            method: GET,
            path: [CHANNELS, VIPS],
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
            endpoint_type: AddChannelVIP,
            method: POST,
            path: [CHANNELS, VIPS],
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
            endpoint_type: RemoveChannelVIP,
            method: DELETE,
            path: [CHANNELS, VIPS],
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
            endpoint_type: UpdateShieldModeStatus,
            method: PUT,
            path: [MODERATION, SHIELD_MODE],
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
            endpoint_type: GetShieldModeStatus,
            method: GET,
            path: [MODERATION, SHIELD_MODE],
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
            data: WarnChatUserRequest,
        ) -> WarnChatUsersResponse {
            endpoint_type: WarnChatUser,
            method: POST,
            path: [MODERATION, "warnings"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(MODERATOR_ID, moderator_id)
            },
            headers: [json],
            body: WarnChatUserBody::new(data).into_json()
        }
    }
}
