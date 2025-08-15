use asknothingx2_util::serde::{deserialize_empty_object_as_none, serialize_none_as_empty_object};
use serde::{Deserialize, Serialize};

use crate::types::{DateRange, Pagination};

use super::types::{BitsLeaderboard, Cheermotes, ExtensionTransaction};

#[derive(Debug, Serialize, Deserialize)]
pub struct BitsLeaderboardResponse {
    pub data: Vec<BitsLeaderboard>,
    pub date_range: DateRange,
    pub total: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionTransactionsResponse {
    pub data: Vec<ExtensionTransaction>,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheermotesResponse {
    pub data: Vec<Cheermotes>,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::bits::response::{
        BitsLeaderboardResponse, CheermotesResponse, ExtensionTransactionsResponse,
    };

    #[test]
    fn bits_leaderboard_response_deserialization() {
        let json_data = json!({
            "data": [
                {
                    "user_id": "123456789",
                    "user_login": "testuser",
                    "user_name": "TestUser",
                    "rank": 1,
                    "score": 12543
                },
                {
                    "user_id": "987654321",
                    "user_login": "anotheruser",
                    "user_name": "AnotherUser",
                    "rank": 2,
                    "score": 8932
                }
            ],
            "date_range": {
                "started_at": "2023-12-01T00:00:00Z",
                "ended_at": "2023-12-07T23:59:59Z"
            },
            "total": 2
        });

        let response: BitsLeaderboardResponse = serde_json::from_value(json_data).unwrap();

        assert_eq!(response.data.len(), 2);
        assert_eq!(response.total, 2);

        let first_user = &response.data[0];
        assert_eq!(first_user.user_id.as_str(), "123456789");
        assert_eq!(first_user.user_login, "testuser");
        assert_eq!(first_user.user_name, "TestUser");
        assert_eq!(first_user.rank, 1);
        assert_eq!(first_user.score, 12543);

        let second_user = &response.data[1];
        assert_eq!(second_user.user_id.as_str(), "987654321");
        assert_eq!(second_user.rank, 2);
        assert_eq!(second_user.score, 8932);

        // assert_eq!(response.date_range.started_at, "2023-12-01T00:00:00Z");
        // assert_eq!(response.date_range.ended_at, "2023-12-07T23:59:59Z");
    }

    #[test]
    fn cheermotes_response_deserialization() {
        let json_data = json!({
            "data": [
                {
                    "prefix": "Cheer",
                    "tiers": [
                        {
                            "min_bits": 1,
                            "id": "1",
                            "color": "#979797",
                            "images": {
                                "dark": {
                                    "animated": {
                                        "1": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/animated/1/1.gif",
                                        "1.5": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/animated/1/1.5.gif",
                                        "2": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/animated/1/2.gif",
                                        "3": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/animated/1/3.gif",
                                        "4": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/animated/1/4.gif"
                                    },
                                    "static": {
                                        "1": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/static/1/1.png",
                                        "1.5": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/static/1/1.5.png",
                                        "2": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/static/1/2.png",
                                        "3": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/static/1/3.png",
                                        "4": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/static/1/4.png"
                                    }
                                },
                                "light": {
                                    "animated": {
                                        "1": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/animated/1/1.gif",
                                        "1.5": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/animated/1/1.5.gif",
                                        "2": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/animated/1/2.gif",
                                        "3": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/animated/1/3.gif",
                                        "4": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/animated/1/4.gif"
                                    },
                                    "static": {
                                        "1": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/static/1/1.png",
                                        "1.5": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/static/1/1.5.png",
                                        "2": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/static/1/2.png",
                                        "3": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/static/1/3.png",
                                        "4": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/static/1/4.png"
                                    }
                                }
                            },
                            "can_cheer": true,
                            "show_in_bits_card": true
                        }
                    ],
                    "type": "global_first_party",
                    "order": 1,
                    "last_updated": "2023-12-01T15:30:00Z",
                    "is_charitable": false
                }
            ]
        });

        let response: CheermotesResponse = serde_json::from_value(json_data).unwrap();

        assert_eq!(response.data.len(), 1);

        let cheermote = &response.data[0];
        assert_eq!(cheermote.prefix, "Cheer");
        assert_eq!(cheermote.order, 1);
        assert!(!cheermote.is_charitable);

        assert_eq!(cheermote.tiers.len(), 1);
        let tier = &cheermote.tiers[0];
        assert_eq!(tier.min_bits, 1);
        assert_eq!(tier.color, "#979797");
        assert!(tier.can_cheer);
        assert!(tier.show_in_bits_card);

        assert!(tier.images.dark.animated.One.contains("1.gif"));
        assert!(tier.images.light.static_image.Two.contains("2.png"));
    }

    #[test]
    fn extension_transaction_response_deserialization() {
        let json_data = json!({
            "data": [
                {
                    "id": "trans123456",
                    "timestamp": "2023-12-01T15:30:00Z",
                    "broadcaster_id": "broadcaster123",
                    "broadcaster_login": "testbroadcaster",
                    "broadcaster_name": "TestBroadcaster",
                    "user_id": "user123456",
                    "user_login": "testuser",
                    "user_name": "TestUser",
                    "product_type": "BITS_IN_EXTENSION",
                    "product_data": {
                        "domain": "twitch-ext",
                        "sku": "test-sku",
                        "cost": {
                            "amount": 100,
                            "type": "bits"
                        },
                        "inDevelopment": false,
                        "displayName": "Test Product",
                        "expiration": "2024-12-01T15:30:00Z",
                        "broadcast": true
                    }
                }
            ],
            "pagination": {
                "cursor": "eyJiI..."
            }
        });

        let response: ExtensionTransactionsResponse = serde_json::from_value(json_data).unwrap();

        assert_eq!(response.data.len(), 1);
        assert!(response.pagination.is_some());

        let transaction = &response.data[0];
        assert_eq!(transaction.id.as_str(), "trans123456");
        assert_eq!(transaction.timestamp, "2023-12-01T15:30:00Z");
        assert_eq!(transaction.broadcaster_id.as_str(), "broadcaster123");
        assert_eq!(transaction.user_id.as_str(), "user123456");
        assert_eq!(transaction.product_type, "BITS_IN_EXTENSION");

        let product_data = &transaction.product_data;
        assert_eq!(product_data.domain, "twitch-ext");
        assert_eq!(product_data.sku, "test-sku");
        assert_eq!(product_data.cost.amount, 100);
        assert!(!product_data.inDevelopment);
        assert_eq!(product_data.displayName, "Test Product");
        assert!(product_data.broadcast);
    }
}
