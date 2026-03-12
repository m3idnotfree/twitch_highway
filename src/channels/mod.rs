mod builder;
mod response;
mod types;

pub use builder::{GetChannelFollowersRequest, GetFollowedChannels, ModifyChannelInfoBuilder};
pub use response::{
    ChannelEditorsResponse, ChannelFollowersResponse, ChannelInfoResponse,
    FollowerdChannelsResponse,
};
pub use types::{
    ChannelEditor, ChannelFollower, ChannelInfo, ContentClassificationLabel,
    ContentClassificationLabelsID, FollowedChannel,
};

use std::future::Future;

use crate::{
    types::{
        constants::{BROADCASTER_ID, CHANNELS, EDITORS},
        BroadcasterId, UserId,
    },
    Client, Error,
};

pub trait ChannelsAPI {
    /// Gets information about one or more channels
    ///
    /// # Arguments
    ///
    /// * `broadcaster_ids` - The ID of the broadcaster whose channel you want to get.
    ///
    /// # Returns
    ///
    /// Returns a [`ChannelInfoResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     channels::ChannelsAPI,
    ///     types::BroadcasterId
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_channel_info(&[BroadcasterId::from("1234")])
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
    /// <https://dev.twitch.tv/docs/api/reference/#get-channel-information>
    fn get_channel_info(
        &self,
        broadcaster_ids: &[BroadcasterId],
    ) -> impl Future<Output = Result<ChannelInfoResponse, Error>> + Send;

    /// Updates a channel’s properties
    ///
    /// # Returns
    ///
    /// Returns a [`ModifyChannelInfoBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     channels::{ChannelsAPI, ContentClassificationLabel, ContentClassificationLabelsID},
    ///     types::{BroadcasterId, GameId}
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .modify_channel_info(&BroadcasterId::from("1234"))
    ///     .broadcaster_language("")
    ///     .title("")
    ///     .game_id(&GameId::from(""))
    ///     .delay(5)
    ///     .tags(&[""])
    ///     .content_classification_labels(&[ContentClassificationLabel::new(ContentClassificationLabelsID::Gambling, false)])
    ///     .is_branded_content(false)
    ///     .send()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `channel:manage:broadcast`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#modify-channel-information>
    fn modify_channel_info<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
    ) -> ModifyChannelInfoBuilder<'a>;

    /// Gets the broadcaster’s list editors
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster that owns the channel.
    ///
    /// # Returns
    ///
    /// Returns a [`ChannelEditorsResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     channels::ChannelsAPI,
    ///     types::BroadcasterId
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_channel_editor(&BroadcasterId::from("1234"))
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `channel:read:editors`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-channel-editors>
    fn get_channel_editor(
        &self,
        broadcaster_id: &BroadcasterId,
    ) -> impl Future<Output = Result<ChannelEditorsResponse, Error>> + Send;

    /// Gets a list of broadcasters that the specified user follows
    ///
    /// # Returns
    ///
    /// Returns a [`FollowerdChannelsResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     channels::ChannelsAPI,
    ///     types::{BroadcasterId, UserId}
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_followed_channels(&UserId::from("1234"))
    ///     .broadcaster_id(&BroadcasterId::from("5678"))
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
    /// `user:read:follows`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-followed-channels>
    fn get_followed_channels<'a>(&'a self, user_id: &'a UserId) -> GetFollowedChannels<'a>;

    /// Gets a list of users that follow the specified broadcaster
    ///
    /// # Returns
    ///
    /// Returns a [`ChannelFollowersResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     channels::ChannelsAPI,
    ///     types::{BroadcasterId,UserId}
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_channel_followers(&BroadcasterId::from("1234"))
    ///     .user_id(&UserId::from("5678"))
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
    /// `moderator:read:followers`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-channel-followers>
    fn get_channel_followers<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
    ) -> GetChannelFollowersRequest<'a>;
}

impl ChannelsAPI for Client {
    async fn get_channel_info(
        &self,
        broadcaster_ids: &[BroadcasterId],
    ) -> Result<ChannelInfoResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().push(CHANNELS);

        url.query_pairs_mut()
            .extend_pairs(broadcaster_ids.iter().map(|id| (BROADCASTER_ID, id)));

        self.json(self.http_client().get(url)).await
    }

    fn modify_channel_info<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
    ) -> ModifyChannelInfoBuilder<'a> {
        ModifyChannelInfoBuilder::new(self, broadcaster_id)
    }

    async fn get_channel_editor(
        &self,
        broadcaster_id: &BroadcasterId,
    ) -> Result<ChannelEditorsResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().extend([CHANNELS, EDITORS]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id);

        self.json(self.http_client().get(url)).await
    }

    fn get_followed_channels<'a>(&'a self, user_id: &'a UserId) -> GetFollowedChannels<'a> {
        GetFollowedChannels::new(self, user_id)
    }

    fn get_channel_followers<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
    ) -> GetChannelFollowersRequest<'a> {
        GetChannelFollowersRequest::new(self, broadcaster_id)
    }
}
