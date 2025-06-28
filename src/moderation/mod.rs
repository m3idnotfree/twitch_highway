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
    request::{EmptyBody, EndpointType, TwitchAPIRequest},
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

#[cfg_attr(docsrs, doc(cfg(feature = "moderation")))]
pub trait ModerationAPI {
    /// <https://dev.twitch.tv/docs/api/reference/#check-automod-status>
    fn check_automod_status(
        &self,
        broadcaster_id: BroadcasterId,
        data: &[CheckAutoMod],
    ) -> TwitchAPIRequest<CheckAutoModStatusResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#manage-held-automod-messages>
    fn manage_held_automod_messages(
        &self,
        user_id: UserId,
        msg_id: &str,
        action: AutoModAction,
    ) -> TwitchAPIRequest<EmptyBody>;
    /// <https://dev.twitch.tv/docs/api/reference/#get-automod-settings>
    fn get_auto_mod_settings(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
    ) -> TwitchAPIRequest<AutoModSettingsResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#update-automod-settings>
    fn update_auto_mod_settings(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        opts: Option<UpdateAutoModSettingsRequest>,
    ) -> TwitchAPIRequest<AutoModSettingsResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#get-banned-users>
    fn get_banned_users(
        &self,
        broadcaster_id: BroadcasterId,
        user_id: Option<&[UserId]>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<GetBannedUsersResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#ban-user>
    fn ban_users(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        data: BanUserRequest,
    ) -> TwitchAPIRequest<BanUsersResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#unban-user>
    fn unban_user(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        user_id: UserId,
    ) -> TwitchAPIRequest<EmptyBody>;
    /// <https://dev.twitch.tv/docs/api/reference/#get-unban-requests>
    fn get_unban_requests(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        status: UnbanRequestStatus,
        user_id: Option<UserId>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<UnbanRequestResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#resolve-unban-requests>
    fn resolve_unban_requests(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        unban_request_id: &str,
        status: UnbanRequestStatus,
        resolution_text: Option<&str>,
    ) -> TwitchAPIRequest<UnbanRequestResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#get-blocked-terms>
    fn get_blocked_terms(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<BlockedTermsResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#add-blocked-term>
    fn add_blocked_term(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        text: &str,
    ) -> TwitchAPIRequest<BlockedTermsResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#remove-blocked-term>
    fn remove_blocked_term(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        id: Id,
    ) -> TwitchAPIRequest<EmptyBody>;
    /// <https://dev.twitch.tv/docs/api/reference/#delete-chat-messages>
    fn delete_chat_messages(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        message_id: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody>;
    /// <https://dev.twitch.tv/docs/api/reference/#get-moderated-channels>
    fn get_moderated_channels(
        &self,
        user_id: UserId,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<ModeratedChannelResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#get-moderators>
    fn get_moderators(
        &self,
        broadcaster_id: BroadcasterId,
        user_id: Option<UserId>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<ModeratorsResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#add-channel-moderator>
    fn add_channel_moderator(
        &self,
        broadcaster_id: BroadcasterId,
        user_id: UserId,
    ) -> TwitchAPIRequest<EmptyBody>;
    /// <https://dev.twitch.tv/docs/api/reference/#remove-channel-moderator>
    fn remove_channel_moderator(
        &self,
        broadcaster_id: BroadcasterId,
        user_id: UserId,
    ) -> TwitchAPIRequest<EmptyBody>;
    /// <https://dev.twitch.tv/docs/api/reference/#get-vips>
    fn get_vips(
        &self,
        user_ids: Option<&[UserId]>,
        broadcaster_id: BroadcasterId,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<ModeratorsResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#add-channel-vip>
    fn add_channel_vip(
        &self,
        user_id: UserId,
        broadcaster_id: BroadcasterId,
    ) -> TwitchAPIRequest<EmptyBody>;
    /// <https://dev.twitch.tv/docs/api/reference/#remove-channel-vip>
    fn remove_channel_vip(
        &self,
        user_id: UserId,
        broadcaster_id: BroadcasterId,
    ) -> TwitchAPIRequest<EmptyBody>;
    /// <https://dev.twitch.tv/docs/api/reference/#update-shield-mode-status>
    fn update_shield_mode_status(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        is_active: bool,
    ) -> TwitchAPIRequest<ShieldModeStatusResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#get-shield-mode-status>
    fn get_shield_mode_status(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
    ) -> TwitchAPIRequest<ShieldModeStatusResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#warn-chat-user>
    fn warm_chat_user(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        data: &[WarnChatUser],
    ) -> TwitchAPIRequest<WarnChatUsersResponse>;
}

impl ModerationAPI for TwitchAPI {
    fn check_automod_status(
        &self,
        broadcaster_id: BroadcasterId,
        data: &[CheckAutoMod],
    ) -> TwitchAPIRequest<CheckAutoModStatusResponse> {
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
            CheckAutoModStatusRequest::new(data).to_json(),
        )
    }
    fn manage_held_automod_messages(
        &self,
        user_id: UserId,
        msg_id: &str,
        action: AutoModAction,
    ) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path([MODERATION, "automod", "message"]);

        let mut headers = self.build_headers();
        headers.json();

        TwitchAPIRequest::new(
            EndpointType::ManageHeldAutoModMessages,
            url.build(),
            Method::POST,
            headers.build(),
            ManageHeldAutoModMeussageRequest::new(user_id, msg_id, action).to_json(),
        )
    }
    fn get_auto_mod_settings(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
    ) -> TwitchAPIRequest<AutoModSettingsResponse> {
        let mut url = self.build_url();
        url.path([MODERATION, "automod", SETTINGS])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(MODERATOR_ID, moderator_id);

        TwitchAPIRequest::new(
            EndpointType::GetAutoModSettings,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            None,
        )
    }
    fn update_auto_mod_settings(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        opts: Option<UpdateAutoModSettingsRequest>,
    ) -> TwitchAPIRequest<AutoModSettingsResponse> {
        let mut url = self.build_url();
        url.path([MODERATION, "automod", SETTINGS])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(MODERATOR_ID, moderator_id);

        let mut headers = self.build_headers();
        headers.json();

        let opts = if let Some(opts) = opts {
            opts.to_json()
        } else {
            None
        };

        TwitchAPIRequest::new(
            EndpointType::UpdateAutoModSettings,
            url.build(),
            Method::PUT,
            headers.build(),
            opts,
        )
    }
    fn get_banned_users(
        &self,
        broadcaster_id: BroadcasterId,
        user_id: Option<&[UserId]>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<GetBannedUsersResponse> {
        let mut url = self.build_url();
        url.path([MODERATION, "banned"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query_opt_extend(user_id.map(|ids| ids.into_iter().map(|id| (USER_ID, id))));
        if let Some(pagination) = pagination {
            pagination.apply_to_url(&mut url);
        }

        TwitchAPIRequest::new(
            EndpointType::GetBannedUsers,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            None,
        )
    }
    fn ban_users(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        data: BanUserRequest,
    ) -> TwitchAPIRequest<BanUsersResponse> {
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
            BanUsersRequest::new(data).to_json(),
        )
    }
    fn unban_user(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        user_id: UserId,
    ) -> TwitchAPIRequest<EmptyBody> {
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
            None,
        )
    }
    fn get_unban_requests(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        status: UnbanRequestStatus,
        user_id: Option<UserId>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<UnbanRequestResponse> {
        let mut url = self.build_url();
        url.path([MODERATION, "unban_requests"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(MODERATOR_ID, moderator_id)
            .query("status", status)
            .query_opt(USER_ID, user_id);
        if let Some(pagination) = pagination {
            pagination.apply_to_url(&mut url);
        }

        TwitchAPIRequest::new(
            EndpointType::GetUnbanRequests,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            None,
        )
    }
    fn resolve_unban_requests(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        unban_request_id: &str,
        status: UnbanRequestStatus,
        resolution_text: Option<&str>,
    ) -> TwitchAPIRequest<UnbanRequestResponse> {
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
            None,
        )
    }
    fn get_blocked_terms(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<BlockedTermsResponse> {
        let mut url = self.build_url();
        url.path([MODERATION, "blocked_terms"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(MODERATOR_ID, moderator_id);
        if let Some(pagination) = pagination {
            pagination.apply_to_url(&mut url);
        }

        TwitchAPIRequest::new(
            EndpointType::GetBlockedTerms,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            None,
        )
    }
    fn add_blocked_term(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        text: &str,
    ) -> TwitchAPIRequest<BlockedTermsResponse> {
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
            AddBlockedTermRequest::new(text).to_json(),
        )
    }
    fn remove_blocked_term(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        id: Id,
    ) -> TwitchAPIRequest<EmptyBody> {
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
            None,
        )
    }
    fn delete_chat_messages(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        message_id: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody> {
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
            None,
        )
    }
    fn get_moderated_channels(
        &self,
        user_id: UserId,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<ModeratedChannelResponse> {
        let mut url = self.build_url();
        url.path([MODERATION, CHANNELS]).query(USER_ID, user_id);
        if let Some(pagination) = pagination {
            pagination.apply_to_url(&mut url);
        }

        TwitchAPIRequest::new(
            EndpointType::GetModeratedChannels,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            None,
        )
    }
    fn get_moderators(
        &self,
        broadcaster_id: BroadcasterId,
        user_id: Option<UserId>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<ModeratorsResponse> {
        let mut url = self.build_url();
        url.path([MODERATION, "moderators"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query_opt(USER_ID, user_id);
        if let Some(pagination) = pagination {
            pagination.apply_to_url(&mut url);
        }

        TwitchAPIRequest::new(
            EndpointType::GetModerators,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            None,
        )
    }
    fn add_channel_moderator(
        &self,
        broadcaster_id: BroadcasterId,
        user_id: UserId,
    ) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path([MODERATION, "moderators"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(USER_ID, user_id);

        TwitchAPIRequest::new(
            EndpointType::AddChannelModerator,
            url.build(),
            Method::POST,
            self.build_headers().build(),
            None,
        )
    }
    fn remove_channel_moderator(
        &self,
        broadcaster_id: BroadcasterId,
        user_id: UserId,
    ) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path([MODERATION, "moderators"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(USER_ID, user_id);

        TwitchAPIRequest::new(
            EndpointType::RemoveChannelModerator,
            url.build(),
            Method::DELETE,
            self.build_headers().build(),
            None,
        )
    }
    fn get_vips(
        &self,
        user_ids: Option<&[UserId]>,
        broadcaster_id: BroadcasterId,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<ModeratorsResponse> {
        let mut url = self.build_url();
        url.path([CHANNELS, "vips"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query_opt_extend(user_ids.map(|ids| ids.into_iter().map(|id| (USER_ID, id))));
        if let Some(pagination) = pagination {
            pagination.apply_to_url(&mut url);
        }

        TwitchAPIRequest::new(
            EndpointType::GetVIPs,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            None,
        )
    }
    fn add_channel_vip(
        &self,
        user_id: UserId,
        broadcaster_id: BroadcasterId,
    ) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path([CHANNELS, "vips"])
            .query(USER_ID, user_id)
            .query(BROADCASTER_ID, broadcaster_id);

        TwitchAPIRequest::new(
            EndpointType::AddChannelVIP,
            url.build(),
            Method::POST,
            self.build_headers().build(),
            None,
        )
    }
    fn remove_channel_vip(
        &self,
        user_id: UserId,
        broadcaster_id: BroadcasterId,
    ) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path([CHANNELS, "vips"])
            .query(USER_ID, user_id)
            .query(BROADCASTER_ID, broadcaster_id);

        TwitchAPIRequest::new(
            EndpointType::RemoveChannelVIP,
            url.build(),
            Method::DELETE,
            self.build_headers().build(),
            None,
        )
    }
    fn update_shield_mode_status(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        is_active: bool,
    ) -> TwitchAPIRequest<ShieldModeStatusResponse> {
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
            UpdateShieldModeStatusRequest::new(is_active).to_json(),
        )
    }
    fn get_shield_mode_status(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
    ) -> TwitchAPIRequest<ShieldModeStatusResponse> {
        let mut url = self.build_url();
        url.path([MODERATION, "shield_mode"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(MODERATOR_ID, moderator_id);

        TwitchAPIRequest::new(
            EndpointType::GetShieldModeStatus,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            None,
        )
    }
    fn warm_chat_user(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        data: &[WarnChatUser],
    ) -> TwitchAPIRequest<WarnChatUsersResponse> {
        let mut url = self.build_url();
        url.path([MODERATION, "warnings"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(MODERATOR_ID, moderator_id);

        TwitchAPIRequest::new(
            EndpointType::WarnChatUser,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            WarnChatUserRequest::new(data).to_json(),
        )
    }
}
