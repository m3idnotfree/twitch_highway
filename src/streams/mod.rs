pub mod request;
pub mod response;
pub mod types;

use request::{CreateStreamMarkerRequest, GetStreamsRequest, StreamMarkerSelector};
use response::{
    CreateStreamMarkerResponse, GetStreamMarkersResponse, StreamKeyResponse, StreamsResponse,
};

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

#[cfg(test)]
mod tests {
    use crate::{
        streams::{
            request::{GetStreamsRequest, StreamMarkerSelector},
            StreamsAPI,
        },
        test_utils::TwitchApiTest,
        types::{BroadcasterId, PaginationQuery, UserId},
    };

    api_test!(get_stream_key, [&BroadcasterId::from("141981764")]);
    api_test!(get_streams, [None, None]);
    api_test!(get_followed_streams, [&UserId::from("141981764"), None]);
    api_test!(
        create_stream_marker,
        [&UserId::from("123"), Some("hello, this is a marker!")]
    );
    api_test!(
        get_stream_markers,
        [
            StreamMarkerSelector::by_user_id(&UserId::from("123")),
            Some(PaginationQuery::new().first(5))
        ]
    );

    #[tokio::test]
    pub(crate) async fn get_streams_2() {
        let suite = TwitchApiTest::new().await;

        suite.get_streams_2().await;

        let _ = suite
            .execute(|api| {
                api.get_streams(
                    Some(GetStreamsRequest::new().user_login(&["cohhcarnage", "lana_lux"])),
                    None,
                )
            })
            .json()
            .await
            .unwrap();
    }
}
