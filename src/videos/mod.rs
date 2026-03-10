mod builder;
mod response;
mod types;

pub use builder::{GetVideosBuilder, VideoSelect};
pub use response::{DeleteVideosResponse, VideosResponse};
pub use types::{MutedSegment, Period, Sort, Type, Video};

use crate::{
    request::TwitchAPIRequest,
    types::{constants::ID, GameId, UserId, VideoId},
    Client,
};

const VIDEOS: &str = "videos";

pub trait VideosAPI {
    /// Gets information about one or more published videos by video IDs
    ///
    /// # Arguments
    ///
    /// * `ids` - A list of IDs that identify the videos you want to get. (max 100)
    ///
    /// # Returns
    ///
    /// Returns a [`GetVideosBuilder`]
    ///
    /// # Example
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     types::VideoId,
    ///     videos::{VideosAPI, Period, Sort, Type},
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let videos = api
    ///     .get_videos_by_ids(&[VideoId::from("85784")])
    ///     .period(Period::Week)
    ///     .sort(Sort::Trending)
    ///     .kind(Type::Highlight)
    ///     .json()
    ///     .await?;
    ///     
    /// # Ok(())
    /// # };
    /// ```
    ///
    /// # Required Scope
    ///
    /// No scope required
    ///
    /// # API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-videos>
    fn get_videos_by_ids<'a>(&'a self, ids: &'a [VideoId]) -> GetVideosBuilder<'a>;

    /// Gets information about one or more published videos by user ID
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose list of videos you want to get.
    ///
    /// # Returns
    ///
    /// Returns a [`GetVideosBuilder`]
    ///
    /// # Example
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     types::UserId,
    ///     videos::{VideosAPI, Period, Sort, Type},
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let videos = api
    ///     .get_videos_by_user_id(&UserId::from("85784"))
    ///     .period(Period::Week)
    ///     .sort(Sort::Trending)
    ///     .kind(Type::Highlight)
    ///     .json()
    ///     .await?;
    ///
    /// # Ok(())
    /// # };
    /// ```
    ///
    /// # Required Scope
    ///
    /// No scope required
    ///
    /// # API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-videos>
    fn get_videos_by_user_id<'a>(&'a self, user_id: &'a UserId) -> GetVideosBuilder<'a>;

    /// Gets information about one or more published videos by game ID
    ///
    /// # Arguments
    ///
    /// * `game_id` - A category or game ID.
    ///
    /// # Returns
    ///
    /// Returns a [`GetVideosBuilder`]
    ///
    /// # Example
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     types::GameId,
    ///     videos::{VideosAPI, Period, Sort, Type},
    /// };
    ///
    /// # async  fn example(api: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let videos = api
    ///     .get_videos_by_game_id(&GameId::from("85784"))
    ///     .period(Period::Week)
    ///     .sort(Sort::Trending)
    ///     .kind(Type::Highlight)
    ///     .json()
    ///     .await?;
    ///
    /// # Ok(())
    /// # };
    /// ```
    ///
    /// # Required Scope
    ///
    /// No scope required
    ///
    /// # API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-videos>
    fn get_videos_by_game_id<'a>(&'a self, game_id: &'a GameId) -> GetVideosBuilder<'a>;

    /// Deletes one or more videos
    ///
    ///
    /// # Arguments
    ///
    /// * `ids` - The list of videos to delete. (max 5 per request)
    ///
    /// # Returns
    /// Returns a [`DeleteVideosResponse`]
    ///
    /// # Example
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway:: {
    ///     types::VideoId,
    ///     videos::VideosAPI,
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .delete_videos(&[VideoId::from("4867"), VideoId::from("9137")])
    ///     .json()
    ///     .await?;
    ///
    /// # Ok(())
    /// # };
    /// ```
    ///
    /// # Required Scope
    ///
    /// `channel:manage:videos`
    ///
    /// # API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#delete-videos>
    fn delete_videos(&self, ids: &[VideoId]) -> TwitchAPIRequest<DeleteVideosResponse>;
}

impl VideosAPI for Client {
    fn get_videos_by_ids<'a>(&'a self, ids: &'a [VideoId]) -> GetVideosBuilder<'a> {
        GetVideosBuilder::ids(self, ids)
    }
    fn get_videos_by_user_id<'a>(&'a self, user_id: &'a UserId) -> GetVideosBuilder<'a> {
        GetVideosBuilder::user_id(self, user_id)
    }
    fn get_videos_by_game_id<'a>(&'a self, game_id: &'a GameId) -> GetVideosBuilder<'a> {
        GetVideosBuilder::game_id(self, game_id)
    }
    fn delete_videos(&self, ids: &[VideoId]) -> TwitchAPIRequest<DeleteVideosResponse> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().extend(&[VIDEOS]);

        let mut query = url.query_pairs_mut();

        query.extend_pairs(ids.iter().map(|id| (ID, id)));

        drop(query);

        TwitchAPIRequest::new(
            crate::request::EndpointType::DeleteVideos,
            url,
            reqwest::Method::DELETE,
            self.default_headers(),
            None,
            self.http_client().clone(),
        )
    }
}
