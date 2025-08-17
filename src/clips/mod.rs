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

endpoints! {
    ClipsAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#create-clip>
        fn create_clip(
            &self,
            broadcaster_id: BroadcasterId,
            has_delay: Option<bool>,
        ) -> CreateClipsResponse {
            endpoint_type: EndpointType::CreateClip,
            method: Method::POST,
            path: ["clips"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                opt("has_delay", has_delay.map(|x| x.to_string()))
            }
        }
        /// <https://dev.twitch.tv/docs/api/reference/#get-clips>
        fn get_clips(
            &self,
            clips_selector: ClipsSelector,
            opts: Option<GetClipsRequest>,
            pagination: Option<PaginationQuery>,
        ) -> ClipsInfoResponse {
            endpoint_type: EndpointType::GetClips,
            method: Method::GET,
            path: ["clips"],
            query_params: {
                into_query(clips_selector),
                opt_into_query(opts),
                pagination(pagination)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        clips::{
            request::{ClipsSelector, GetClipsRequest},
            ClipsAPI,
        },
        test_utils::TwitchApiTest,
        types::{BroadcasterId, GameId, Id, PaginationQuery},
    };

    #[tokio::test]
    pub(crate) async fn create_clip() {
        let suite = TwitchApiTest::new().await;

        suite.mock_clips_success().await;

        let response = suite
            .execute("/clips", |api| {
                api.create_clip(BroadcasterId::new("123456789"), Some(false))
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 1);

        let created_clip = &response.data[0];
        assert_eq!(created_clip.id.as_str(), "FantasticClip123");
        assert!(created_clip.edit_url.contains("FantasticClip123"));
    }

    #[tokio::test]
    async fn create_clip_endpoint_no_delay() {
        let suite = TwitchApiTest::new().await;

        suite.mock_clips_success().await;

        let response = suite
            .execute("/clips", |api| {
                api.create_clip(BroadcasterId::new("987654321"), None)
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].id.as_str(), "InstantClip456");
    }

    #[tokio::test]
    pub(crate) async fn get_clips() {
        let suite = TwitchApiTest::new().await;

        suite.mock_clips_success().await;

        let selector = ClipsSelector::by_broadcaster_id(BroadcasterId::new("123456789"));
        let pagination = PaginationQuery::new().first(20);

        let response = suite
            .execute("/clips", |api| {
                api.get_clips(selector, None, Some(pagination))
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 1);
        assert!(response.pagination.is_some());

        let clip = &response.data[0];
        assert_eq!(clip.id.as_str(), "BroadcasterClip123");
        assert_eq!(clip.broadcaster_id.as_str(), "123456789");
        assert_eq!(clip.broadcaster_name, "MainStreamer");
        assert_eq!(clip.title, "Best Moment Ever");
        assert_eq!(clip.view_count, 10000);
        assert_eq!(clip.duration, 60.0);
        assert!(clip.is_featured);
    }

    #[tokio::test]
    async fn get_clips_by_game_id_endpoint() {
        let suite = TwitchApiTest::new().await;

        suite.mock_clips_success().await;

        let selector = ClipsSelector::by_game_id(GameId::new("21779"));

        let response = suite
            .execute("/clips", |api| api.get_clips(selector, None, None))
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 1);
        assert!(response.pagination.is_none());

        let clip = &response.data[0];
        assert_eq!(clip.id.as_str(), "GameClip789");
        assert_eq!(clip.game_id, "21779");
        assert_eq!(clip.broadcaster_name, "GameMaster");
        assert_eq!(clip.title, "Incredible Play");
        assert_eq!(clip.view_count, 7500);
    }

    #[tokio::test]
    async fn get_clips_by_ids_endpoint() {
        let suite = TwitchApiTest::new().await;

        suite.mock_clips_success().await;

        let clip_ids = vec![Id::new("SpecificClip1"), Id::new("SpecificClip2")];
        let selector = ClipsSelector::by_ids(&clip_ids);

        let response = suite
            .execute("/clips", |api| api.get_clips(selector, None, None))
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 2);

        let first_clip = &response.data[0];
        assert_eq!(first_clip.id.as_str(), "SpecificClip1");
        assert_eq!(first_clip.title, "First Specific Clip");
        assert_eq!(first_clip.view_count, 500);
        assert!(!first_clip.is_featured);

        let second_clip = &response.data[1];
        assert_eq!(second_clip.id.as_str(), "SpecificClip2");
        assert_eq!(second_clip.title, "Second Specific Clip");
        assert_eq!(second_clip.view_count, 750);
        assert!(second_clip.is_featured);
    }

    #[tokio::test]
    async fn get_clips_with_date_filter_endpoint() {
        let suite = TwitchApiTest::new().await;

        suite.mock_clips_success().await;

        let selector = ClipsSelector::by_broadcaster_id(BroadcasterId::new("123456789"));
        let request = GetClipsRequest::new()
            .started_at("2024-01-01T00:00:00Z")
            .ended_at("2024-01-31T23:59:59Z")
            .is_featured(true);

        let response = suite
            .execute("/clips", |api| api.get_clips(selector, Some(request), None))
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 1);

        let clip = &response.data[0];
        assert_eq!(clip.id.as_str(), "RecentClip123");
        assert_eq!(clip.title, "Recent Amazing Play");
        assert!(clip.is_featured);
        assert_eq!(clip.view_count, 3000);
    }

    #[tokio::test]
    async fn clips_api_error_response() {
        let suite = TwitchApiTest::new().await;

        suite.mock_clips_failure().await;

        let response = suite
            .execute("/clips", |api| {
                api.create_clip(BroadcasterId::new("123456789"), None)
            })
            .send()
            .await;

        match response {
            Ok(response) => {
                panic!("Expected Error, got: {response:?}")
            }
            Err(e) => {
                assert!(e.is_api());
            }
        }
    }
}
