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
    /// - `&UserId`, `&GameId`, or `&[VideoId]` (slice, array, or Vec)
    ///
    /// See <https://dev.twitch.tv/docs/api/reference/#get-videos>
    fn get_videos<'a>(&'a self, select: impl Into<VideoSelect<'a>>) -> GetVideos<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#delete-videos>
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
