mod builder;
mod response;
mod types;

pub use builder::{CreateClipBuilder, GetClipsBuilder};
pub use response::{ClipsInfoResponse, CreateClipsResponse};
pub use types::{Clip, CreateClip};

use crate::{
    types::{BroadcasterId, ClipId, GameId},
    TwitchAPI,
};

pub trait ClipsAPI {
    /// Creates a clip from the broadcaster’s stream
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster whose stream you want to create a clip from.
    ///
    /// # Returns
    ///
    /// Returns a [`CreateClipBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::TwitchAPI;
    /// use twitch_highway::{
    ///     clips::ClipsAPI,
    ///     types::BroadcasterId
    /// };
    ///
    /// # async fn example(api: TwitchAPI) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .create_clip(&BroadcasterId::from("1234"))
    ///     .has_delay(true)
    ///     .json()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `clips:edit`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#create-clip>
    fn create_clip<'a>(&'a self, broadcaster_id: &'a BroadcasterId) -> CreateClipBuilder<'a>;

    /// Gets one or more video clips that were captured from streams by broadcaster ID
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - An ID that identifies the broadcaster whose video clips you want to get.
    ///
    /// # Returns
    ///
    /// Returns a [`GetClipsBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::TwitchAPI;
    /// use std::str::FromStr;
    /// use twitch_highway::{
    ///     clips::ClipsAPI,
    ///     types::BroadcasterId,
    /// };
    ///
    /// # async fn example(api: TwitchAPI) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .get_clips_by_broadcaster_id(&BroadcasterId::from("1234"))
    ///     .started_at(&"2018-01-01T00:00:00Z".parse().unwrap())
    ///     .ended_at(&"2018-03-01T00:00:00Z".parse().unwrap())
    ///     .is_featured(true)
    ///     .first(50)
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
    /// No scope required
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-clips>
    fn get_clips_by_broadcaster_id<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
    ) -> GetClipsBuilder<'a>;

    /// Gets one or more video clips that were captured from streams by game ID
    ///
    /// # Arguments
    ///
    /// * `game_id` - An ID that identifies the game whose clips you want to get.
    ///
    /// # Returns
    ///
    /// Returns a [`GetClipsBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::TwitchAPI;
    /// use std::str::FromStr;
    /// use twitch_highway::{
    ///     clips::ClipsAPI,
    ///     types::GameId,
    /// };
    ///
    /// # async fn example(api: TwitchAPI) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .get_clips_by_game_id(&GameId::from("1234"))
    ///     .started_at(&"2018-01-01T00:00:00Z".parse().unwrap())
    ///     .ended_at(&"2018-03-01T00:00:00Z".parse().unwrap())
    ///     .is_featured(true)
    ///     .first(50)
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
    /// No scope required
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-clips>
    fn get_clips_by_game_id<'a>(&'a self, game_id: &'a GameId) -> GetClipsBuilder<'a>;

    /// Gets one or more video clips that were captured from streams by clip IDs
    ///
    /// # Arguments
    ///
    /// * `ids` - An ID that identifies the clip to get. (max 100)
    ///
    /// # Returns
    ///
    /// Returns a [`GetClipsBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::TwitchAPI;
    /// use std::str::FromStr;
    /// use twitch_highway::{
    ///     clips::ClipsAPI,
    ///     types::ClipId
    /// };
    ///
    /// # async fn example(api: TwitchAPI) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .get_clips_by_ids(&[ClipId::from("1234"), ClipId::from("5678")])
    ///     .started_at(&"2018-01-01T00:00:00Z".parse().unwrap())
    ///     .ended_at(&"2018-03-01T00:00:00Z".parse().unwrap())
    ///     .is_featured(true)
    ///     .first(50)
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
    /// No scope required
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-clips>
    fn get_clips_by_ids<'a>(&'a self, ids: &'a [ClipId]) -> GetClipsBuilder<'a>;
}

impl ClipsAPI for TwitchAPI {
    fn create_clip<'a>(&'a self, broadcaster_id: &'a BroadcasterId) -> CreateClipBuilder<'a> {
        CreateClipBuilder::new(self, broadcaster_id)
    }
    fn get_clips_by_broadcaster_id<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
    ) -> GetClipsBuilder<'a> {
        GetClipsBuilder::by_broadcaster_id(self, broadcaster_id)
    }
    fn get_clips_by_game_id<'a>(&'a self, game_id: &'a GameId) -> GetClipsBuilder<'a> {
        GetClipsBuilder::by_game_id(self, game_id)
    }
    fn get_clips_by_ids<'a>(&'a self, ids: &'a [ClipId]) -> GetClipsBuilder<'a> {
        GetClipsBuilder::by_ids(self, ids)
    }
}
