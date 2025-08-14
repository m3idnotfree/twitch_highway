use asknothingx2_util::api::Method;
use request::StartCommercialBody;
use response::{AdScheduleResponse, SnoozeNextAdResponse, StartCommercialResponse};

use crate::{
    request::{EndpointType, TwitchAPIRequest},
    types::{
        constants::{BROADCASTER_ID, CHANNELS},
        BroadcasterId,
    },
    TwitchAPI,
};

pub mod request;
pub mod response;
pub mod types;

twitch_api_trait! {
    #[cfg_attr(docsrs, doc(cfg(feature = "ads")))]
    trait AdsAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#start-commercial>
        fn start_commercial(
            &self,
            broadcaster_id: BroadcasterId,
            length: u64,
        ) -> StartCommercialResponse;
        /// <https://dev.twitch.tv/docs/api/reference/#get-ad-schedule>
        fn get_ad_schedule(
            &self,
            broadcaster_id: BroadcasterId,
        ) -> AdScheduleResponse;
        /// <https://dev.twitch.tv/docs/api/reference/#snooze-next-ad>
        fn snooze_next_ad(
            &self,
            broadcaster_id: BroadcasterId,
        ) -> SnoozeNextAdResponse;
    }
    impl {
        start_commercial => {
            endpoint_type: EndpointType::StartCommercial,
            method: Method::POST,
            path: [CHANNELS, "commercial"],
            headers: [json],
            body: StartCommercialBody::new(broadcaster_id, length).into_json()
        }
        get_ad_schedule => {
            endpoint_type: EndpointType::GetAdSchedule,
            method: Method::GET,
            path: [CHANNELS, "ads"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id)
            }
        }
        snooze_next_ad => {
            endpoint_type: EndpointType::SnoozeNextAd,
            method: Method::POST,
            path: [CHANNELS, "ads", "schedule", "snooze"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id)
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        ads::AdsAPI,
        test_utils::{assert_datetime, TwitchApiTest},
        types::BroadcasterId,
    };

    #[tokio::test]
    async fn start_commercial_endpoint() {
        let suite = TwitchApiTest::new().await;

        suite.mock_ads_success().await;

        let response = suite
            .execute("/channels/commercial", |api| {
                api.start_commercial(BroadcasterId::new("141981764"), 60)
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].length, 60);
        assert_eq!(response.data[0].message, "");
        assert_eq!(response.data[0].retry_after, 480);
    }

    #[tokio::test]
    async fn get_ad_schedule_endpoint() {
        let suite = TwitchApiTest::new().await;

        suite.mock_ads_success().await;

        let response = suite
            .execute("/channels/ads", |api| {
                api.get_ad_schedule(BroadcasterId::new("123456789"))
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 1);
        let schedule = &response.data[0];
        assert_eq!(schedule.snooze_count, "1");
        assert_eq!(schedule.duration, "60");
        assert_eq!(schedule.preroll_free_time, "90");
        assert!(schedule.snooze_refresh_at.is_some());
        assert!(schedule.next_ad_at.is_some());
        assert!(schedule.last_ad_at.is_some());

        assert_datetime(schedule.next_ad_at.unwrap(), 2023, 8, 1, 23, 8);
        assert_datetime(schedule.last_ad_at.unwrap(), 2023, 8, 1, 23, 8);
        assert_datetime(schedule.snooze_refresh_at.unwrap(), 2023, 8, 1, 23, 8);
    }

    #[tokio::test]
    async fn snooze_next_ad_endpoint() {
        let suite = TwitchApiTest::new().await;

        suite.mock_ads_success().await;

        let response = suite
            .execute("/channels/ads/schedule/snooze", |api| {
                api.snooze_next_ad(BroadcasterId::new("123456789"))
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 1);
        let snooze_data = &response.data[0];
        assert_eq!(snooze_data.snooze_count, "1");
        assert_datetime(snooze_data.next_ad_at, 2023, 8, 1, 23, 8);
        assert_datetime(snooze_data.snooze_refresh_at, 2023, 8, 1, 23, 8);
    }

    #[tokio::test]
    async fn start_commercial_error_response() {
        let helper = TwitchApiTest::new().await;

        helper.mock_ads_failure().await;

        let response = helper
            .execute("/channels/commercial", |api| {
                api.start_commercial(BroadcasterId::new("123456789"), 60)
            })
            .json()
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
