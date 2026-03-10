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

use crate::{
    request::TwitchAPIRequest,
    types::{
        constants::{BROADCASTER_ID, CHANNELS, EDITORS},
        BroadcasterId, UserId,
    },
    Client,
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
    /// # async fn example(api: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .get_channel_info(&[BroadcasterId::from("1234")])
    ///     .json()
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
    ) -> TwitchAPIRequest<ChannelInfoResponse>;

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
    /// # async fn example(api: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .modify_channel_info(&BroadcasterId::from("1234"))
    ///     .broadcaster_language("")
    ///     .title("")
    ///     .game_id(&GameId::from(""))
    ///     .delay(5)
    ///     .tags(&[""])
    ///     .content_classification_labels(&[ContentClassificationLabel::new(ContentClassificationLabelsID::Gambling, false)])
    ///     .is_branded_content(false)
    ///     .json()
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
    /// # async fn example(api: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .get_channel_editor(&BroadcasterId::from("1234"))
    ///     .json()
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
    ) -> TwitchAPIRequest<ChannelEditorsResponse>;

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
    /// # async fn example(api: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .get_followed_channels(&UserId::from("1234"))
    ///     .broadcaster_id(&BroadcasterId::from("5678"))
    ///     .first(5)
    ///     .after("eyJiI...")
    ///     .json()
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
    /// # async fn example(api: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .get_channel_followers(&BroadcasterId::from("1234"))
    ///     .user_id(&UserId::from("5678"))
    ///     .first(5)
    ///     .after("eyJiI...")
    ///     .json()
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
    fn get_channel_info(
        &self,
        broadcaster_ids: &[BroadcasterId],
    ) -> TwitchAPIRequest<ChannelInfoResponse> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().extend(&[CHANNELS]);
        let mut query = url.query_pairs_mut();

        query.extend_pairs(broadcaster_ids.iter().map(|id| (BROADCASTER_ID, id)));

        drop(query);

        TwitchAPIRequest::new(
            crate::request::EndpointType::GetChanelInformation,
            url,
            reqwest::Method::GET,
            self.default_headers(),
            None,
            self.http_client().clone(),
        )
    }
    fn modify_channel_info<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
    ) -> ModifyChannelInfoBuilder<'a> {
        ModifyChannelInfoBuilder::new(self, broadcaster_id)
    }
    fn get_channel_editor(
        &self,
        broadcaster_id: &BroadcasterId,
    ) -> TwitchAPIRequest<ChannelEditorsResponse> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend(&[CHANNELS, EDITORS]);
        let mut query = url.query_pairs_mut();

        query.append_pair(BROADCASTER_ID, broadcaster_id);

        drop(query);

        TwitchAPIRequest::new(
            crate::request::EndpointType::GetChannelEditors,
            url,
            reqwest::Method::GET,
            self.default_headers(),
            None,
            self.http_client().clone(),
        )
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
