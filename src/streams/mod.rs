mod request;
mod response;
mod types;

pub use request::{CreateStreamMarkerRequest, GetStreamsRequest, StreamMarkerSelector};
pub use response::{
    CreateStreamMarkerResponse, GetStreamMarkersResponse, StreamKeyResponse, StreamsResponse,
};
pub use types::{Marker, Stream, StreamKey, StreamMarker, StreamVideos};

use crate::types::{
    constants::{BROADCASTER_ID, USER_ID},
    BroadcasterId, PaginationQuery, UserId,
};

const STREAMS: &str = "streams";

endpoints! {
    StreamsAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#get-stream-key>
        fn get_stream_key(
            &self,
            broadcaster_id: &BroadcasterId,
        ) -> StreamKeyResponse {
            endpoint_type: GetStreamKey,
            method: GET,
            path: [STREAMS, "key"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#get-streams>
        fn get_streams(
            &self,
            opts: Option<GetStreamsRequest>,
            pagination: Option<PaginationQuery>,
        ) -> StreamsResponse {
            endpoint_type: GetStreams,
            method: GET,
            path: [STREAMS],
            query_params: {
                opt_into_query(opts),
                pagination(pagination)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#get-followed-streams>
        fn get_followed_streams(
            &self,
            user_id: &UserId,
            pagination: Option<PaginationQuery>,
        ) -> StreamsResponse {
            endpoint_type: GetFollowedStreams,
            method: GET,
            path: [STREAMS, "followed"],
            query_params: {
                query(USER_ID, user_id),
                pagination(pagination)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#create-stream-marker>
        fn create_stream_marker(
            &self,
            user_id: &UserId,
            description: Option<&str>,
        ) -> CreateStreamMarkerResponse {
            endpoint_type: CreateStreamMarker,
            method: POST,
            path: [STREAMS, "markers"],
            headers: [json],
            body: CreateStreamMarkerRequest::new(user_id, description).into_json()
        }

        /// <https://dev.twitch.tv/docs/api/reference/#get-stream-markers>
        fn get_stream_markers(
            &self,
            selector: StreamMarkerSelector,
            pagination: Option<PaginationQuery>,
        ) -> GetStreamMarkersResponse {
            endpoint_type: GetStreamMarkers,
            method: GET,
            path: [STREAMS, "markers"],
            query_params: {
                into_query(selector),
                pagination(pagination)
            }
        }
    }
}
