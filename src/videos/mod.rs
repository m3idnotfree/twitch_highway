use asknothingx2_util::api::Method;
use request::{VideoFilter, VideosRequest};
use response::{DeleteVideosResponse, VideosResponse};

use crate::{
    request::{EmptyBody, EndpointType, TwitchAPIRequest},
    types::{
        constants::{ID, VIDEOS},
        Id, PaginationQuery,
    },
    TwitchAPI,
};

pub mod request;
pub mod response;
pub mod types;

#[cfg_attr(docsrs, doc(cfg(feature = "videos")))]
pub trait VideosAPI {
    /// <https://dev.twitch.tv/docs/api/reference/#get-videos>
    fn get_videos(
        &self,
        video_filter: VideoFilter,
        opts: Option<VideosRequest>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, VideosResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#delete-videos>
    fn delete_videos(&self, id: &[Id]) -> TwitchAPIRequest<EmptyBody, DeleteVideosResponse>;
}

impl VideosAPI for TwitchAPI {
    fn get_videos(
        &self,
        video_filter: VideoFilter,
        opts: Option<VideosRequest>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, VideosResponse> {
        let mut url = self.build_url();
        url.path([VIDEOS]).query_pairs(video_filter);
            .query_pairs(video_filter)
        if let Some(opts) = opts {
            opts.apply_to_url(&mut url);
        }

        TwitchAPIRequest::new(
            EndpointType::GetVideos,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn delete_videos(&self, ids: &[Id]) -> TwitchAPIRequest<EmptyBody, DeleteVideosResponse> {
        let mut url = self.build_url();
        url.path([VIDEOS])
            .query_extend(ids.iter().map(|id| (ID, id)));

        TwitchAPIRequest::new(
            EndpointType::DeleteVideos,
            url.build(),
            Method::DELETE,
            self.build_headers().build(),
            EmptyBody,
        )
    }
}
