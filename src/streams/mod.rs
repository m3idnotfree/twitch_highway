use asknothingx2_util::api::Method;
use request::{CreateStreamMarkerRequest, GetStreamMarkerRequest, GetStreamsRequest};

use crate::{
    base::TwitchAPIBase,
    types::{BroadcasterId, UserId, AFTER, BROADCASTER_ID, FIRST, USER_ID},
    EmptyBody, EndpointType, TwitchAPI, TwitchAPIRequest,
};

pub mod request;
pub mod response;
pub mod types;

pub trait StreamsAPI: TwitchAPIBase {
    fn get_stream_key(&self, broadcaster_id: BroadcasterId) -> TwitchAPIRequest<EmptyBody>;
    fn get_streams(&self, request: GetStreamsRequest) -> TwitchAPIRequest<EmptyBody>;
    fn get_followed_streams(
        &self,
        user_id: UserId,
        first: Option<u64>,
        after: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody>;
    fn create_stream_marker(
        &self,
        request: CreateStreamMarkerRequest,
    ) -> TwitchAPIRequest<CreateStreamMarkerRequest>;
    fn get_stream_marker(&self, request: GetStreamMarkerRequest) -> TwitchAPIRequest<EmptyBody>;
}

impl StreamsAPI for TwitchAPI {
    fn get_stream_key(&self, broadcaster_id: BroadcasterId) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path(["streams", "key"])
            .query(BROADCASTER_ID, broadcaster_id);

        TwitchAPIRequest::new(
            EndpointType::GetStreamKey,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn get_streams(&self, request: GetStreamsRequest) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path(["streams"]).query_pairs(request);

        TwitchAPIRequest::new(
            EndpointType::GetStreams,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn get_followed_streams(
        &self,
        user_id: UserId,
        first: Option<u64>,
        after: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path(["streams", "followed"])
            .query(USER_ID, user_id)
            .query_opt(FIRST, first.map(|x| x.to_string()))
            .query_opt(AFTER, after);

        TwitchAPIRequest::new(
            EndpointType::GetFollowedStreams,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn create_stream_marker(
        &self,
        request: CreateStreamMarkerRequest,
    ) -> TwitchAPIRequest<CreateStreamMarkerRequest> {
        let mut url = self.build_url();
        url.path(["streams", "markers"]);

        let mut headers = self.build_headers();
        headers.json();

        TwitchAPIRequest::new(
            EndpointType::CreateStreamMarker,
            url.build(),
            Method::POST,
            headers.build(),
            request,
        )
    }
    fn get_stream_marker(&self, request: GetStreamMarkerRequest) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path(["streams", "markers"]).query_pairs(request);

        TwitchAPIRequest::new(
            EndpointType::GetStreamMarkers,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
}
