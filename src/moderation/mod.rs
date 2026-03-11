mod builder;
mod response;
mod types;

pub use builder::{
    BanUserBuilder, DeleteChatMessagesBuilder, GetBannedUsersBuilder, GetBlockedTermsBuilder,
    GetModeratedChannelsBuilder, GetModeratorsBuilder, GetUnbanRequestsBuilder, GetVipsBuilder,
    ResolveUnbanRequestBuilder, UpdateAutomodSettingsBuilder,
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
    types::{
        constants::{
            AUTOMOD, BANS, BLOCKED_TERMS, BROADCASTER_ID, CHANNELS, ENFORCEMENTS, ID, MESSAGE,
            MODERATION, MODERATORS, MODERATOR_ID, SETTINGS, SHIELD_MODE, STATUS, USER_ID, VIPS,
            WARNINGS,
        },
        BlockedTermId, BroadcasterId, ModeratorId, UserId,
    },
    Client, Error,
};

const SUSPICIOUS_USERS: &str = "suspicious_users";

pub trait ModerationAPI {
    /// Checks whether AutoMod would flag the specified message for review
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster whose AutoMod settings and list of blocked terms are used to check the message.
    /// * `data` - Array of messages to check
    ///
    /// # Returns
    ///
    /// Returns a [`CheckAutoModStatusResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     moderation::{CheckAutoMod, ModerationAPI},
    ///     types::BroadcasterId
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .check_automod_status(
    ///         &BroadcasterId::from("1234"),
    ///         &[CheckAutoMod::new("123", "text")],
    ///     )
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `moderation:read`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#check-automod-status>
    fn check_automod_status(
        &self,
        broadcaster_id: &BroadcasterId,
        data: &[CheckAutoMod],
    ) -> impl Future<Output = Result<CheckAutoModStatusResponse, Error>> + Send;

    /// Allow or deny the message that AutoMod flagged for review
    ///
    /// # Arguments
    ///
    /// * `user_id` - The moderator who is approving or denying the held message.
    /// * `msg_id` - The ID of the message to allow or deny.
    /// * `action` - [`AutoModAction`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     moderation::{AutoModAction, ModerationAPI},
    ///     types::UserId,
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .manage_held_automod_messages(
    ///         &UserId::from("1234"),
    ///         "836485",
    ///         AutoModAction::ALLOW,
    ///     )
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `moderator:manage:automod`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#manage-held-automod-messages>
    fn manage_held_automod_messages(
        &self,
        user_id: &UserId,
        msg_id: &str,
        action: AutoModAction,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Gets the broadcaster’s AutoMod settings
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster whose AutoMod settings you want to get.
    /// * `moderator_id` - The ID of the broadcaster or a user that has permission to moderate the broadcaster’s chat room.
    ///
    /// # Returns
    ///
    /// Returns a [`AutoModSettingsResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     moderation::ModerationAPI,
    ///     types::{BroadcasterId, ModeratorId}
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_automod_settings(
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
    /// `moderator:read:automod_settings`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-automod-settings>
    fn get_automod_settings(
        &self,
        broadcaster_id: &BroadcasterId,
        moderator_id: &ModeratorId,
    ) -> impl Future<Output = Result<AutoModSettingsResponse, Error>> + Send;

    /// Updates the broadcaster’s AutoMod settings
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster whose AutoMod settings you want to update.
    /// * `moderator_id` - The ID of the broadcaster or a user that has permission to moderate the broadcaster’s chat room.
    ///
    /// # Returns
    ///
    /// Returns a [`UpdateAutomodSettingsBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     moderation::ModerationAPI,
    ///     types::{BroadcasterId, ModeratorId},
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .update_automod_settings(
    ///         &BroadcasterId::from("1234"),
    ///         &ModeratorId::from("5678"),
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
    ///  `moderator:manage:automod_settings`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#update-automod-settings>
    fn update_automod_settings<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
    ) -> UpdateAutomodSettingsBuilder<'a>;

    /// Gets all users that the broadcaster banned or put in a timeout
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster whose list of banned users you want to get
    ///
    /// # Returns
    ///
    /// Returns a [`GetBannedUsersBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     moderation::ModerationAPI,
    ///     types::BroadcasterId,
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_banned_users(&BroadcasterId::from("1234"))
    ///     .send()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `moderation:read or moderator:manage:banned_users`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-banned-users>
    fn get_banned_users<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
    ) -> GetBannedUsersBuilder<'a>;

    /// Bans a user from participating in the specified broadcaster’s chat room or puts them in a timeout
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster whose chat room the user is being banned from.
    /// * `moderator_id` - The ID of the broadcaster or a user that has permission to moderate the broadcaster’s chat room.
    /// * `user_id` - The ID of the user to ban or put in a timeout.
    /// # Returns
    ///
    /// Returns a [`BanUserBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     moderation::ModerationAPI,
    ///     types::{BroadcasterId, ModeratorId, UserId},
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .ban_user(
    ///         &BroadcasterId::from("1234"),
    ///         &ModeratorId::from("5678"),
    ///         &UserId::from("6789")
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
    /// `moderator:manage:banned_users`
    ///
    /// API Reference
    ///
    /// maximum is 1,209,600 seconds (2 weeks).
    /// <https://dev.twitch.tv/docs/api/reference/#ban-user>
    fn ban_user<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
        user_id: &'a UserId,
    ) -> BanUserBuilder<'a>;

    /// emoves the ban or timeout that was placed on the specified user
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster whose chat room the user is banned from chatting in.
    /// * `moderator_id` - The ID of the broadcaster or a user that has permission to moderate the broadcaster’s chat room.
    /// * `user_id` - The ID of the user to remove the ban or timeout from.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     moderation::ModerationAPI,
    ///     types::{BroadcasterId, ModeratorId, UserId},
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .unban_user(
    ///         &BroadcasterId::from("1234"),
    ///         &ModeratorId::from("5678"),
    ///         &UserId::from("5678"),
    ///     )
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `moderator:manage:banned_users `
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#unban-user>
    fn unban_user(
        &self,
        broadcaster_id: &BroadcasterId,
        moderator_id: &ModeratorId,
        user_id: &UserId,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Gets a list of unban requests for a broadcaster’s channel
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster whose channel is receiving unban requests.
    /// * `moderator_id` - The ID of the broadcaster or a user that has permission to moderate the broadcaster’s unban requests.
    /// * `status` -
    ///
    /// # Returns
    ///
    /// Returns a [`GetUnbanRequestsBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     moderation::{ModerationAPI, UnbanRequestStatus},
    ///     types::{BroadcasterId, ModeratorId},
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_unban_requests(
    ///         &BroadcasterId::from("1234"),
    ///         &ModeratorId::from("5678"),
    ///         UnbanRequestStatus::Pending,
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
    /// `moderator:read:unban_requests or moderator:manage:unban_requests`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-unban-requests>
    fn get_unban_requests<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
        status: UnbanRequestStatus,
    ) -> GetUnbanRequestsBuilder<'a>;

    /// Resolves an unban request by approving or denying it
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster whose channel is approving or denying the unban request.
    /// * `moderator_id` - The ID of the broadcaster or a user that has permission to moderate the broadcaster’s unban requests.
    /// * `unban_request_id` - The ID of the broadcaster or a user that has permission to moderate the broadcaster’s unban requests.
    /// * `status` - [`UnbanRequestStatus`]
    ///
    /// # Returns
    ///
    /// Returns a [`ResolveUnbanRequestBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     moderation::{ModerationAPI, UnbanRequestStatus},
    ///     types::{BroadcasterId, ModeratorId},
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .resolve_unban_request(
    ///         &BroadcasterId::from("1234"),
    ///         &ModeratorId::from("5678"),
    ///         "unban_request_id",
    ///         UnbanRequestStatus::Denied,
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
    /// `moderator:manage:unban_requests`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#resolve-unban-requests>
    fn resolve_unban_request<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
        unban_request_id: &'a str,
        status: UnbanRequestStatus,
    ) -> ResolveUnbanRequestBuilder<'a>;

    /// Gets the broadcaster’s list of non-private, blocked words or phrases
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster whose blocked terms you’re getting.
    /// * `moderator_id` - The ID of the broadcaster or a user that has permission to moderate the broadcaster’s chat room.
    ///
    /// # Returns
    ///
    /// Returns a [`GetBlockedTermsBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     moderation::ModerationAPI,
    ///     types::{BroadcasterId, ModeratorId},
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_blocked_terms(
    ///         &BroadcasterId::from("1234"),
    ///         &ModeratorId::from("5678")
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
    /// `moderator:read:blocked_terms or moderator:manage:blocked_terms`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-blocked-terms>
    fn get_blocked_terms<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
    ) -> GetBlockedTermsBuilder<'a>;

    /// Adds a word or phrase to the broadcaster’s list of blocked terms
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster that owns the list of blocked terms.
    /// * `moderator_id` - The ID of the broadcaster or a user that has permission to moderate the broadcaster’s chat room.
    /// * `text` - The word or phrase to block from being used in the broadcaster’s chat room. (min 2, max 500)
    ///
    /// # Returns
    ///
    /// Returns a [`BlockedTermsResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     moderation::ModerationAPI,
    ///     types::{BroadcasterId, ModeratorId},
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .add_blocked_term(
    ///         &BroadcasterId::from("1234"),
    ///         &ModeratorId::from("5678"),
    ///         "text"
    ///     )
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `moderator:manage:blocked_terms`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#add-blocked-term>
    fn add_blocked_term(
        &self,
        broadcaster_id: &BroadcasterId,
        moderator_id: &ModeratorId,
        text: &str,
    ) -> impl Future<Output = Result<BlockedTermsResponse, Error>> + Send;

    /// Removes the word or phrase from the broadcaster’s list of blocked terms
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster that owns the list of blocked terms.
    /// * `moderator_id` - The ID of the broadcaster or a user that has permission to moderate the broadcaster’s chat room.
    /// * `id` - The ID of the blocked term to remove from the broadcaster’s list of blocked terms.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     moderation::ModerationAPI,
    ///     types::{BroadcasterId, BlockedTermId, ModeratorId},
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .remove_blocked_term(
    ///         &BroadcasterId::from("1234"),
    ///         &ModeratorId::from("5678"),
    ///         &BlockedTermId::from("5678"),
    ///     )
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `moderator:manage:blocked_terms`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#remove-blocked-term>
    fn remove_blocked_term(
        &self,
        broadcaster_id: &BroadcasterId,
        moderator_id: &ModeratorId,
        id: &BlockedTermId,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Removes a single chat message or all chat messages from the broadcaster’s chat room
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster that owns the chat room to remove messages from.
    /// * `moderator_id` - The ID of the broadcaster or a user that has permission to moderate the broadcaster’s chat room.
    ///
    /// # Returns
    ///
    /// Returns a [`DeleteChatMessagesBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     moderation::ModerationAPI,
    ///     types::{BroadcasterId, ModeratorId},
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .delete_chat_messages(
    ///         &BroadcasterId::from("1234"),
    ///         &ModeratorId::from("5678"),
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
    /// `moderator:manage:chat_messages`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#delete-chat-messages>
    fn delete_chat_messages<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
    ) -> DeleteChatMessagesBuilder<'a>;

    /// Gets a list of channels that the specified user has moderator privileges in
    ///
    /// # Arguments
    ///
    /// * `user_id` - A user’s ID. Returns the list of channels that this user has moderator privileges in.
    ///
    /// # Returns
    ///
    /// Returns a [`GetModeratedChannelsBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     moderation::ModerationAPI,
    ///     types::UserId,
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_moderated_channels(&UserId::from("1234"))
    ///     .send()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `user:read:moderated_channels`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-moderated-channels>
    fn get_moderated_channels<'a>(&'a self, user_id: &'a UserId)
        -> GetModeratedChannelsBuilder<'a>;

    /// Gets all users allowed to moderate the broadcaster’s chat room
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster whose list of moderators you want to get.
    ///
    /// # Returns
    ///
    /// Returns a [`GetModeratorsBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     moderation::ModerationAPI,
    ///     types::BroadcasterId,
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_moderators(&BroadcasterId::from("1234"))
    ///     .send()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `moderation:read or channel:manage:moderators`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-moderators>
    fn get_moderators<'a>(&'a self, broadcaster_id: &'a BroadcasterId) -> GetModeratorsBuilder<'a>;

    /// Adds a moderator to the broadcaster’s chat room
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster that owns the chat room.
    /// * `user_id` - The ID of the user to add as a moderator in the broadcaster’s chat room.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     moderation::ModerationAPI,
    ///     types::{BroadcasterId, UserId},
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .add_channel_moderator(
    ///         &BroadcasterId::from("1234"),
    ///         &UserId::from("5678"),
    ///     )
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `channel:manage:moderators`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#add-channel-moderator>
    fn add_channel_moderator(
        &self,
        broadcaster_id: &BroadcasterId,
        user_id: &UserId,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Removes a moderator from the broadcaster’s chat room
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster that owns the chat room.
    /// * `user_id` - The ID of the user to remove as a moderator from the broadcaster’s chat room.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     moderation::ModerationAPI,
    ///     types::{BroadcasterId, UserId},
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .remove_channel_moderator(
    ///         &BroadcasterId::from("1234"),
    ///         &UserId::from("5678")
    ///     )
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `channel:manage:moderators`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#remove-channel-moderator>
    fn remove_channel_moderator(
        &self,
        broadcaster_id: &BroadcasterId,
        user_id: &UserId,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Gets a list of the broadcaster’s VIPs
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster whose list of VIPs you want to get.
    ///
    /// # Returns
    ///
    /// Returns a [`GetVipsBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     moderation::ModerationAPI,
    ///     types::BroadcasterId,
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_vips(&BroadcasterId::from("1234"))
    ///     .send()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `channel:read:vips or channel:manage:vips `
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-vips>
    fn get_vips<'a>(&'a self, broadcaster_id: &'a BroadcasterId) -> GetVipsBuilder<'a>;

    /// Adds the specified user as a VIP in the broadcaster’s channel
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster that’s adding the user as a VIP.
    /// * `user_id` - The ID of the user to give VIP status to.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     moderation::ModerationAPI,
    ///     types::{BroadcasterId, UserId},
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .add_channel_vip(
    ///         &BroadcasterId::from("1234"),
    ///         &UserId::from("5678")
    ///     )
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `channel:manage:vips`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#add-channel-vip>
    fn add_channel_vip(
        &self,
        broadcaster_id: &BroadcasterId,
        user_id: &UserId,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Removes the specified user as a VIP in the broadcaster’s channel
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster who owns the channel where the user has VIP status.
    /// * `user_id` - The ID of the user to remove VIP status from.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     moderation::ModerationAPI,
    ///     types::{BroadcasterId, UserId},
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .remove_channel_vip(
    ///         &BroadcasterId::from("1234"),
    ///         &UserId::from("5678"),
    ///     )
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `channel:manage:vips`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#remove-channel-vip>
    fn remove_channel_vip(
        &self,
        broadcaster_id: &BroadcasterId,
        user_id: &UserId,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Activates or deactivates the broadcaster’s Shield Mode
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster whose Shield Mode you want to activate or deactivate.
    /// * `moderator_id` - The ID of the broadcaster whose Shield Mode you want to activate or deactivate.
    /// * `is_active` - A Boolean value that determines whether to activate Shield Mode.
    ///
    /// # Returns
    ///
    /// Returns a [`ShieldModeStatusResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     moderation::ModerationAPI,
    ///     types::{BroadcasterId, ModeratorId},
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .update_shield_mode_status(
    ///         &BroadcasterId::from("1234"),
    ///         &ModeratorId::from("5678"),
    ///         false
    ///     )
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `moderator:manage:shield_mode`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#update-shield-mode-status>
    fn update_shield_mode_status(
        &self,
        broadcaster_id: &BroadcasterId,
        moderator_id: &ModeratorId,
        is_active: bool,
    ) -> impl Future<Output = Result<ShieldModeStatusResponse, Error>> + Send;

    /// Gets the broadcaster’s Shield Mode activation status
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster whose Shield Mode activation status you want to get.
    /// * `moderator_id` - The ID of the broadcaster or a user that is one of the broadcaster’s moderators.
    ///
    /// # Returns
    ///
    /// Returns a [`ShieldModeStatusResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     moderation::ModerationAPI,
    ///     types::{BroadcasterId, ModeratorId},
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_shield_mode_status(
    ///         &BroadcasterId::from("1234"),
    ///         &ModeratorId::from("5678"),
    ///     )
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `moderator:read:shield_mode or moderator:manage:shield_mode`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-shield-mode-status>
    fn get_shield_mode_status(
        &self,
        broadcaster_id: &BroadcasterId,
        moderator_id: &ModeratorId,
    ) -> impl Future<Output = Result<ShieldModeStatusResponse, Error>> + Send;

    /// Warns a user in the specified broadcaster’s chat room, preventing them from chat interaction until the warning is acknowledged
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the channel in which the warning will take effect.
    /// * `moderator_id` - The ID of the twitch user who requested the warning.
    /// * `user_id` - The ID of the twitch user to be warned.
    /// * `reason` - custom reason for the warning. Max 500 chars.
    ///
    /// # Returns
    ///
    /// Returns a [`WarnChatUsersResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     moderation::{ModerationAPI},
    ///     types::{BroadcasterId, ModeratorId, UserId},
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .warn_chat_user(
    ///         &BroadcasterId::from("1234"),
    ///         &ModeratorId::from("5678"),
    ///         &UserId::from("7890"),
    ///         "no reason"
    ///     )
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `moderator:manage:warnings`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#warn-chat-user>
    fn warn_chat_user(
        &self,
        broadcaster_id: &BroadcasterId,
        moderator_id: &ModeratorId,
        user_id: &UserId,
        reason: &str,
    ) -> impl Future<Output = Result<WarnChatUsersResponse, Error>> + Send;

    fn add_suspicious_status_to_chat_user(
        &self,
        broadcaster_id: &BroadcasterId,
        moderator_id: &ModeratorId,
        user_id: &UserId,
        status: SuspiciousStatus,
    ) -> impl Future<Output = Result<SuspiciousResponse, Error>> + Send;

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
    ) -> UpdateAutomodSettingsBuilder<'a> {
        UpdateAutomodSettingsBuilder::new(self, broadcaster_id, moderator_id)
    }

    fn get_banned_users<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
    ) -> GetBannedUsersBuilder<'a> {
        GetBannedUsersBuilder::new(self, broadcaster_id)
    }

    fn ban_user<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
        user_id: &'a UserId,
    ) -> BanUserBuilder<'a> {
        BanUserBuilder::new(self, broadcaster_id, moderator_id, user_id)
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
    ) -> GetUnbanRequestsBuilder<'a> {
        GetUnbanRequestsBuilder::new(self, broadcaster_id, moderator_id, status)
    }

    fn resolve_unban_request<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
        unban_request_id: &'a str,
        status: UnbanRequestStatus,
    ) -> ResolveUnbanRequestBuilder<'a> {
        ResolveUnbanRequestBuilder::new(
            self,
            broadcaster_id,
            moderator_id,
            unban_request_id,
            status,
        )
    }

    fn get_blocked_terms<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
    ) -> GetBlockedTermsBuilder<'a> {
        GetBlockedTermsBuilder::new(self, broadcaster_id, moderator_id)
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
    ) -> DeleteChatMessagesBuilder<'a> {
        DeleteChatMessagesBuilder::new(self, broadcaster_id, moderator_id)
    }

    fn get_moderated_channels<'a>(
        &'a self,
        user_id: &'a UserId,
    ) -> GetModeratedChannelsBuilder<'a> {
        GetModeratedChannelsBuilder::new(self, user_id)
    }

    fn get_moderators<'a>(&'a self, broadcaster_id: &'a BroadcasterId) -> GetModeratorsBuilder<'a> {
        GetModeratorsBuilder::new(self, broadcaster_id)
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

    fn get_vips<'a>(&'a self, broadcaster_id: &'a BroadcasterId) -> GetVipsBuilder<'a> {
        GetVipsBuilder::new(self, broadcaster_id)
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
