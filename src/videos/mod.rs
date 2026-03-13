mod builder;
mod response;
mod types;

pub use builder::GetVideos;
pub use response::{DeleteVideosResponse, VideosResponse};
pub use types::{MutedSegment, Period, Sort, Type, Video};

use builder::VideoSelect;

use std::future::Future;

use crate::{
    types::{
        constants::{ID, VIDEOS},
        VideoId,
    },
    Client, Error,
};

pub trait VideosAPI {
    /// Gets information about one or more published videos
    ///
    /// # Arguments
    ///
    /// * `select` - The filter to use. Pass a [UserId](crate::types::UserId), [GameId](crate::types::GameId), or `&[`[`VideoId`]`]`.
    ///
    /// # Returns
    ///
    /// Returns a [`GetVideosBuilder`]
    ///
    /// # Example
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     types::{GameId, VideoId, UserId},
    ///     videos::{Period, VideosAPI},
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// // By user ID
    /// let videos = api
    ///     .get_videos(&UserId::from("85784"))
    ///     .send()
    ///     .await?;
    ///
    /// // By game ID
    /// let videos = api
    ///     .get_videos(&GameId::from("85784"))
    ///     .period(Period::Week)
    ///     .send()
    ///     .await?;
    ///
    /// // By video IDs
    /// let ids = vec![VideoId::from("85784")];
    /// let videos = api
    ///     .get_videos(&ids)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// No scope required
    ///
    /// # API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-videos>
    fn get_videos<'a>(&'a self, select: impl Into<VideoSelect<'a>>) -> GetVideos<'a>;

    /// Deletes one or more videos
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
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .delete_videos(&[VideoId::from("4867"), VideoId::from("9137")])
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
    fn delete_videos(
        &self,
        ids: &[VideoId],
    ) -> impl Future<Output = Result<DeleteVideosResponse, Error>> + Send;
}

impl VideosAPI for Client {
    fn get_videos<'a>(&'a self, select: impl Into<VideoSelect<'a>>) -> GetVideos<'a> {
        GetVideos::new(self, select)
    }

    async fn delete_videos(&self, ids: &[VideoId]) -> Result<DeleteVideosResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().push(VIDEOS);

        url.query_pairs_mut()
            .extend_pairs(ids.iter().map(|id| (ID, id)));

        self.json(self.http_client().delete(url)).await
    }
}
