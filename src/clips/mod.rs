use crate::{
    base::TwitchAPIBase,
    types::{constants::BROADCASTER_ID, BroadcasterId, PaginationQuery},
    EmptyBody, EndpointType, TwitchAPI, TwitchAPIRequest,
};
use asknothingx2_util::api::Method;
use request::{ClipsFilter, GetClipsRequest};
use response::{ClipsInfoResponse, CreateClipsResponse};

pub mod request;
pub mod response;
pub mod types;

pub trait ClipsAPI: TwitchAPIBase {
    fn create_clip(
        &self,
        broadcaster_id: BroadcasterId,
        has_delay: Option<bool>,
    ) -> TwitchAPIRequest<EmptyBody, CreateClipsResponse>;
    fn get_clips(
        &self,
        clips_filter: ClipsFilter,
        opts: Option<GetClipsRequest>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, ClipsInfoResponse>;
}

impl ClipsAPI for TwitchAPI {
    fn create_clip(
        &self,
        broadcaster_id: BroadcasterId,
        has_delay: Option<bool>,
    ) -> TwitchAPIRequest<EmptyBody, CreateClipsResponse> {
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
    fn get_clips(
        &self,
        clips_filter: ClipsFilter,
        opts: Option<GetClipsRequest>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, ClipsInfoResponse> {
        let mut url = self.build_url();
        url.path(["clips"])
            .query_pairs(clips_filter)
            .query_opt_pairs(opts)
            .query_opt_pairs(pagination);

        TwitchAPIRequest::new(
            EndpointType::GetClips,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
}
