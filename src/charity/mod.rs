use asknothingx2_util::api::Method;
use response::{CharityCampaignDonationResponse, CharityCampaignResponse};

use crate::{
    request::{EndpointType, TwitchAPIRequest},
    types::{
        constants::{BROADCASTER_ID, CHARITY},
        BroadcasterId, PaginationQuery,
    },
    TwitchAPI,
};

pub mod response;
pub mod types;

twitch_api_trait! {
    #[cfg_attr(docsrs, doc(cfg(feature = "charity")))]
    trait CharityAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#get-charity-campaign>
        fn get_charity_campaign(
            &self,
            broadcaster_id: BroadcasterId,
        ) -> CharityCampaignResponse;
        /// <https://dev.twitch.tv/docs/api/reference/#get-charity-campaign-donations>
        fn get_charity_campaign_donations(
            &self,
            broadcaster_id: BroadcasterId,
            pagination: Option<PaginationQuery>,
        ) -> CharityCampaignDonationResponse;
    }
    impl {
        get_charity_campaign => {
            endpoint_type: EndpointType::GetCharityCampaign,
            method: Method::GET,
            path: [CHARITY, "campaigns"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id)
            }
        }
        get_charity_campaign_donations => {
            endpoint_type: EndpointType::GetCharityCampaignDonations,
            method: Method::GET,
            path: [CHARITY, "donations"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                pagination(pagination)
            }
        }
    }
}

#[cfg(test)]
mod charity_api_tests {
    use crate::{
        charity::CharityAPI,
        test_utils::TwitchApiTest,
        types::{BroadcasterId, PaginationQuery},
    };

    #[tokio::test]
    async fn get_charity_campaign_endpoint() {
        let suite = TwitchApiTest::new().await;

        suite.mock_charity_success().await;

        let response = suite
            .execute("/charity/campaigns", |api| {
                api.get_charity_campaign(BroadcasterId::new("123456789"))
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 1);
        let campaign = &response.data[0];
        assert_eq!(campaign.id.as_str(), "campaign123");
        assert_eq!(campaign.broadcaster_id.as_str(), "123456789");
        assert_eq!(campaign.charity_name, "Doctors Without Borders");
        assert_eq!(campaign.current_amount.value, 125000);
        assert_eq!(campaign.target_amount.value, 200000);
    }

    #[tokio::test]
    async fn get_charity_campaign_donations_endpoint() {
        let suite = TwitchApiTest::new().await;

        suite.mock_charity_success().await;

        let pagination = PaginationQuery::new().first(20);

        let response = suite
            .execute("/charity/donations", |api| {
                api.get_charity_campaign_donations(
                    BroadcasterId::new("123456789"),
                    Some(pagination),
                )
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 2);
        assert!(response.pagination.is_some());

        let first_donation = &response.data[0];
        assert_eq!(first_donation.id.as_str(), "donation001");
        assert_eq!(first_donation.user_login, "generous_donor");
        assert_eq!(first_donation.amount.value, 5000);

        let second_donation = &response.data[1];
        assert_eq!(second_donation.id.as_str(), "donation002");
        assert_eq!(second_donation.amount.value, 2500);
    }

    #[tokio::test]
    async fn get_charity_campaign_donations_no_pagination() {
        let suite = TwitchApiTest::new().await;

        suite.mock_charity_extra().await;

        let response = suite
            .execute("/charity/donations", |api| {
                api.get_charity_campaign_donations(BroadcasterId::new("123456789"), None)
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 1);
        assert!(response.pagination.is_none());
        assert_eq!(response.data[0].amount.value, 10000);
    }

    #[tokio::test]
    async fn charity_api_error_response() {
        let suite = TwitchApiTest::new().await;

        suite.mock_charity_failure().await;

        let response = suite
            .execute("/charity/campaigns", |api| {
                api.get_charity_campaign(BroadcasterId::new("123456789"))
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
