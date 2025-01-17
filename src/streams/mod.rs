use asknothingx2_util::api::Method;
use request::{CreateStreamMarkerRequest, GetStreamsRequest, StreamMarkerFilter};
use response::{
    CreateStreamMarkerResponse, GetStreamMarkersResponse, StreamKeyResponse, StreamsResponse,
};

use crate::{
    base::TwitchAPIBase,
    types::{
        constants::{BROADCASTER_ID, STREAMS, USER_ID},
        BroadcasterId, PaginationQuery, UserId,
    },
    EmptyBody, EndpointType, TwitchAPI, TwitchAPIRequest,
};

pub mod request;
pub mod response;
pub mod types;

pub trait StreamsAPI: TwitchAPIBase {
    fn get_stream_key(
        &self,
        broadcaster_id: BroadcasterId,
    ) -> TwitchAPIRequest<EmptyBody, StreamKeyResponse>;
    fn get_streams(
        &self,
        opts: Option<GetStreamsRequest>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, StreamsResponse>;
    fn get_followed_streams(
        &self,
        user_id: UserId,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, StreamsResponse>;
    fn create_stream_marker(
        &self,
        user_id: UserId,
        description: Option<String>,
    ) -> TwitchAPIRequest<CreateStreamMarkerRequest, CreateStreamMarkerResponse>;
    fn get_stream_marker(
        &self,
        filter: StreamMarkerFilter,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, GetStreamMarkersResponse>;
}

impl StreamsAPI for TwitchAPI {
    fn get_stream_key(
        &self,
        broadcaster_id: BroadcasterId,
    ) -> TwitchAPIRequest<EmptyBody, StreamKeyResponse> {
        let mut url = self.build_url();
        url.path([STREAMS, "key"])
            .query(BROADCASTER_ID, broadcaster_id);

        TwitchAPIRequest::new(
            EndpointType::GetStreamKey,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn get_streams(
        &self,
        opts: Option<GetStreamsRequest>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, StreamsResponse> {
        let mut url = self.build_url();
        url.path([STREAMS])
            .query_opt_pairs(opts)
            .query_opt_pairs(pagination);

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
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, StreamsResponse> {
        let mut url = self.build_url();
        url.path([STREAMS, "followed"])
            .query(USER_ID, user_id)
            .query_opt_pairs(pagination);

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
        user_id: UserId,
        description: Option<String>,
    ) -> TwitchAPIRequest<CreateStreamMarkerRequest, CreateStreamMarkerResponse> {
        let mut url = self.build_url();
        url.path([STREAMS, "markers"]);

        let mut headers = self.build_headers();
        headers.json();

        TwitchAPIRequest::new(
            EndpointType::CreateStreamMarker,
            url.build(),
            Method::POST,
            headers.build(),
            CreateStreamMarkerRequest {
                user_id,
                description,
            },
        )
    }
    fn get_stream_marker(
        &self,
        filter: StreamMarkerFilter,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, GetStreamMarkersResponse> {
        let mut url = self.build_url();
        url.path([STREAMS, "markers"])
            .query_pairs(filter)
            .query_opt_pairs(pagination);

        TwitchAPIRequest::new(
            EndpointType::GetStreamMarkers,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
}
