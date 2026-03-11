mod builder;
mod response;
mod types;

pub use builder::{
    DeleteGuestStarSlotBuilder, UpdateChannelGuestStarSettingsBuilder, UpdateGuestStarSlotBuilder,
    UpdateGuestStarSlotSettingsBuilder,
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
    /// Gets the channel settings for configuration of the Guest Star feature for a particular host
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster you want to get guest star settings for.
    /// * `moderator_id` - The ID of the broadcaster or a user that has permission to moderate the broadcaster’s chat room.
    ///
    /// # Returns
    ///
    /// Returns a [`GuestStarSettingsResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     guest_star::GuestStarAPI,
    ///     types::{BroadcasterId, ModeratorId}
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_channel_guest_star_settings(
    ///         &BroadcasterId::from("1234"),
    ///         &ModeratorId::from("5678")
    ///     )
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `channel:read:guest_star, channel:manage:guest_star, moderator:read:guest_star or moderator:manage:guest_star`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-channel-guest-star-settings>
    fn get_channel_guest_star_settings(
        &self,
        broadcaster_id: &BroadcasterId,
        moderator_id: &ModeratorId,
    ) -> impl Future<Output = Result<GuestStarSettingsResponse, Error>> + Send;

    /// Mutates the channel settings for configuration of the Guest Star feature for a particular host
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster you want to update Guest Star settings for.
    ///
    /// # Returns
    ///
    /// Returns a [`UpdateChannelGuestStarSettingsBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     guest_star::GuestStarAPI,
    ///     types::BroadcasterId,
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let broadcaster_id = BroadcasterId::from("1234");
    /// let response = api
    ///     .update_channel_guest_star_settings(&broadcaster_id)
    ///     .send()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `channel:manage:guest_star`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#update-channel-guest-star-settings>
    fn update_channel_guest_star_settings<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
    ) -> UpdateChannelGuestStarSettingsBuilder<'a>;

    /// Gets information about an ongoing Guest Star session for a particular channel
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - ID for the user hosting the Guest Star session.
    /// * `moderator_id` - The ID of the broadcaster or a user that has permission to moderate the broadcaster’s chat room.
    ///
    /// # Returns
    ///
    /// Returns a [`GustStarSessionResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     guest_star::GuestStarAPI,
    ///     types::{BroadcasterId, ModeratorId},
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_guest_star_session(
    ///         &BroadcasterId::from("1234"),
    ///         &ModeratorId::from("5678")
    ///     )
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `channel:read:guest_star, channel:manage:guest_star, moderator:read:guest_star or moderator:manage:guest_star`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-guest-star-session>
    fn get_guest_star_session(
        &self,
        broadcaster_id: &BroadcasterId,
        moderator_id: &ModeratorId,
    ) -> impl Future<Output = Result<GustStarSessionResponse, Error>> + Send;

    /// Programmatically creates a Guest Star session on behalf of the broadcaster
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster you want to create a Guest Star session for.
    ///
    /// # Returns
    ///
    /// Returns a [`GustStarSessionResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     guest_star::GuestStarAPI,
    ///     types::BroadcasterId,
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .create_guest_star_session(&BroadcasterId::from("1234"))
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `channel:manage:guest_star`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#create-guest-star-session>
    fn create_guest_star_session(
        &self,
        broadcaster_id: &BroadcasterId,
    ) -> impl Future<Output = Result<GustStarSessionResponse, Error>> + Send;

    /// Programmatically ends a Guest Star session on behalf of the broadcaster
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster you want to end a Guest Star session for.
    /// * `session_id` - ID for the session to end on behalf of the broadcaster.
    ///
    /// # Returns
    ///
    /// Returns a [`GustStarSessionResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     guest_star::GuestStarAPI,
    ///     types::{BroadcasterId, SessionId},
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .end_guest_star_session(&BroadcasterId::from("1234"), &SessionId::from("5678"))
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `channel:manage:guest_star`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#end-guest-star-session>
    fn end_guest_star_session(
        &self,
        broadcaster_id: &BroadcasterId,
        session_id: &SessionId,
    ) -> impl Future<Output = Result<GustStarSessionResponse, Error>> + Send;

    /// Provides the caller with a list of pending invites to a Guest Star session, including the invitee’s ready status while joining the waiting room
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster running the Guest Star session.
    /// * `moderator_id` - The ID of the broadcaster or a user that has permission to moderate the broadcaster’s chat room.
    /// * `session_id` - The session ID to query for invite status.
    ///
    /// # Returns
    ///
    /// Returns a [`GustStarInvitesResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     guest_star::GuestStarAPI,
    ///     types::{BroadcasterId, ModeratorId, SessionId},
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_guest_star_invites(
    ///         &BroadcasterId::from("1234"),
    ///         &ModeratorId::from("5678"),
    ///         &SessionId::from("5678"),
    ///     )
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `channel:read:guest_star, channel:manage:guest_star, moderator:read:guest_star or moderator:manage:guest_star`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-guest-star-invites>
    fn get_guest_star_invites(
        &self,
        broadcaster_id: &BroadcasterId,
        moderator_id: &ModeratorId,
        session_id: &SessionId,
    ) -> impl Future<Output = Result<GustStarInvitesResponse, Error>> + Send;

    /// Sends an invite to a specified guest on behalf of the broadcaster for a Guest Star session in progress
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster running the Guest Star session.
    /// * `moderator_id` - The ID of the broadcaster or a user that has permission to moderate the broadcaster’s chat room.
    /// * `session_id` - The session ID for the invite to be sent on behalf of the broadcaster.
    /// * `guest_id` - Twitch User ID for the guest to invite to the Guest Star session.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     guest_star::GuestStarAPI,
    ///     types::{BroadcasterId, ModeratorId, SessionId, UserId}
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let broadcaster_id = BroadcasterId::from("1234");
    /// let moderator_id = ModeratorId::from("5678");
    /// let session_id = SessionId::from("5678");
    /// let user_id = UserId::from("8428");
    /// let response = api
    ///     .send_guest_star_invite(&broadcaster_id, &moderator_id, &session_id, &user_id)
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// No scope required
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#send-guest-star-invite>
    fn send_guest_star_invite(
        &self,
        broadcaster_id: &BroadcasterId,
        moderator_id: &ModeratorId,
        session_id: &SessionId,
        guest_id: &UserId,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Revokes a previously sent invite for a Guest Star session
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster running the Guest Star session.
    /// * `moderator_id` - The ID of the broadcaster or a user that has permission to moderate the broadcaster’s chat room.
    /// * `session_id` - The ID of the session for the invite to be revoked on behalf of the broadcaster.
    /// * `guest_id` - Twitch User ID for the guest to revoke the Guest Star session invite from.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     guest_star::GuestStarAPI,
    ///     types::{BroadcasterId, ModeratorId, SessionId, UserId}
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let broadcaster_id = BroadcasterId::from("1234");
    /// let moderator_id = ModeratorId::from("5678");
    /// let session_id = SessionId::from("5678");
    /// let user_id = UserId::from("8428");
    /// let response = api
    ///     .delete_guest_star_invite(&broadcaster_id, &moderator_id, &session_id, &user_id)
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `channel:manage:guest_star or moderator:manage:guest_star`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#delete-guest-star-invite>
    fn delete_guest_star_invite(
        &self,
        broadcaster_id: &BroadcasterId,
        moderator_id: &ModeratorId,
        session_id: &SessionId,
        guest_id: &UserId,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Allows a previously invited user to be assigned a slot within the active Guest Star session, once that guest has indicated they are ready to join
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster running the Guest Star session.
    /// * `moderator_id` - The ID of the broadcaster or a user that has permission to moderate the broadcaster’s chat room.
    /// * `session_id` - The ID of the Guest Star session in which to assign the slot.
    /// * `guest_id` - The Twitch User ID corresponding to the guest to assign a slot in the session.
    /// * `slot_id` - The slot assignment to give to the user.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     guest_star::GuestStarAPI,
    ///     types::{BroadcasterId, ModeratorId, SessionId, UserId}
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let broadcaster_id = BroadcasterId::from("1234");
    /// let moderator_id = ModeratorId::from("5678");
    /// let session_id = SessionId::from("5678");
    /// let user_id = UserId::from("8428");
    /// let response = api
    ///     .assign_guest_star_slot(
    ///         &broadcaster_id,
    ///         &moderator_id,
    ///         &session_id,
    ///         &user_id,
    ///         "slot_id"
    ///     )
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `channel:manage:guest_star or moderator:manage:guest_star`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#assign-guest-star-slot>
    fn assign_guest_star_slot(
        &self,
        broadcaster_id: &BroadcasterId,
        moderator_id: &ModeratorId,
        session_id: &SessionId,
        guest_id: &UserId,
        slot_id: &str,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Allows a user to update the assigned slot for a particular user within the active Guest Star session
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster running the Guest Star session.
    /// * `moderator_id` - The ID of the broadcaster or a user that has permission to moderate the broadcaster’s chat room.
    /// * `session_id` - The ID of the Guest Star session in which to update slot settings.
    /// * `source_slot_id` - The slot assignment previously assigned to a user.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     guest_star::GuestStarAPI,
    ///     types::{BroadcasterId, ModeratorId, SessionId}
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let broadcaster_id = BroadcasterId::from("1234");
    /// let moderator_id = ModeratorId::from("5678");
    /// let session_id = SessionId::from("5678");
    /// let response = api
    ///     .update_guest_star_slot(
    ///     &broadcaster_id,
    ///     &moderator_id,
    ///     &session_id,
    ///     "source_slot_id",
    ///     )
    ///     .send()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `channel:manage:guest_star or moderator:manage:guest_star`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#update-guest-star-slot>
    fn update_guest_star_slot<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
        session_id: &'a SessionId,
        source_slot_id: &'a str,
    ) -> UpdateGuestStarSlotBuilder<'a>;

    /// Allows a caller to remove a slot assignment from a user participating in an active Guest Star session
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster running the Guest Star session.
    /// * `moderator_id` - The ID of the broadcaster or a user that has permission to moderate the broadcaster’s chat room.
    /// * `session_id` - The ID of the Guest Star session in which to remove the slot assignment.
    /// * `guest_id` - The Twitch User ID corresponding to the guest to remove from the session.
    /// * `slot_id` - The slot ID representing the slot assignment to remove from the session.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     guest_star::GuestStarAPI,
    ///     types::{BroadcasterId, ModeratorId, SessionId, UserId}
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let broadcaster_id = BroadcasterId::from("1234");
    /// let moderator_id = ModeratorId::from("5678");
    /// let session_id = SessionId::from("5678");
    /// let user_id = UserId::from("8428");
    /// let response = api
    ///     .delete_guest_star_slot(
    ///     &broadcaster_id,
    ///     &moderator_id,
    ///     &session_id,
    ///     &user_id,
    ///     "slot_id",
    ///     )
    ///     .send()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `channel:manage:guest_star or moderator:manage:guest_star`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#delete-guest-star-slot>
    fn delete_guest_star_slot<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
        session_id: &'a SessionId,
        guest_id: &'a UserId,
        slot_id: &'a str,
    ) -> DeleteGuestStarSlotBuilder<'a>;

    /// Allows a user to update slot settings for a particular guest within a Guest Star session
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster running the Guest Star session.
    /// * `moderator_id` - The ID of the broadcaster or a user that has permission to moderate the broadcaster’s chat room.
    /// * `session_id` - The ID of the Guest Star session in which to update a slot’s settings.
    /// * `slot_id` - The slot assignment that has previously been assigned to a user.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     guest_star::GuestStarAPI,
    ///     types::{BroadcasterId, ModeratorId, SessionId}
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let broadcaster_id = BroadcasterId::from("1234");
    /// let moderator_id = ModeratorId::from("5678");
    /// let session_id = SessionId::from("5678");
    /// let response = api
    ///     .update_guest_star_slot_settings(
    ///     &broadcaster_id,
    ///     &moderator_id,
    ///     &session_id,
    ///     "slot_id",
    ///     )
    ///     .send()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `channel:manage:guest_star or moderator:manage:guest_star`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#update-guest-star-slot-settings>
    fn update_guest_star_slot_settings<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
        session_id: &'a SessionId,
        slot_id: &'a str,
    ) -> UpdateGuestStarSlotSettingsBuilder<'a>;
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
    ) -> UpdateChannelGuestStarSettingsBuilder<'a> {
        UpdateChannelGuestStarSettingsBuilder::new(self, broadcaster_id)
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
    ) -> UpdateGuestStarSlotBuilder<'a> {
        UpdateGuestStarSlotBuilder::new(
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
    ) -> DeleteGuestStarSlotBuilder<'a> {
        DeleteGuestStarSlotBuilder::new(
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
    ) -> UpdateGuestStarSlotSettingsBuilder<'a> {
        UpdateGuestStarSlotSettingsBuilder::new(
            self,
            broadcaster_id,
            moderator_id,
            session_id,
            slot_id,
        )
    }
}
