use asknothingx2_util::api::Method;
use request::{VideoSelector, VideosRequest};
use response::{DeleteVideosResponse, VideosResponse};

use crate::{
    request::{EndpointType, TwitchAPIRequest},
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
        video_selector: VideoSelector,
        opts: Option<VideosRequest>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<VideosResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#delete-videos>
    fn delete_videos(&self, id: &[Id]) -> TwitchAPIRequest<DeleteVideosResponse>;
}

impl VideosAPI for TwitchAPI {
    fn get_videos(
        &self,
        video_selector: VideoSelector,
        opts: Option<VideosRequest>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<VideosResponse> {
        let mut url = self.build_url();
        url.path([VIDEOS]);
        video_selector.apply_to_url(&mut url);

        if let Some(opts) = opts {
            opts.apply_to_url(&mut url);
        }

        if let Some(pagination) = pagination {
            pagination.apply_to_url(&mut url);
        }

        TwitchAPIRequest::new(
            EndpointType::GetVideos,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            None,
        )
    }
    fn delete_videos(&self, ids: &[Id]) -> TwitchAPIRequest<DeleteVideosResponse> {
        let mut url = self.build_url();
        url.path([VIDEOS])
            .query_extend(ids.iter().map(|id| (ID, id)));

        TwitchAPIRequest::new(
            EndpointType::DeleteVideos,
            url.build(),
            Method::DELETE,
            self.build_headers().build(),
            None,
        )
    }
}
