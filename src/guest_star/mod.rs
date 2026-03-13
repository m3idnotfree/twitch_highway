mod builder;
mod response;
mod types;

pub use builder::{
    DeleteGuestStarSlot, UpdateChannelGuestStarSettings, UpdateGuestStarSlot,
    UpdateGuestStarSlotSettings,
};
pub use response::{GuestStarSettingsResponse, GustStarInvitesResponse, GustStarSessionResponse};
pub use types::{
    GroupLayout, Guest, GuestSetting, GuestStarInvite, GuestStarStatus, GustStarSession,
    GustStarSetting,
};

use std::future::Future;

use crate::{
    types::{
        constants::{
            BROADCASTER_ID, CHANNEL_SETTINGS, GUEST_ID, GUEST_STAR, INVITES, MODERATOR_ID, SESSION,
            SESSION_ID, SLOT, SLOT_ID,
        },
        BroadcasterId, ModeratorId, SessionId, UserId,
    },
    Client, Error,
};

pub trait GuestStarAPI {
    /// See <https://dev.twitch.tv/docs/api/reference/#get-channel-guest-star-settings>
    fn get_channel_guest_star_settings(
        &self,
        broadcaster_id: &BroadcasterId,
        moderator_id: &ModeratorId,
    ) -> impl Future<Output = Result<GuestStarSettingsResponse, Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#update-channel-guest-star-settings>
    fn update_channel_guest_star_settings<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
    ) -> UpdateChannelGuestStarSettings<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#get-guest-star-session>
    fn get_guest_star_session(
        &self,
        broadcaster_id: &BroadcasterId,
        moderator_id: &ModeratorId,
    ) -> impl Future<Output = Result<GustStarSessionResponse, Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#create-guest-star-session>
    fn create_guest_star_session(
        &self,
        broadcaster_id: &BroadcasterId,
    ) -> impl Future<Output = Result<GustStarSessionResponse, Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#end-guest-star-session>
    fn end_guest_star_session(
        &self,
        broadcaster_id: &BroadcasterId,
        session_id: &SessionId,
    ) -> impl Future<Output = Result<GustStarSessionResponse, Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#get-guest-star-invites>
    fn get_guest_star_invites(
        &self,
        broadcaster_id: &BroadcasterId,
        moderator_id: &ModeratorId,
        session_id: &SessionId,
    ) -> impl Future<Output = Result<GustStarInvitesResponse, Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#send-guest-star-invite>
    fn send_guest_star_invite(
        &self,
        broadcaster_id: &BroadcasterId,
        moderator_id: &ModeratorId,
        session_id: &SessionId,
        guest_id: &UserId,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#delete-guest-star-invite>
    fn delete_guest_star_invite(
        &self,
        broadcaster_id: &BroadcasterId,
        moderator_id: &ModeratorId,
        session_id: &SessionId,
        guest_id: &UserId,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#assign-guest-star-slot>
    fn assign_guest_star_slot(
        &self,
        broadcaster_id: &BroadcasterId,
        moderator_id: &ModeratorId,
        session_id: &SessionId,
        guest_id: &UserId,
        slot_id: &str,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#update-guest-star-slot>
    fn update_guest_star_slot<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
        session_id: &'a SessionId,
        source_slot_id: &'a str,
    ) -> UpdateGuestStarSlot<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#delete-guest-star-slot>
    fn delete_guest_star_slot<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
        session_id: &'a SessionId,
        guest_id: &'a UserId,
        slot_id: &'a str,
    ) -> DeleteGuestStarSlot<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#update-guest-star-slot-settings>
    fn update_guest_star_slot_settings<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
        session_id: &'a SessionId,
        slot_id: &'a str,
    ) -> UpdateGuestStarSlotSettings<'a>;
}

impl GuestStarAPI for Client {
    async fn get_channel_guest_star_settings(
        &self,
        broadcaster_id: &BroadcasterId,
        moderator_id: &ModeratorId,
    ) -> Result<GuestStarSettingsResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([GUEST_STAR, CHANNEL_SETTINGS]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id)
            .append_pair(MODERATOR_ID, moderator_id);

        self.json(self.http_client().get(url)).await
    }

    fn update_channel_guest_star_settings<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
    ) -> UpdateChannelGuestStarSettings<'a> {
        UpdateChannelGuestStarSettings::new(self, broadcaster_id)
    }

    async fn get_guest_star_session(
        &self,
        broadcaster_id: &BroadcasterId,
        moderator_id: &ModeratorId,
    ) -> Result<GustStarSessionResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([GUEST_STAR, SESSION]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id)
            .append_pair(MODERATOR_ID, moderator_id);

        self.json(self.http_client().get(url)).await
    }

    async fn create_guest_star_session(
        &self,
        broadcaster_id: &BroadcasterId,
    ) -> Result<GustStarSessionResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([GUEST_STAR, SESSION]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id);

        self.json(self.http_client().post(url)).await
    }

    async fn end_guest_star_session(
        &self,
        broadcaster_id: &BroadcasterId,
        session_id: &SessionId,
    ) -> Result<GustStarSessionResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([GUEST_STAR, SESSION]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id)
            .append_pair(SESSION_ID, session_id);

        self.json(self.http_client().delete(url)).await
    }

    async fn get_guest_star_invites(
        &self,
        broadcaster_id: &BroadcasterId,
        moderator_id: &ModeratorId,
        session_id: &SessionId,
    ) -> Result<GustStarInvitesResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([GUEST_STAR, INVITES]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id)
            .append_pair(MODERATOR_ID, moderator_id)
            .append_pair(SESSION_ID, session_id);

        let req = self.http_client().get(url);
        self.json(req).await
    }

    async fn send_guest_star_invite(
        &self,
        broadcaster_id: &BroadcasterId,
        moderator_id: &ModeratorId,
        session_id: &SessionId,
        guest_id: &UserId,
    ) -> Result<(), Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([GUEST_STAR, INVITES]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id)
            .append_pair(MODERATOR_ID, moderator_id)
            .append_pair(SESSION_ID, session_id)
            .append_pair(GUEST_ID, guest_id);

        self.no_content(self.http_client().post(url)).await
    }

    async fn delete_guest_star_invite(
        &self,
        broadcaster_id: &BroadcasterId,
        moderator_id: &ModeratorId,
        session_id: &SessionId,
        guest_id: &UserId,
    ) -> Result<(), Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([GUEST_STAR, INVITES]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id)
            .append_pair(MODERATOR_ID, moderator_id)
            .append_pair(SESSION_ID, session_id)
            .append_pair(GUEST_ID, guest_id);

        self.no_content(self.http_client().delete(url)).await
    }

    async fn assign_guest_star_slot(
        &self,
        broadcaster_id: &BroadcasterId,
        moderator_id: &ModeratorId,
        session_id: &SessionId,
        guest_id: &UserId,
        slot_id: &str,
    ) -> Result<(), Error> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().extend([GUEST_STAR, SLOT]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id)
            .append_pair(MODERATOR_ID, moderator_id)
            .append_pair(SESSION_ID, session_id)
            .append_pair(GUEST_ID, guest_id)
            .append_pair(SLOT_ID, slot_id);

        self.no_content(self.http_client().post(url)).await
    }

    fn update_guest_star_slot<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
        session_id: &'a SessionId,
        source_slot_id: &'a str,
    ) -> UpdateGuestStarSlot<'a> {
        UpdateGuestStarSlot::new(
            self,
            broadcaster_id,
            moderator_id,
            session_id,
            source_slot_id,
        )
    }

    fn delete_guest_star_slot<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
        session_id: &'a SessionId,
        guest_id: &'a UserId,
        slot_id: &'a str,
    ) -> DeleteGuestStarSlot<'a> {
        DeleteGuestStarSlot::new(
            self,
            broadcaster_id,
            moderator_id,
            session_id,
            guest_id,
            slot_id,
        )
    }

    fn update_guest_star_slot_settings<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
        session_id: &'a SessionId,
        slot_id: &'a str,
    ) -> UpdateGuestStarSlotSettings<'a> {
        UpdateGuestStarSlotSettings::new(self, broadcaster_id, moderator_id, session_id, slot_id)
    }
}
