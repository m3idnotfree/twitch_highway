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
    base::TwitchAPIBase,
    types::{
        constants::{BROADCASTER_ID, CHANNELS, CHAT, ID, MODERATOR_ID, SETTINGS, USER_ID},
        BroadcasterId, Id, ModeratorId, PaginationQuery, UserId,
    },
    EmptyBody, EndpointType, TwitchAPI, TwitchAPIRequest,
};

pub mod request;
pub mod response;
pub mod types;

const MODERATION: &str = "moderation";

pub trait ModerationAPI: TwitchAPIBase {
    /// <https://dev.twitch.tv/docs/api/reference/#check-automod-status>
    fn check_automod_status(
        &self,
        broadcaster_id: BroadcasterId,
        data: Vec<CheckAutoMod>,
    ) -> TwitchAPIRequest<CheckAutoModStatusRequest, CheckAutoModStatusResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#manage-held-automod-messages>
    fn manage_held_automod_messages(
        &self,
        user_id: UserId,
        msg_id: &str,
        action: AutoModAction,
    ) -> TwitchAPIRequest<ManageHeldAutoModMeussageRequest, EmptyBody>;
    /// <https://dev.twitch.tv/docs/api/reference/#get-automod-settings>
    fn get_auto_mod_settings(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
    ) -> TwitchAPIRequest<EmptyBody, AutoModSettingsResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#update-automod-settings>
    fn update_auto_mod_settings(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        opts: Option<UpdateAutoModSettingsRequest>,
    ) -> TwitchAPIRequest<UpdateAutoModSettingsRequest, AutoModSettingsResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#get-banned-users>
    fn get_banned_users(
        &self,
        broadcaster_id: BroadcasterId,
        user_id: Option<Vec<UserId>>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, GetBannedUsersResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#ban-user>
    fn ban_users(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        data: BanUserRequest,
    ) -> TwitchAPIRequest<BanUsersRequest, BanUsersResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#unban-user>
    fn unban_user(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        user_id: UserId,
    ) -> TwitchAPIRequest<EmptyBody, EmptyBody>;
    /// <https://dev.twitch.tv/docs/api/reference/#get-unban-requests>
    fn get_unban_requests(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        status: UnbanRequestStatus,
        user_id: Option<UserId>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, UnbanRequestResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#resolve-unban-requests>
    fn resolve_unban_requests(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        unban_request_id: &str,
        status: UnbanRequestStatus,
        resolution_text: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody, UnbanRequestResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#get-blocked-terms>
    fn get_blocked_terms(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, BlockedTermsResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#add-blocked-term>
    fn add_blocked_term(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        text: &str,
    ) -> TwitchAPIRequest<AddBlockedTermRequest, BlockedTermsResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#remove-blocked-term>
    fn remove_blocked_term(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        id: Id,
    ) -> TwitchAPIRequest<EmptyBody, EmptyBody>;
    /// <https://dev.twitch.tv/docs/api/reference/#delete-chat-messages>
    fn delete_chat_messages(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        message_id: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody, EmptyBody>;
    /// <https://dev.twitch.tv/docs/api/reference/#get-moderated-channels>
    fn get_moderated_channels(
        &self,
        user_id: UserId,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, ModeratedChannelResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#get-moderators>
    fn get_moderators(
        &self,
        broadcaster_id: BroadcasterId,
        user_id: Option<UserId>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, ModeratorsResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#add-channel-moderator>
    fn add_channel_moderator(
        &self,
        broadcaster_id: BroadcasterId,
        user_id: UserId,
    ) -> TwitchAPIRequest<EmptyBody, EmptyBody>;
    /// <https://dev.twitch.tv/docs/api/reference/#remove-channel-moderator>
    fn remove_channel_moderator(
        &self,
        broadcaster_id: BroadcasterId,
        user_id: UserId,
    ) -> TwitchAPIRequest<EmptyBody, EmptyBody>;
    /// <https://dev.twitch.tv/docs/api/reference/#get-vips>
    fn get_vips(
        &self,
        user_ids: Option<Vec<UserId>>,
        broadcaster_id: BroadcasterId,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, ModeratorsResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#add-channel-vip>
    fn add_channel_vip(
        &self,
        user_id: UserId,
        broadcaster_id: BroadcasterId,
    ) -> TwitchAPIRequest<EmptyBody, EmptyBody>;
    /// <https://dev.twitch.tv/docs/api/reference/#remove-channel-vip>
    fn remove_channel_vip(
        &self,
        user_id: UserId,
        broadcaster_id: BroadcasterId,
    ) -> TwitchAPIRequest<EmptyBody, EmptyBody>;
    /// <https://dev.twitch.tv/docs/api/reference/#update-shield-mode-status>
    fn update_shield_mode_status(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        is_active: bool,
    ) -> TwitchAPIRequest<UpdateShieldModeStatusRequest, ShieldModeStatusResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#get-shield-mode-status>
    fn get_shield_mode_status(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
    ) -> TwitchAPIRequest<EmptyBody, ShieldModeStatusResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#warn-chat-user>
    fn warm_chat_user(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        data: Vec<WarnChatUser>,
    ) -> TwitchAPIRequest<WarnChatUserRequest, WarnChatUsersResponse>;
}

impl ModerationAPI for TwitchAPI {
    fn check_automod_status(
        &self,
        broadcaster_id: BroadcasterId,
        data: Vec<CheckAutoMod>,
    ) -> TwitchAPIRequest<CheckAutoModStatusRequest, CheckAutoModStatusResponse> {
        let mut url = self.build_url();
        url.path([MODERATION, "enforcements", "status"])
            .query(BROADCASTER_ID, broadcaster_id);

        let mut headers = self.build_headers();
        headers.json();

        TwitchAPIRequest::new(
            EndpointType::CheckAutoModStatus,
            url.build(),
            Method::POST,
            headers.build(),
            CheckAutoModStatusRequest::new(data),
        )
    }
    fn manage_held_automod_messages(
        &self,
        user_id: UserId,
        msg_id: &str,
        action: AutoModAction,
    ) -> TwitchAPIRequest<ManageHeldAutoModMeussageRequest, EmptyBody> {
        let mut url = self.build_url();
        url.path([MODERATION, "automod", "message"]);

        let mut headers = self.build_headers();
        headers.json();

        TwitchAPIRequest::new(
            EndpointType::ManageHeldAutoModMessages,
            url.build(),
            Method::POST,
            headers.build(),
            ManageHeldAutoModMeussageRequest::new(user_id, msg_id.to_string(), action),
        )
    }
    fn get_auto_mod_settings(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
    ) -> TwitchAPIRequest<EmptyBody, AutoModSettingsResponse> {
        let mut url = self.build_url();
        url.path([MODERATION, "automod", SETTINGS])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(MODERATOR_ID, moderator_id);

        TwitchAPIRequest::new(
            EndpointType::GetAutoModSettings,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn update_auto_mod_settings(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        opts: Option<UpdateAutoModSettingsRequest>,
    ) -> TwitchAPIRequest<UpdateAutoModSettingsRequest, AutoModSettingsResponse> {
        let mut url = self.build_url();
        url.path([MODERATION, "automod", SETTINGS])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(MODERATOR_ID, moderator_id);

        let mut headers = self.build_headers();
        headers.json();

        TwitchAPIRequest::new(
            EndpointType::UpdateAutoModSettings,
            url.build(),
            Method::PUT,
            headers.build(),
            opts.unwrap_or_default(),
        )
    }
    fn get_banned_users(
        &self,
        broadcaster_id: BroadcasterId,
        user_id: Option<Vec<UserId>>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, GetBannedUsersResponse> {
        let mut url = self.build_url();
        url.path([MODERATION, "banned"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query_opt_extend(user_id.map(|ids| ids.into_iter().map(|id| (USER_ID, id))))
            .query_opt_pairs(pagination);

        TwitchAPIRequest::new(
            EndpointType::GetBannedUsers,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn ban_users(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        data: BanUserRequest,
    ) -> TwitchAPIRequest<BanUsersRequest, BanUsersResponse> {
        let mut url = self.build_url();
        url.path([MODERATION, "bans"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(MODERATOR_ID, moderator_id);

        let mut headers = self.build_headers();
        headers.json();

        TwitchAPIRequest::new(
            EndpointType::BanUsers,
            url.build(),
            Method::POST,
            headers.build(),
            BanUsersRequest::new(data),
        )
    }
    fn unban_user(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        user_id: UserId,
    ) -> TwitchAPIRequest<EmptyBody, EmptyBody> {
        let mut url = self.build_url();
        url.path([MODERATION, "bans"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(MODERATOR_ID, moderator_id)
            .query(USER_ID, user_id);

        TwitchAPIRequest::new(
            EndpointType::BanUsers,
            url.build(),
            Method::DELETE,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn get_unban_requests(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        status: UnbanRequestStatus,
        user_id: Option<UserId>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, UnbanRequestResponse> {
        let mut url = self.build_url();
        url.path([MODERATION, "unban_requests"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(MODERATOR_ID, moderator_id)
            .query("status", status)
            .query_opt(USER_ID, user_id)
            .query_opt_pairs(pagination);

        TwitchAPIRequest::new(
            EndpointType::GetUnbanRequests,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn resolve_unban_requests(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        unban_request_id: &str,
        status: UnbanRequestStatus,
        resolution_text: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody, UnbanRequestResponse> {
        let mut url = self.build_url();
        url.path([MODERATION, "unban_requests"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(MODERATOR_ID, moderator_id)
            .query("unban_request_id", unban_request_id)
            .query("status", status)
            .query_opt("resolution_text", resolution_text);

        TwitchAPIRequest::new(
            EndpointType::ResolveUnbanRequests,
            url.build(),
            Method::PATCH,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn get_blocked_terms(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, BlockedTermsResponse> {
        let mut url = self.build_url();
        url.path([MODERATION, "blocked_terms"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(MODERATOR_ID, moderator_id)
            .query_opt_pairs(pagination);

        TwitchAPIRequest::new(
            EndpointType::GetBlockedTerms,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn add_blocked_term(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        text: &str,
    ) -> TwitchAPIRequest<AddBlockedTermRequest, BlockedTermsResponse> {
        let mut url = self.build_url();
        url.path([MODERATION, "blocked_terms"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(MODERATOR_ID, moderator_id);

        let mut headers = self.build_headers();
        headers.json();

        TwitchAPIRequest::new(
            EndpointType::AddBlockedTerm,
            url.build(),
            Method::POST,
            headers.build(),
            AddBlockedTermRequest::new(text.to_string()),
        )
    }
    fn remove_blocked_term(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        id: Id,
    ) -> TwitchAPIRequest<EmptyBody, EmptyBody> {
        let mut url = self.build_url();
        url.path([MODERATION, "blocked_terms"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(MODERATOR_ID, moderator_id)
            .query(ID, id);

        TwitchAPIRequest::new(
            EndpointType::RemoveBlockedTerm,
            url.build(),
            Method::DELETE,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn delete_chat_messages(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        message_id: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody, EmptyBody> {
        let mut url = self.build_url();
        url.path([MODERATION, CHAT])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(MODERATOR_ID, moderator_id)
            .query_opt("message_id", message_id);

        TwitchAPIRequest::new(
            EndpointType::DeleteChatMessages,
            url.build(),
            Method::DELETE,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn get_moderated_channels(
        &self,
        user_id: UserId,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, ModeratedChannelResponse> {
        let mut url = self.build_url();
        url.path([MODERATION, CHANNELS])
            .query(USER_ID, user_id)
            .query_opt_pairs(pagination);

        TwitchAPIRequest::new(
            EndpointType::GetModeratedChannels,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn get_moderators(
        &self,
        broadcaster_id: BroadcasterId,
        user_id: Option<UserId>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, ModeratorsResponse> {
        let mut url = self.build_url();
        url.path([MODERATION, "moderators"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query_opt(USER_ID, user_id)
            .query_opt_pairs(pagination);

        TwitchAPIRequest::new(
            EndpointType::GetModerators,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn add_channel_moderator(
        &self,
        broadcaster_id: BroadcasterId,
        user_id: UserId,
    ) -> TwitchAPIRequest<EmptyBody, EmptyBody> {
        let mut url = self.build_url();
        url.path([MODERATION, "moderators"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(USER_ID, user_id);

        TwitchAPIRequest::new(
            EndpointType::AddChannelModerator,
            url.build(),
            Method::POST,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn remove_channel_moderator(
        &self,
        broadcaster_id: BroadcasterId,
        user_id: UserId,
    ) -> TwitchAPIRequest<EmptyBody, EmptyBody> {
        let mut url = self.build_url();
        url.path([MODERATION, "moderators"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(USER_ID, user_id);

        TwitchAPIRequest::new(
            EndpointType::RemoveChannelModerator,
            url.build(),
            Method::DELETE,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn get_vips(
        &self,
        user_ids: Option<Vec<UserId>>,
        broadcaster_id: BroadcasterId,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, ModeratorsResponse> {
        let mut url = self.build_url();
        url.path([CHANNELS, "vips"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query_opt_extend(user_ids.map(|ids| ids.into_iter().map(|id| (USER_ID, id))))
            .query_opt_pairs(pagination);

        TwitchAPIRequest::new(
            EndpointType::GetVIPs,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn add_channel_vip(
        &self,
        user_id: UserId,
        broadcaster_id: BroadcasterId,
    ) -> TwitchAPIRequest<EmptyBody, EmptyBody> {
        let mut url = self.build_url();
        url.path([CHANNELS, "vips"])
            .query(USER_ID, user_id)
            .query(BROADCASTER_ID, broadcaster_id);

        TwitchAPIRequest::new(
            EndpointType::AddChannelVIP,
            url.build(),
            Method::POST,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn remove_channel_vip(
        &self,
        user_id: UserId,
        broadcaster_id: BroadcasterId,
    ) -> TwitchAPIRequest<EmptyBody, EmptyBody> {
        let mut url = self.build_url();
        url.path([CHANNELS, "vips"])
            .query(USER_ID, user_id)
            .query(BROADCASTER_ID, broadcaster_id);

        TwitchAPIRequest::new(
            EndpointType::RemoveChannelVIP,
            url.build(),
            Method::DELETE,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn update_shield_mode_status(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        is_active: bool,
    ) -> TwitchAPIRequest<UpdateShieldModeStatusRequest, ShieldModeStatusResponse> {
        let mut url = self.build_url();
        url.path([MODERATION, "shield_mode"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(MODERATOR_ID, moderator_id);

        let mut headers = self.build_headers();
        headers.json();
        TwitchAPIRequest::new(
            EndpointType::UpdateShieldModeStatus,
            url.build(),
            Method::PUT,
            headers.build(),
            UpdateShieldModeStatusRequest::new(is_active),
        )
    }
    fn get_shield_mode_status(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
    ) -> TwitchAPIRequest<EmptyBody, ShieldModeStatusResponse> {
        let mut url = self.build_url();
        url.path([MODERATION, "shield_mode"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(MODERATOR_ID, moderator_id);

        TwitchAPIRequest::new(
            EndpointType::GetShieldModeStatus,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn warm_chat_user(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        data: Vec<WarnChatUser>,
    ) -> TwitchAPIRequest<WarnChatUserRequest, WarnChatUsersResponse> {
        let mut url = self.build_url();
        url.path([MODERATION, "warnings"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(MODERATOR_ID, moderator_id);

        TwitchAPIRequest::new(
            EndpointType::WarnChatUser,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            WarnChatUserRequest::new(data),
        )
    }
}
