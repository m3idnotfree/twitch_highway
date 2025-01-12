use asknothingx2_util::api::Method;
use request::VideosRequest;

use crate::{
    base::TwitchAPIBase,
    types::{Id, ID},
    EmptyBody, EndpointType, TwitchAPI, TwitchAPIRequest,
};

pub mod request;
pub mod response;
pub mod types;

pub trait VideosAPI: TwitchAPIBase {
    fn get_videos(&self, request: VideosRequest) -> TwitchAPIRequest<EmptyBody>;
    fn delete_videos(&self, id: &[Id]) -> TwitchAPIRequest<EmptyBody>;
}

impl VideosAPI for TwitchAPI {
    fn get_videos(&self, request: VideosRequest) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path(["videos"]).query_pairs(request);

        TwitchAPIRequest::new(
            EndpointType::GetVideos,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn delete_videos(&self, ids: &[Id]) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path(["videos"])
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
