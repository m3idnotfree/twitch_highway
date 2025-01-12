use crate::{
    base::TwitchAPIBase,
    types::{BroadcasterId, BROADCASTER_ID},
    EmptyBody, EndpointType, TwitchAPI, TwitchAPIRequest,
};
use asknothingx2_util::api::Method;
use request::GetClipsRequest;

pub mod request;
pub mod response;
pub mod types;

pub trait ClipsAPI: TwitchAPIBase {
    fn create_clip(
        &self,
        broadcaster_id: BroadcasterId,
        has_delay: Option<bool>,
    ) -> TwitchAPIRequest<EmptyBody>;
    fn get_clips(&self, request: GetClipsRequest) -> TwitchAPIRequest<EmptyBody>;
}

impl ClipsAPI for TwitchAPI {
    fn create_clip(
        &self,
        broadcaster_id: BroadcasterId,
        has_delay: Option<bool>,
    ) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path(["clips"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query_opt("has_delay", has_delay.map(|x| x.to_string()));

        TwitchAPIRequest::new(
            EndpointType::CreateClip,
            url.build(),
            Method::POST,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn get_clips(&self, request: GetClipsRequest) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path(["clips"]).query_pairs(request);

        TwitchAPIRequest::new(
            EndpointType::GetClips,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
}
