use asknothingx2_util::serde::{deserialize_empty_object_as_none, serialize_none_as_empty_object};
use serde::{Deserialize, Serialize};

use crate::types::Pagination;

use super::types::{CharityCampaign, CharityCampaignDonation};

#[derive(Debug, Serialize, Deserialize)]
pub struct CharityCampaignResponse {
    pub data: Vec<CharityCampaign>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharityCampaignDonationResponse {
    pub data: Vec<CharityCampaignDonation>,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::charity::response::{CharityCampaignDonationResponse, CharityCampaignResponse};

    #[test]
    fn charity_campaign_response_deserialization() {
        let json_data = json!({
            "data": [
                {
                    "id": "campaign123",
                    "broadcaster_id": "123456789",
                    "broadcaster_login": "teststreamer",
                    "broadcaster_name": "TestStreamer",
                    "charity_name": "Extra Life",
                    "charity_description": "Supporting Children's Miracle Network Hospitals to help kids live better lives.",
                    "charity_logo": "https://static-cdn.jtvnw.net/jtv_user_pictures/extralife-logo.png",
                    "charity_website": "https://www.extra-life.org/",
                    "current_amount": {
                        "value": 50000,
                        "decimal_places": 2,
                        "currency": "USD"
                    },
                    "target_amount": {
                        "value": 100000,
                        "decimal_places": 2,
                        "currency": "USD"
                    }
                }
            ]
        });

        let response: CharityCampaignResponse = serde_json::from_value(json_data).unwrap();

        assert_eq!(response.data.len(), 1);

        let campaign = &response.data[0];
        assert_eq!(campaign.id.as_str(), "campaign123");
        assert_eq!(campaign.broadcaster_id.as_str(), "123456789");
        assert_eq!(campaign.broadcaster_login, "teststreamer");
        assert_eq!(campaign.broadcaster_name, "TestStreamer");
        assert_eq!(campaign.charity_name, "Extra Life");
        assert!(campaign
            .charity_description
            .contains("Children's Miracle Network"));
        assert!(campaign.charity_logo.contains("extralife-logo.png"));
        assert_eq!(campaign.charity_website, "https://www.extra-life.org/");

        assert_eq!(campaign.current_amount.value, 50000);
        assert_eq!(campaign.current_amount.decimal_places, 2);
        assert_eq!(campaign.current_amount.currency, "USD");

        assert_eq!(campaign.target_amount.value, 100000);
        assert_eq!(campaign.target_amount.decimal_places, 2);
        assert_eq!(campaign.target_amount.currency, "USD");
    }

    #[test]
    fn charity_campaign_donation_response_deserialization() {
        let json_data = json!({
            "data": [
                {
                    "id": "donation123",
                    "campaign_id": "campaign123",
                    "user_id": "user123",
                    "user_login": "generoususer",
                    "user_name": "GenerousUser",
                    "amount": {
                        "value": 2500,
                        "decimal_places": 2,
                        "currency": "USD"
                    }
                },
                {
                    "id": "donation456",
                    "campaign_id": "campaign123",
                    "user_id": "user456",
                    "user_login": "kindviewer",
                    "user_name": "KindViewer",
                    "amount": {
                        "value": 1000,
                        "decimal_places": 2,
                        "currency": "USD"
                    }
                }
            ],
            "pagination": {
                "cursor": "eyJiI..."
            }
        });

        let response: CharityCampaignDonationResponse = serde_json::from_value(json_data).unwrap();

        assert_eq!(response.data.len(), 2);
        assert!(response.pagination.is_some());

        let first_donation = &response.data[0];
        assert_eq!(first_donation.id.as_str(), "donation123");
        assert_eq!(first_donation.campaign_id, "campaign123");
        assert_eq!(first_donation.user_id.as_str(), "user123");
        assert_eq!(first_donation.user_login, "generoususer");
        assert_eq!(first_donation.user_name, "GenerousUser");
        assert_eq!(first_donation.amount.value, 2500);
        assert_eq!(first_donation.amount.currency, "USD");

        let second_donation = &response.data[1];
        assert_eq!(second_donation.id.as_str(), "donation456");
        assert_eq!(second_donation.user_login, "kindviewer");
        assert_eq!(second_donation.amount.value, 1000);
    }

    #[test]
    fn charity_campaign_empty_data() {
        let json_data = json!({
            "data": []
        });

        let response: CharityCampaignResponse = serde_json::from_value(json_data).unwrap();
        assert_eq!(response.data.len(), 0);
    }

    #[test]
    fn charity_campaign_donation_empty_pagination() {
        let json_data = json!({
            "data": [
                {
                    "id": "donation123",
                    "campaign_id": "campaign123",
                    "user_id": "user123",
                    "user_login": "testuser",
                    "user_name": "TestUser",
                    "amount": {
                        "value": 1000,
                        "decimal_places": 2,
                        "currency": "USD"
                    }
                }
            ],
            "pagination": {}
        });

        let response: CharityCampaignDonationResponse = serde_json::from_value(json_data).unwrap();
        assert_eq!(response.data.len(), 1);
        assert!(response.pagination.is_none());
    }

    #[test]
    fn charity_campaign_donation_no_pagination() {
        let json_data = json!({
            "data": [
                {
                    "id": "donation123",
                    "campaign_id": "campaign123",
                    "user_id": "user123",
                    "user_login": "testuser",
                    "user_name": "TestUser",
                    "amount": {
                        "value": 500,
                        "decimal_places": 2,
                        "currency": "USD"
                    }
                }
            ]
        });

        let response: CharityCampaignDonationResponse = serde_json::from_value(json_data).unwrap();
        assert_eq!(response.data.len(), 1);
        assert!(response.pagination.is_none());
    }
}
