mod builder;
mod response;
mod types;

pub use builder::{
    CreateBan, DeleteChatMessages, GetBannedUsers, GetBlockedTerms, GetModeratedChannels,
    GetModerators, GetUnbanRequests, GetVips, ResolveUnbanRequest, UpdateAutomodSettings,
};

pub use response::{
    AutoModSettingsResponse, BanUsersResponse, BlockedTermsResponse, CheckAutoModStatusResponse,
    GetBannedUsersResponse, ModeratedChannelResponse, ModeratorsResponse, ShieldModeStatusResponse,
    SuspiciousResponse, UnbanRequestResponse, WarnChatUsersResponse,
};
pub use types::{
    AutoModAction, AutoModSetting, AutoModStatus, BanUser, BannedUser, BlockedTerm, CheckAutoMod,
    ModeratedChannel, Moderator, ShieldModeStatus, SuspiciousStatus, SuspiciousType,
    SuspiciousUser, UnbanRequest, UnbanRequestStatus, WarnChatUser,
};

use types::{
    AddBlockedTermBody, CheckAutomodStatusBody, ManageHeldAutomodMessagesBody, SuspiciousBody,
    UpdateShieldModeStatusBody, WarnChatUserBody, WarnChatUserBodyWrapper,
};

use std::future::Future;

use crate::{
    Client, Error,
    types::{
        BlockedTermId, BroadcasterId, ModeratorId, UserId,
        constants::{
            AUTOMOD, BANS, BLOCKED_TERMS, BROADCASTER_ID, CHANNELS, ENFORCEMENTS, ID, MESSAGE,
            MODERATION, MODERATOR_ID, MODERATORS, SETTINGS, SHIELD_MODE, STATUS, USER_ID, VIPS,
            WARNINGS,
        },
    },
};

const SUSPICIOUS_USERS: &str = "suspicious_users";

pub trait ModerationAPI {
    /// See <https://dev.twitch.tv/docs/api/reference/#check-automod-status>
    fn check_automod_status(
        &self,
        broadcaster_id: &BroadcasterId,
        data: &[CheckAutoMod],
    ) -> impl Future<Output = Result<CheckAutoModStatusResponse, Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#manage-held-automod-messages>
    fn manage_held_automod_messages(
        &self,
        user_id: &UserId,
        msg_id: &str,
        action: AutoModAction,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#get-automod-settings>
    fn get_automod_settings(
        &self,
        broadcaster_id: &BroadcasterId,
        moderator_id: &ModeratorId,
    ) -> impl Future<Output = Result<AutoModSettingsResponse, Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#update-automod-settings>
    fn update_automod_settings<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
    ) -> UpdateAutomodSettings<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#get-banned-users>
    fn get_banned_users<'a>(&'a self, broadcaster_id: &'a BroadcasterId) -> GetBannedUsers<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#ban-user>
    fn ban_user<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
        user_id: &'a UserId,
    ) -> CreateBan<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#unban-user>
    fn unban_user(
        &self,
        broadcaster_id: &BroadcasterId,
        moderator_id: &ModeratorId,
        user_id: &UserId,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#get-unban-requests>
    fn get_unban_requests<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
        status: UnbanRequestStatus,
    ) -> GetUnbanRequests<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#resolve-unban-requests>
    fn resolve_unban_request<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
        unban_request_id: &'a str,
        status: UnbanRequestStatus,
    ) -> ResolveUnbanRequest<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#get-blocked-terms>
    fn get_blocked_terms<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
    ) -> GetBlockedTerms<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#add-blocked-term>
    fn add_blocked_term(
        &self,
        broadcaster_id: &BroadcasterId,
        moderator_id: &ModeratorId,
        text: &str,
    ) -> impl Future<Output = Result<BlockedTermsResponse, Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#remove-blocked-term>
    fn remove_blocked_term(
        &self,
        broadcaster_id: &BroadcasterId,
        moderator_id: &ModeratorId,
        id: &BlockedTermId,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#delete-chat-messages>
    fn delete_chat_messages<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
    ) -> DeleteChatMessages<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#get-moderated-channels>
    fn get_moderated_channels<'a>(&'a self, user_id: &'a UserId) -> GetModeratedChannels<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#get-moderators>
    fn get_moderators<'a>(&'a self, broadcaster_id: &'a BroadcasterId) -> GetModerators<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#add-channel-moderator>
    fn add_channel_moderator(
        &self,
        broadcaster_id: &BroadcasterId,
        user_id: &UserId,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#remove-channel-moderator>
    fn remove_channel_moderator(
        &self,
        broadcaster_id: &BroadcasterId,
        user_id: &UserId,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#get-vips>
    fn get_vips<'a>(&'a self, broadcaster_id: &'a BroadcasterId) -> GetVips<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#add-channel-vip>
    fn add_channel_vip(
        &self,
        broadcaster_id: &BroadcasterId,
        user_id: &UserId,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#remove-channel-vip>
    fn remove_channel_vip(
        &self,
        broadcaster_id: &BroadcasterId,
        user_id: &UserId,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#update-shield-mode-status>
    fn update_shield_mode_status(
        &self,
        broadcaster_id: &BroadcasterId,
        moderator_id: &ModeratorId,
        is_active: bool,
    ) -> impl Future<Output = Result<ShieldModeStatusResponse, Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#get-shield-mode-status>
    fn get_shield_mode_status(
        &self,
        broadcaster_id: &BroadcasterId,
        moderator_id: &ModeratorId,
    ) -> impl Future<Output = Result<ShieldModeStatusResponse, Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#warn-chat-user>
    fn warn_chat_user(
        &self,
        broadcaster_id: &BroadcasterId,
        moderator_id: &ModeratorId,
        user_id: &UserId,
        reason: &str,
    ) -> impl Future<Output = Result<WarnChatUsersResponse, Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#add-suspicious-status-to-chat-user>
    fn add_suspicious_status_to_chat_user(
        &self,
        broadcaster_id: &BroadcasterId,
        moderator_id: &ModeratorId,
        user_id: &UserId,
        status: SuspiciousStatus,
    ) -> impl Future<Output = Result<SuspiciousResponse, Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#remove-suspicious-status-from-chat-user>
    fn remove_suspicious_status_from_chat_user(
        &self,
        broadcaster_id: &BroadcasterId,
        moderator_id: &ModeratorId,
        user_id: &UserId,
    ) -> impl Future<Output = Result<SuspiciousResponse, Error>> + Send;
}

impl ModerationAPI for Client {
    async fn check_automod_status(
        &self,
        broadcaster_id: &BroadcasterId,
        data: &[CheckAutoMod],
    ) -> Result<CheckAutoModStatusResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([MODERATION, ENFORCEMENTS, STATUS]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id);

        let req = self
            .http_client()
            .post(url)
            .json(&CheckAutomodStatusBody { data });
        self.json(req).await
    }

    async fn manage_held_automod_messages(
        &self,
        user_id: &UserId,
        msg_id: &str,
        action: AutoModAction,
    ) -> Result<(), Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([MODERATION, AUTOMOD, MESSAGE]);

        let req = self
            .http_client()
            .post(url)
            .json(&ManageHeldAutomodMessagesBody {
                user_id,
                msg_id,
                action,
            });
        self.no_content(req).await
    }

    async fn get_automod_settings(
        &self,
        broadcaster_id: &BroadcasterId,
        moderator_id: &ModeratorId,
    ) -> Result<AutoModSettingsResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([MODERATION, AUTOMOD, SETTINGS]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id)
            .append_pair(MODERATOR_ID, moderator_id);

        let req = self.http_client().get(url);
        self.json(req).await
    }

    fn update_automod_settings<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
    ) -> UpdateAutomodSettings<'a> {
        UpdateAutomodSettings::new(self, broadcaster_id, moderator_id)
    }

    fn get_banned_users<'a>(&'a self, broadcaster_id: &'a BroadcasterId) -> GetBannedUsers<'a> {
        GetBannedUsers::new(self, broadcaster_id)
    }

    fn ban_user<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
        user_id: &'a UserId,
    ) -> CreateBan<'a> {
        CreateBan::new(self, broadcaster_id, moderator_id, user_id)
    }

    async fn unban_user(
        &self,
        broadcaster_id: &BroadcasterId,
        moderator_id: &ModeratorId,
        user_id: &UserId,
    ) -> Result<(), Error> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().extend([MODERATION, BANS]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id)
            .append_pair(MODERATOR_ID, moderator_id)
            .append_pair(USER_ID, user_id);

        let req = self.http_client().delete(url);
        self.no_content(req).await
    }

    fn get_unban_requests<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
        status: UnbanRequestStatus,
    ) -> GetUnbanRequests<'a> {
        GetUnbanRequests::new(self, broadcaster_id, moderator_id, status)
    }

    fn resolve_unban_request<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
        unban_request_id: &'a str,
        status: UnbanRequestStatus,
    ) -> ResolveUnbanRequest<'a> {
        ResolveUnbanRequest::new(self, broadcaster_id, moderator_id, unban_request_id, status)
    }

    fn get_blocked_terms<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
    ) -> GetBlockedTerms<'a> {
        GetBlockedTerms::new(self, broadcaster_id, moderator_id)
    }

    async fn add_blocked_term(
        &self,
        broadcaster_id: &BroadcasterId,
        moderator_id: &ModeratorId,
        text: &str,
    ) -> Result<BlockedTermsResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([MODERATION, BLOCKED_TERMS]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id)
            .append_pair(MODERATOR_ID, moderator_id);

        let req = self
            .http_client()
            .post(url)
            .json(&AddBlockedTermBody { text });
        self.json(req).await
    }

    async fn remove_blocked_term(
        &self,
        broadcaster_id: &BroadcasterId,
        moderator_id: &ModeratorId,
        id: &BlockedTermId,
    ) -> Result<(), Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([MODERATION, BLOCKED_TERMS]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id)
            .append_pair(MODERATOR_ID, moderator_id)
            .append_pair(ID, id);

        let req = self.http_client().delete(url);
        self.no_content(req).await
    }

    fn delete_chat_messages<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
    ) -> DeleteChatMessages<'a> {
        DeleteChatMessages::new(self, broadcaster_id, moderator_id)
    }

    fn get_moderated_channels<'a>(&'a self, user_id: &'a UserId) -> GetModeratedChannels<'a> {
        GetModeratedChannels::new(self, user_id)
    }

    fn get_moderators<'a>(&'a self, broadcaster_id: &'a BroadcasterId) -> GetModerators<'a> {
        GetModerators::new(self, broadcaster_id)
    }

    async fn add_channel_moderator(
        &self,
        broadcaster_id: &BroadcasterId,
        user_id: &UserId,
    ) -> Result<(), Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([MODERATION, MODERATORS]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id)
            .append_pair(USER_ID, user_id);

        self.no_content(self.http_client().post(url)).await
    }

    async fn remove_channel_moderator(
        &self,
        broadcaster_id: &BroadcasterId,
        user_id: &UserId,
    ) -> Result<(), Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([MODERATION, MODERATORS]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id)
            .append_pair(USER_ID, user_id);

        self.no_content(self.http_client().delete(url)).await
    }

    fn get_vips<'a>(&'a self, broadcaster_id: &'a BroadcasterId) -> GetVips<'a> {
        GetVips::new(self, broadcaster_id)
    }

    async fn add_channel_vip(
        &self,
        broadcaster_id: &BroadcasterId,
        user_id: &UserId,
    ) -> Result<(), Error> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().extend(&[CHANNELS, VIPS]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id)
            .append_pair(USER_ID, user_id);

        let req = self.http_client().post(url);
        self.execute(req).await?;
        Ok(())
    }

    async fn remove_channel_vip(
        &self,
        broadcaster_id: &BroadcasterId,
        user_id: &UserId,
    ) -> Result<(), Error> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().extend([CHANNELS, VIPS]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id)
            .append_pair(USER_ID, user_id);

        self.no_content(self.http_client().delete(url)).await
    }

    async fn update_shield_mode_status(
        &self,
        broadcaster_id: &BroadcasterId,
        moderator_id: &ModeratorId,
        is_active: bool,
    ) -> Result<ShieldModeStatusResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([MODERATION, SHIELD_MODE]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id)
            .append_pair(MODERATOR_ID, moderator_id);

        let req = self
            .http_client()
            .put(url)
            .json(&UpdateShieldModeStatusBody { is_active });
        self.json(req).await
    }

    async fn get_shield_mode_status(
        &self,
        broadcaster_id: &BroadcasterId,
        moderator_id: &ModeratorId,
    ) -> Result<ShieldModeStatusResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([MODERATION, SHIELD_MODE]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id)
            .append_pair(MODERATOR_ID, moderator_id);

        self.json(self.http_client().get(url)).await
    }

    async fn warn_chat_user(
        &self,
        broadcaster_id: &BroadcasterId,
        moderator_id: &ModeratorId,
        user_id: &UserId,
        reason: &str,
    ) -> Result<WarnChatUsersResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([MODERATION, WARNINGS]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id)
            .append_pair(MODERATOR_ID, moderator_id);

        let req = self.http_client().post(url).json(&WarnChatUserBodyWrapper {
            data: WarnChatUserBody { user_id, reason },
        });
        self.json(req).await
    }

    async fn add_suspicious_status_to_chat_user(
        &self,
        broadcaster_id: &BroadcasterId,
        moderator_id: &ModeratorId,
        user_id: &UserId,
        status: SuspiciousStatus,
    ) -> Result<SuspiciousResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([MODERATION, SUSPICIOUS_USERS]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id)
            .append_pair(MODERATOR_ID, moderator_id);

        let req = self
            .http_client()
            .post(url)
            .json(&SuspiciousBody { user_id, status });
        self.json(req).await
    }

    async fn remove_suspicious_status_from_chat_user(
        &self,
        broadcaster_id: &BroadcasterId,
        moderator_id: &ModeratorId,
        user_id: &UserId,
    ) -> Result<SuspiciousResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([MODERATION, SUSPICIOUS_USERS]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id)
            .append_pair(MODERATOR_ID, moderator_id)
            .append_pair(USER_ID, user_id);

        self.json(self.http_client().delete(url)).await
    }
}
