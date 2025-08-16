use asknothingx2_util::api::Method;
use response::GoalsResponse;

use crate::{
    request::{EndpointType, TwitchAPIRequest},
    types::{constants::BROADCASTER_ID, BroadcasterId},
    TwitchAPI,
};

pub mod response;
pub mod types;

twitch_api_trait! {
    #[cfg_attr(docsrs, doc(cfg(feature = "goals")))]
    trait GoalsAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#get-creator-goals>
        fn get_creator_goals(
            &self,
            broadcaster_id: BroadcasterId,
        ) -> GoalsResponse;
    }
    impl {
        get_creator_goals => {
            endpoint_type: EndpointType::GetCreatorGoals,
            method: Method::GET,
            path: ["goals"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{goals::GoalsAPI, test_utils::TwitchApiTest, types::BroadcasterId};

    #[tokio::test]
    pub(crate) async fn get_creator_goals() {
        let suite = TwitchApiTest::new().await;

        suite.mock_goals_success().await;

        let response = suite
            .execute("/goals", |api| {
                api.get_creator_goals(BroadcasterId::new("123456789"))
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 2);

        let first_goal = &response.data[0];
        assert_eq!(first_goal.id.as_str(), "goal123");
        assert_eq!(first_goal.broadcaster_id.as_str(), "123456789");
        assert_eq!(first_goal.broadcaster_name, "TestStreamer");
        assert_eq!(first_goal.broadcaster_login, "teststreamer");
        assert_eq!(
            first_goal.description,
            "Reach 1000 followers by end of month"
        );
        assert_eq!(first_goal.current_amount, 875);
        assert_eq!(first_goal.target_amount, 1000);

        let second_goal = &response.data[1];
        assert_eq!(second_goal.id.as_str(), "goal456");
        assert_eq!(second_goal.description, "Get 500 new subscribers");
        assert_eq!(second_goal.current_amount, 320);
        assert_eq!(second_goal.target_amount, 500);
    }

    #[tokio::test]
    async fn get_creator_goals_empty_response() {
        let suite = TwitchApiTest::new().await;

        suite.mock_goals_success().await;

        let response = suite
            .execute("/goals", |api| {
                api.get_creator_goals(BroadcasterId::new("987654321"))
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 0);
    }

    #[tokio::test]
    async fn goals_api_error_response() {
        let suite = TwitchApiTest::new().await;

        suite.mock_goals_failure().await;

        let response = suite
            .execute("/goals", |api| {
                api.get_creator_goals(BroadcasterId::new("123456789"))
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
