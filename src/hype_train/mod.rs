use asknothingx2_util::api::Method;
use response::HypeTrainResponse;

use crate::{
    request::{EndpointType, TwitchAPIRequest},
    types::{constants::BROADCASTER_ID, BroadcasterId, PaginationQuery},
    TwitchAPI,
};

pub mod response;
pub mod types;

endpoints! {
    HypeTrainAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#get-hype-train-events>
        fn get_hype_train_events(
            &self,
            broadcaster_id: BroadcasterId,
            pagination: Option<PaginationQuery>,
        ) -> HypeTrainResponse {
            endpoint_type: EndpointType::GetHypeTrainEvents,
            method: Method::GET,
            path: ["hypetrain", "events"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                pagination(pagination)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        hype_train::HypeTrainAPI,
        test_utils::TwitchApiTest,
        types::{BroadcasterId, PaginationQuery},
    };

    #[tokio::test]
    pub(crate) async fn get_hype_train_events() {
        let suite = TwitchApiTest::new().await;

        suite.mock_hype_train_success().await;

        let pagination = PaginationQuery::new().first(20);

        let response = suite
            .execute("/hypetrain/events", |api| {
                api.get_hype_train_events(BroadcasterId::new("123456789"), Some(pagination))
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 1);
        assert!(response.pagination.is_none());

        let hype_train = &response.data[0];
        assert_eq!(hype_train.id.as_str(), "hypetrain789");
        assert_eq!(hype_train.event_data.level, 4);
        assert_eq!(hype_train.event_data.total, 6500);
        assert_eq!(hype_train.event_data.goal, 8000);
    }
}
