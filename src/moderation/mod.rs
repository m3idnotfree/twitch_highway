use asknothingx2_util::api::Method;
use request::{
    AddBlockedTermRequest, BanUsersRequest, CheckAutoModStatusRequest,
    ManageHeldAutoModMeussageRequest, UpdateAutoModSettingsRequest, UpdateShieldModeStatusRequest,
    WarnChatUserRequest,
};

use crate::{
    base::TwitchAPIBase,
    types::{
        BroadcasterId, Id, ModeratorId, UserId, AFTER, BROADCASTER_ID, CHANNELS, FIRST, ID,
        MODERATOR_ID, USER_ID,
    },
    EmptyBody, EndpointType, TwitchAPI, TwitchAPIRequest,
};

pub mod request;
pub mod response;
pub mod types;

const MODERATION: &str = "moderation";

pub trait ModerationAPI: TwitchAPIBase {
    fn check_automod_status(
        &self,
        broadcaster_id: BroadcasterId,
        request: CheckAutoModStatusRequest,
    ) -> TwitchAPIRequest<CheckAutoModStatusRequest>;
    fn manage_held_automod_messages(
        &self,
        request: ManageHeldAutoModMeussageRequest,
    ) -> TwitchAPIRequest<ManageHeldAutoModMeussageRequest>;
    fn get_auto_mod_settings(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
    ) -> TwitchAPIRequest<EmptyBody>;
    fn update_auto_mod_settings(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        request: UpdateAutoModSettingsRequest,
    ) -> TwitchAPIRequest<UpdateAutoModSettingsRequest>;
    fn get_banned_users(
        &self,
        broadcaster_id: BroadcasterId,
        user_id: Option<Vec<UserId>>,
        first: Option<u64>,
        after: Option<String>,
        before: Option<String>,
    ) -> TwitchAPIRequest<EmptyBody>;
    fn ban_users(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        request: BanUsersRequest,
    ) -> TwitchAPIRequest<BanUsersRequest>;
    fn unban_user(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        user_id: UserId,
    ) -> TwitchAPIRequest<EmptyBody>;
    fn get_unban_requests(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        status: &str,
        user_id: Option<UserId>,
        after: Option<&str>,
        first: Option<u64>,
    ) -> TwitchAPIRequest<EmptyBody>;
    fn resolve_unban_requests(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        unban_request_id: &str,
        status: &str,
        resolution_text: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody>;
    fn get_blocked_terms(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        first: Option<u64>,
        after: Option<String>,
    ) -> TwitchAPIRequest<EmptyBody>;
    fn add_blocked_term(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        text: &str,
    ) -> TwitchAPIRequest<AddBlockedTermRequest>;
    fn remove_blocked_term(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        id: Id,
    ) -> TwitchAPIRequest<EmptyBody>;
    fn delete_chat_messages(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        message_id: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody>;
    fn get_moderated_channels(
        &self,
        user_id: UserId,
        after: Option<&str>,
        first: Option<u64>,
    ) -> TwitchAPIRequest<EmptyBody>;
    fn get_moderators(
        &self,
        broadcaster_id: BroadcasterId,
        user_id: Option<UserId>,
        first: Option<&str>,
        after: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody>;
    fn add_channel_moderator(
        &self,
        broadcaster_id: BroadcasterId,
        user_id: UserId,
    ) -> TwitchAPIRequest<EmptyBody>;
    fn remove_channel_moderator(
        &self,
        broadcaster_id: BroadcasterId,
        user_id: UserId,
    ) -> TwitchAPIRequest<EmptyBody>;
    fn get_vips(
        &self,
        user_ids: Option<Vec<UserId>>,
        broadcaster_id: BroadcasterId,
        first: Option<u64>,
        after: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody>;
    fn add_channel_vip(
        &self,
        user_id: UserId,
        broadcaster_id: BroadcasterId,
    ) -> TwitchAPIRequest<EmptyBody>;
    fn remove_channel_vip(
        &self,
        user_id: UserId,
        broadcaster_id: BroadcasterId,
    ) -> TwitchAPIRequest<EmptyBody>;
    fn update_shield_mode_status(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        is_active: bool,
    ) -> TwitchAPIRequest<UpdateShieldModeStatusRequest>;
    fn get_shield_mode_status(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
    ) -> TwitchAPIRequest<EmptyBody>;
    fn warm_chat_user(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        request: WarnChatUserRequest,
    ) -> TwitchAPIRequest<WarnChatUserRequest>;
}

impl ModerationAPI for TwitchAPI {
    fn check_automod_status(
        &self,
        broadcaster_id: BroadcasterId,
        request: CheckAutoModStatusRequest,
    ) -> TwitchAPIRequest<CheckAutoModStatusRequest> {
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
            request,
        )
    }
    fn manage_held_automod_messages(
        &self,
        request: ManageHeldAutoModMeussageRequest,
    ) -> TwitchAPIRequest<ManageHeldAutoModMeussageRequest> {
        let mut url = self.build_url();
        url.path([MODERATION, "automod", "message"]);

        let mut headers = self.build_headers();
        headers.json();

        TwitchAPIRequest::new(
            EndpointType::ManageHeldAutoModMessages,
            url.build(),
            Method::POST,
            headers.build(),
            request,
        )
    }
    fn get_auto_mod_settings(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
    ) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path([MODERATION, "automod", "settings"])
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
        request: UpdateAutoModSettingsRequest,
    ) -> TwitchAPIRequest<UpdateAutoModSettingsRequest> {
        let mut url = self.build_url();
        url.path([MODERATION, "automod", "settings"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(MODERATOR_ID, moderator_id);

        let mut headers = self.build_headers();
        headers.json();

        TwitchAPIRequest::new(
            EndpointType::UpdateAutoModSettings,
            url.build(),
            Method::PUT,
            headers.build(),
            request,
        )
    }
    fn get_banned_users(
        &self,
        broadcaster_id: BroadcasterId,
        user_id: Option<Vec<UserId>>,
        first: Option<u64>,
        after: Option<String>,
        before: Option<String>,
    ) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path([MODERATION, "banned"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query_opt_extend(user_id.map(|ids| ids.into_iter().map(|id| (USER_ID, id))))
            .query_opt(FIRST, first.map(|x| x.to_string()))
            .query_opt(AFTER, after)
            .query_opt("before", before);

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
        request: BanUsersRequest,
    ) -> TwitchAPIRequest<BanUsersRequest> {
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
            request,
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
            EmptyBody,
        )
    }
    fn get_unban_requests(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        status: &str,
        user_id: Option<UserId>,
        after: Option<&str>,
        first: Option<u64>,
    ) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path([MODERATION, "unban_requests"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(MODERATOR_ID, moderator_id)
            .query("status", status)
            .query_opt(USER_ID, user_id)
            .query_opt(AFTER, after)
            .query_opt(FIRST, first.map(|x| x.to_string()));

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
        status: &str,
        resolution_text: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path([MODERATION, "unban_requests"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(MODERATOR_ID, moderator_id)
            .query_extend([("unban_request_id", unban_request_id), ("status", status)])
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
        first: Option<u64>,
        after: Option<String>,
    ) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path([MODERATION, "blocked_terms"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(MODERATOR_ID, moderator_id)
            .query_opt(FIRST, first.map(|x| x.to_string()))
            .query_opt(AFTER, after);

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
    ) -> TwitchAPIRequest<AddBlockedTermRequest> {
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
            EmptyBody,
        )
    }
    fn delete_chat_messages(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        message_id: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path([MODERATION, "chat"])
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
        after: Option<&str>,
        first: Option<u64>,
    ) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path([MODERATION, "channels"])
            .query(USER_ID, user_id)
            .query_opt("after", after)
            .query_opt("first", first.map(|x| x.to_string()));

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
        first: Option<&str>,
        after: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path([MODERATION, "moderators"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query_opt(USER_ID, user_id)
            .query_opt("first", first)
            .query_opt("after", after);

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
            EmptyBody,
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
            EmptyBody,
        )
    }
    fn get_vips(
        &self,
        user_ids: Option<Vec<UserId>>,
        broadcaster_id: BroadcasterId,
        first: Option<u64>,
        after: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path([CHANNELS, "vips"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query_opt_extend(user_ids.map(|ids| ids.into_iter().map(|id| (USER_ID, id))))
            .query_opt(FIRST, first.map(|x| x.to_string()))
            .query_opt(AFTER, after);

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
            EmptyBody,
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
            EmptyBody,
        )
    }
    fn update_shield_mode_status(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        is_active: bool,
    ) -> TwitchAPIRequest<UpdateShieldModeStatusRequest> {
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
    ) -> TwitchAPIRequest<EmptyBody> {
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
        request: WarnChatUserRequest,
    ) -> TwitchAPIRequest<WarnChatUserRequest> {
        let mut url = self.build_url();
        url.path([MODERATION, "warnings"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(MODERATOR_ID, moderator_id);

        TwitchAPIRequest::new(
            EndpointType::WarnChatUser,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            request,
        )
    }
}
