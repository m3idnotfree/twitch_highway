mod builder;
mod response;
mod types;

pub use response::{
    BadgesResponse, ChatSettingResponse, ChattersResponse, EmotesResponse, SendChatMessageResponse,
    SharedChatSessionResponse, UsersColorResponse,
};
pub use types::{
    AnnouncementColor, Badge, BroadcasterIdField, ChatColor, ChatSetting, Chatter, DropReason,
    Emote, EmoteType, Format, MessageResponse, Scale, SharedChatSession, ThemeMode, UserColor,
    Version,
};

pub use builder::{
    GetChatSettingsBuilder, GetChattersBuilder, GetUserEmotesBuilder, SendChatAnnouncementBuilder,
    SendChatMessageBuilder, UpdateChatSettingsBuilder,
};

use std::future::Future;

use crate::{
    types::{
        constants::{
            BADGES, BROADCASTER_ID, CHAT, COLOR, EMOTES, EMOTE_SET_ID, FROM_BROADCASTER_ID, GLOBAL,
            MODERATOR_ID, SESSION, SET, SHARED_CHAT, SHOUTOUTS, TO_BROADCASTER_ID, USER_ID,
        },
        BroadcasterId, ModeratorId, UserId,
    },
    Client, Error,
};

pub trait ChatAPI {
    /// Gets the list of users that are connected to the broadcaster’s chat session
    ///
    /// # Returns
    ///
    /// Returns a [`GetChattersBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     chat::ChatAPI,
    ///     types::{BroadcasterId, ModeratorId},
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_chatters(&BroadcasterId::from("1234"), &ModeratorId::from("5678"))
    ///     .first(5)
    ///     .after("eyJiI...")
    ///     .send()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `moderator:read:chatters`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-chatters>
    fn get_chatters<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
    ) -> GetChattersBuilder<'a>;

    /// Gets the broadcaster’s list of custom emotes
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - An ID that identifies the broadcaster whose emotes you want to get.
    ///
    /// # Returns
    ///
    /// Returns a [`EmotesResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     chat::ChatAPI,
    ///     types::BroadcasterId
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_channel_emotes(&BroadcasterId::from("1234"))
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
    /// <https://dev.twitch.tv/docs/api/reference/#get-channel-emotes>
    fn get_channel_emotes(
        &self,
        broadcaster_id: &BroadcasterId,
    ) -> impl Future<Output = Result<EmotesResponse, Error>> + Send;

    /// Gets the list of global emotes
    ///
    /// # Returns
    ///
    /// Returns a [`EmotesResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::chat::ChatAPI;
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_global_emotes()
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
    /// <https://dev.twitch.tv/docs/api/reference/#get-global-emotes>
    fn get_global_emotes(&self) -> impl Future<Output = Result<EmotesResponse, Error>> + Send;

    /// Gets emotes for one or more specified emote sets
    ///
    /// # Arguments
    ///
    /// * `emote_set_ids` - An ID that identifies the emote set to get.
    ///
    /// # Returns
    ///
    /// Returns a [`EmotesResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::chat::ChatAPI;
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_emote_sets(&["e1", "e2"])
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
    /// <https://dev.twitch.tv/docs/api/reference/#get-emote-sets>
    fn get_emote_sets(
        &self,
        emote_set_ids: &[&str],
    ) -> impl Future<Output = Result<EmotesResponse, Error>> + Send;

    /// Gets the broadcaster’s list of custom chat badges
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster whose chat badges you want to get.
    ///
    /// # Returns
    ///
    /// Returns a [`BadgesResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     chat::ChatAPI,
    ///     types::BroadcasterId
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_channel_chat_badges(&BroadcasterId::from("1234"))
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
    /// <https://dev.twitch.tv/docs/api/reference/#get-channel-chat-badges>
    fn get_channel_chat_badges(
        &self,
        broadcaster_id: &BroadcasterId,
    ) -> impl Future<Output = Result<BadgesResponse, Error>> + Send;

    /// Gets Twitch’s list of chat badges
    ///
    /// # Returns
    ///
    /// Returns a [`BadgesResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::chat::ChatAPI;
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_global_chat_badges()
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
    /// <https://dev.twitch.tv/docs/api/reference/#get-global-chat-badges>
    fn get_global_chat_badges(&self) -> impl Future<Output = Result<BadgesResponse, Error>> + Send;

    /// Gets the broadcaster’s chat settings
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster whose chat settings you want to get.
    ///
    /// # Returns
    ///
    /// Returns a [`GetChatSettingsBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     chat::ChatAPI,
    ///     types::{BroadcasterId, ModeratorId}
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_chat_settings(&BroadcasterId::from("1234"))
    ///     .moderator_id(&ModeratorId::from("5678"))
    ///     .send()
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
    /// <https://dev.twitch.tv/docs/api/reference/#get-chat-settings>
    fn get_chat_settings<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
    ) -> GetChatSettingsBuilder<'a>;

    /// Retrieves the active shared chat session for a channel
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The User ID of the channel broadcaster.
    ///
    /// # Returns
    ///
    /// Returns a [`SharedChatSessionResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     chat::ChatAPI,
    ///     types::BroadcasterId
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_shared_chat_session(&BroadcasterId::from("1234"))
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
    /// <https://dev.twitch.tv/docs/api/reference/#get-shared-chat-session>
    fn get_shared_chat_session(
        &self,
        broadcaster_id: &BroadcasterId,
    ) -> impl Future<Output = Result<SharedChatSessionResponse, Error>> + Send;

    /// Retrieves emotes available to the user across all channels
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user.
    ///
    /// # Returns
    ///
    /// Returns a [`GetUserEmotesBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     chat::ChatAPI,
    ///     types::{BroadcasterId, UserId}
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_user_emotes(&UserId::from("5678"))
    ///     .broadcaster_id(&BroadcasterId::from("1234"))
    ///     .after("eyJiI...")
    ///     .send()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `user:read:emotes`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-user-emotes>
    fn get_user_emotes<'a>(&'a self, user_id: &'a UserId) -> GetUserEmotesBuilder<'a>;

    /// Updates the broadcaster’s chat settings
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster whose chat settings you want to update.
    /// * `moderator_id` - The ID of a user that has permission to moderate the broadcaster’s chat room, or the broadcaster’s ID if they’re making the update.
    ///
    /// # Returns
    ///
    /// Returns a [`UpdateChatSettingsBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     chat::ChatAPI,
    ///     types::{BroadcasterId, ModeratorId}
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .update_chat_settings(
    ///         &BroadcasterId::from("1234"),
    ///         &ModeratorId::from("5678"))
    ///     .send()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `moderator:manage:chat_settings`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#update-chat-settings>
    fn update_chat_settings<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
    ) -> UpdateChatSettingsBuilder<'a>;

    /// Sends an announcement to the broadcaster’s chat room
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster that owns the chat room to send the announcement to.
    /// * `moderator_id` - The ID of a user who has permission to moderate the broadcaster’s chat room, or the broadcaster’s ID if they’re sending the announcement.
    /// * `message` - (max 500)
    ///
    /// # Returns
    ///
    /// Returns a [`SendChatAnnouncementBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     chat::{ChatAPI, AnnouncementColor},
    ///     types::{BroadcasterId, ModeratorId}
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .send_chat_announcement(
    ///         &BroadcasterId::from("1234"),
    ///         &ModeratorId::from("5678"),
    ///         "message"
    ///     )
    ///     .color(AnnouncementColor::Blue)
    ///     .send()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `moderator:manage:announcements`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#send-chat-announcement>
    fn send_chat_announcement<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
        message: &'a str,
    ) -> SendChatAnnouncementBuilder<'a>;

    /// Sends a Shoutout to the specified broadcaster
    ///
    /// # Arguments
    ///
    /// * `from_broadcaster_id` - The ID of the broadcaster that’s sending the Shoutout.
    /// * `to_broadcaster_id` - The ID of the broadcaster that’s receiving the Shoutout.
    /// * `moderator_id` - The ID of the broadcaster or a user that is one of the broadcaster’s moderators.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     chat::ChatAPI,
    ///     types::{BroadcasterId, ModeratorId}
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .send_a_shoutout(
    ///         &BroadcasterId::from("1234"),
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
    /// `moderator:manage:shoutouts`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#send-a-shoutout>
    fn send_a_shoutout(
        &self,
        from_broadcaster_id: &BroadcasterId,
        to_broadcaster_id: &BroadcasterId,
        moderator_id: &ModeratorId,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Sends a message to the broadcaster’s chat room
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster whose chat room the message will be sent to.
    /// * `sender_id` - The ID of the user sending the message.
    /// * `message` - (max 500)
    ///
    /// # Returns
    ///
    /// Returns a [`SendChatMessageBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     chat::ChatAPI,
    ///     types::{BroadcasterId, UserId}
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .send_chat_message(
    ///         &BroadcasterId::from("1234"),
    ///         &UserId::from("5678"),
    ///         "message"
    ///     )
    ///     .for_source_only(true)
    ///     .send()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// Requires an app access token or user access token that includes the user:write:chat scope. If app access token used, then additionally requires user:bot scope from chatting user, and either channel:bot scope from broadcaster or moderator status.
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#send-chat-message>
    fn send_chat_message<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        sender_id: &'a UserId,
        message: &'a str,
    ) -> SendChatMessageBuilder<'a>;

    /// Gets the color used for the user’s name in chat
    ///
    /// # Arguments
    ///
    /// * `user_ids` - The ID of the user whose username color you want to get. (max 100)
    ///
    /// # Returns
    ///
    /// Returns a [`UsersColorResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     chat::ChatAPI,
    ///     types::UserId
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_user_chat_color(&[UserId::from("1234")])
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
    /// <https://dev.twitch.tv/docs/api/reference/#get-user-chat-color>
    fn get_user_chat_color(
        &self,
        user_ids: &[UserId],
    ) -> impl Future<Output = Result<UsersColorResponse, Error>> + Send;

    /// Updates the color used for the user’s name in chat
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose chat color you want to update.
    /// * `color` -
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     chat::{ChatAPI, ChatColor},
    ///     types::UserId,
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .update_user_chat_color(&UserId::from("1234"), ChatColor::BlueViolet)
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `No scope required`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#update-user-chat-color>
    fn update_user_chat_color(
        &self,
        user_id: &UserId,
        color: ChatColor,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

impl ChatAPI for Client {
    fn get_chatters<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
    ) -> GetChattersBuilder<'a> {
        GetChattersBuilder::new(self, broadcaster_id, moderator_id)
    }

    async fn get_channel_emotes(
        &self,
        broadcaster_id: &BroadcasterId,
    ) -> Result<EmotesResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().extend([CHAT, EMOTES]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id);

        self.json(self.http_client().get(url)).await
    }

    async fn get_global_emotes(&self) -> Result<EmotesResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([CHAT, EMOTES, GLOBAL]);

        self.json(self.http_client().get(url)).await
    }

    async fn get_emote_sets(&self, emote_set_ids: &[&str]) -> Result<EmotesResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().extend([CHAT, EMOTES, SET]);

        url.query_pairs_mut()
            .extend_pairs(emote_set_ids.iter().map(|id| (EMOTE_SET_ID, id)));

        self.json(self.http_client().get(url)).await
    }

    /// <https://dev.twitch.tv/docs/api/reference/#get-channel-chat-badges>
    async fn get_channel_chat_badges(
        &self,
        broadcaster_id: &BroadcasterId,
    ) -> Result<BadgesResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().extend([CHAT, BADGES]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id);

        self.json(self.http_client().get(url)).await
    }

    async fn get_global_chat_badges(&self) -> Result<BadgesResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([CHAT, BADGES, GLOBAL]);

        self.json(self.http_client().get(url)).await
    }

    fn get_chat_settings<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
    ) -> GetChatSettingsBuilder<'a> {
        GetChatSettingsBuilder::new(self, broadcaster_id)
    }

    async fn get_shared_chat_session(
        &self,
        broadcaster_id: &BroadcasterId,
    ) -> Result<SharedChatSessionResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([SHARED_CHAT, SESSION]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id);

        self.json(self.http_client().get(url)).await
    }

    fn get_user_emotes<'a>(&'a self, user_id: &'a UserId) -> GetUserEmotesBuilder<'a> {
        GetUserEmotesBuilder::new(self, user_id)
    }

    fn update_chat_settings<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
    ) -> UpdateChatSettingsBuilder<'a> {
        UpdateChatSettingsBuilder::new(self, broadcaster_id, moderator_id)
    }

    fn send_chat_announcement<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
        message: &'a str,
    ) -> SendChatAnnouncementBuilder<'a> {
        SendChatAnnouncementBuilder::new(self, broadcaster_id, moderator_id, message)
    }

    async fn send_a_shoutout(
        &self,
        from_broadcaster_id: &BroadcasterId,
        to_broadcaster_id: &BroadcasterId,
        moderator_id: &ModeratorId,
    ) -> Result<(), Error> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().extend([CHAT, SHOUTOUTS]);

        url.query_pairs_mut()
            .append_pair(FROM_BROADCASTER_ID, from_broadcaster_id)
            .append_pair(TO_BROADCASTER_ID, to_broadcaster_id)
            .append_pair(MODERATOR_ID, moderator_id);

        self.no_content(self.http_client().post(url)).await
    }

    fn send_chat_message<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        sender_id: &'a UserId,
        message: &'a str,
    ) -> SendChatMessageBuilder<'a> {
        SendChatMessageBuilder::new(self, broadcaster_id, sender_id, message)
    }

    async fn get_user_chat_color(&self, user_ids: &[UserId]) -> Result<UsersColorResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().extend([CHAT, COLOR]);

        url.query_pairs_mut()
            .extend_pairs(user_ids.iter().map(|id| (USER_ID, id)));

        self.json(self.http_client().get(url)).await
    }

    async fn update_user_chat_color(
        &self,
        user_id: &UserId,
        color: ChatColor,
    ) -> Result<(), Error> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().extend([CHAT, COLOR]);

        url.query_pairs_mut()
            .append_pair(USER_ID, user_id)
            .append_pair(COLOR, color.as_ref());

        self.no_content(self.http_client().put(url)).await
    }
}
