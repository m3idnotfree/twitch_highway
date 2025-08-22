use serde::{Deserialize, Serialize};

use crate::types::{BroadcasterId, Id, UserId};

#[derive(Debug, Serialize, Deserialize)]
pub struct CharityCampaign {
    pub id: Id,
    pub broadcaster_id: BroadcasterId,
    pub broadcaster_login: String,
    pub broadcaster_name: String,
    pub charity_name: String,
    pub charity_description: String,
    pub charity_logo: String,
    pub charity_website: String,
    pub current_amount: Amount,
    pub target_amount: Amount,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharityCampaignDonation {
    pub id: Id,
    pub campaign_id: String,
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    pub amount: Amount,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Amount {
    pub value: u64,
    pub decimal_places: u64,
    pub currency: String,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::{
        charity::types::{Amount, CharityCampaignDonation},
        types::{Id, UserId},
    };

    #[test]
    fn amount() {
        let amount = Amount {
            value: 2550,
            decimal_places: 2,
            currency: "USD".to_string(),
        };

        assert_eq!(amount.value, 2550);
        assert_eq!(amount.decimal_places, 2);
        assert_eq!(amount.currency, "USD");

        let serialized = serde_json::to_string(&amount).unwrap();
        let expected = json!({
            "value": 2550,
            "decimal_places": 2,
            "currency": "USD"
        });

        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&serialized).unwrap(),
            expected
        );

        let deserialized: Amount = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.value, amount.value);
        assert_eq!(deserialized.decimal_places, amount.decimal_places);
        assert_eq!(deserialized.currency, amount.currency);
    }

    #[test]
    fn donation_user_information() {
        let users = vec![
            ("user123", "generous_user", "GenerousUser"),
            ("user456", "kind_viewer", "KindViewer"),
            ("user789", "charity_supporter", "CharitySupporter"),
        ];

        for (user_id, user_login, user_name) in users {
            let donation = CharityCampaignDonation {
                id: Id::from("donation123"),
                campaign_id: "campaign123".to_string(),
                user_id: UserId::from(user_id),
                user_login: user_login.to_string(),
                user_name: user_name.to_string(),
                amount: Amount {
                    value: 2500,
                    decimal_places: 2,
                    currency: "USD".to_string(),
                },
            };

            assert_eq!(donation.user_id.as_str(), user_id);
            assert_eq!(donation.user_login, user_login);
            assert_eq!(donation.user_name, user_name);

            let serialized = serde_json::to_string(&donation).unwrap();
            let deserialized: CharityCampaignDonation = serde_json::from_str(&serialized).unwrap();
            assert_eq!(deserialized.user_login, user_login);
        }
    }
}
