use asknothingx2_util::api::Method;
use request::{ClipsSelector, GetClipsRequest};
use response::{ClipsInfoResponse, CreateClipsResponse};

use crate::{
    request::{EndpointType, TwitchAPIRequest},
    types::{constants::BROADCASTER_ID, BroadcasterId, PaginationQuery},
    TwitchAPI,
};

pub mod request;
pub mod response;
pub mod types;

#[cfg_attr(docsrs, doc(cfg(feature = "clips")))]
pub trait ClipsAPI {
    /// <https://dev.twitch.tv/docs/api/reference/#create-clip>
    fn create_clip(
        &self,
        broadcaster_id: BroadcasterId,
        has_delay: Option<bool>,
    ) -> TwitchAPIRequest<CreateClipsResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#get-clips>
    fn get_clips(
        &self,
        clips_selector: ClipsSelector,
        opts: Option<GetClipsRequest>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<ClipsInfoResponse>;
}

impl ClipsAPI for TwitchAPI {
    fn create_clip(
        &self,
        broadcaster_id: BroadcasterId,
        has_delay: Option<bool>,
    ) -> TwitchAPIRequest<CreateClipsResponse> {
        let mut url = self.build_url();
        url.path(["clips"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query_opt("has_delay", has_delay.map(|x| x.to_string()));

        TwitchAPIRequest::new(
            EndpointType::CreateClip,
            url.build(),
            Method::POST,
            self.build_headers().build(),
            None,
        )
    }
    fn get_clips(
        &self,
        clips_selector: ClipsSelector,
        opts: Option<GetClipsRequest>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<ClipsInfoResponse> {
        let mut url = self.build_url();
        url.path(["clips"]);
        clips_selector.apply_to_url(&mut url);

        if let Some(opts) = opts {
            opts.apply_to_url(&mut url);
        }

        if let Some(pagination) = pagination {
            pagination.apply_to_url(&mut url);
        }

        TwitchAPIRequest::new(
            EndpointType::GetClips,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            None,
        )
    }
}
