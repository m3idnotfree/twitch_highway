use asknothingx2_util::serde::{deserialize_empty_object_as_none, serialize_none_as_empty_object};
use serde::{Deserialize, Serialize};

use crate::types::Pagination;

use super::types::{
    BitsProductExtension, ConfigurationSegment, Extension, LiveChannel, SecretData,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigurationSegmentResponse {
    pub data: Vec<ConfigurationSegment>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionLiveChannelsRespnose {
    pub data: Vec<LiveChannel>,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionSecretsResponse {
    pub data: Vec<SecretData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionsResponse {
    pub data: Vec<Extension>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionsBitsProductsResponse {
    pub data: Vec<BitsProductExtension>,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::extensions::response::{
        ConfigurationSegmentResponse, ExtensionLiveChannelsRespnose, ExtensionSecretsResponse,
        ExtensionsBitsProductsResponse,
    };

    #[test]
    fn configuration_segment_response_deserialization() {
        let json_data = json!({
            "data": [
                {
                    "segment": "broadcaster",
                    "broadcaster_id": "123456789",
                    "content": "broadcaster_config_content",
                    "version": "1.0.0"
                },
                {
                    "segment": "global",
                    "content": "global_config_content",
                    "version": "1.1.0"
                }
            ]
        });

        let response: ConfigurationSegmentResponse = serde_json::from_value(json_data).unwrap();

        assert_eq!(response.data.len(), 2);

        let broadcaster_segment = &response.data[0];
        assert_eq!(broadcaster_segment.segment.as_ref(), "broadcaster");
        assert_eq!(
            broadcaster_segment
                .broadcaster_id
                .as_ref()
                .unwrap()
                .as_str(),
            "123456789"
        );
        assert_eq!(broadcaster_segment.content, "broadcaster_config_content");
        assert_eq!(broadcaster_segment.version, "1.0.0");

        let global_segment = &response.data[1];
        assert_eq!(global_segment.segment.as_ref(), "global");
        assert!(global_segment.broadcaster_id.is_none());
        assert_eq!(global_segment.content, "global_config_content");
        assert_eq!(global_segment.version, "1.1.0");
    }

    #[test]
    fn extension_live_channels_response_deserialization() {
        let json_data = json!({
            "data": [
                {
                    "broadcaster_id": "123456789",
                    "broadcaster_name": "TestStreamer1",
                    "game_name": "Just Chatting",
                    "game_id": "509658",
                    "title": "Testing Extension Live"
                },
                {
                    "broadcaster_id": "987654321",
                    "broadcaster_name": "TestStreamer2",
                    "game_name": "League of Legends",
                    "game_id": "21779",
                    "title": "Extension in Action"
                }
            ],
            "pagination": {
                "cursor": "eyJiI..."
            }
        });

        let response: ExtensionLiveChannelsRespnose = serde_json::from_value(json_data).unwrap();

        assert_eq!(response.data.len(), 2);
        assert!(response.pagination.is_some());

        let first_channel = &response.data[0];
        assert_eq!(first_channel.broadcaster_id.as_str(), "123456789");
        assert_eq!(first_channel.broadcaster_name, "TestStreamer1");
        assert_eq!(first_channel.game_name, "Just Chatting");
        assert_eq!(first_channel.game_id, "509658");

        let second_channel = &response.data[1];
        assert_eq!(second_channel.broadcaster_id.as_str(), "987654321");
        assert_eq!(second_channel.game_name, "League of Legends");
        assert_eq!(second_channel.game_id, "21779");
    }

    #[test]
    fn extension_secrets_response_deserialization() {
        let json_data = json!({
            "data": [
                {
                    "format_version": 1,
                    "secrets": [
                        {
                            "content": "current_secret_content",
                            "active_at": "2024-01-15T10:00:00Z",
                            "expires_at": "2024-02-15T10:00:00Z"
                        },
                        {
                            "content": "next_secret_content",
                            "active_at": "2024-02-15T10:00:00Z",
                            "expires_at": "2024-03-15T10:00:00Z"
                        }
                    ]
                }
            ]
        });

        let response: ExtensionSecretsResponse = serde_json::from_value(json_data).unwrap();

        assert_eq!(response.data.len(), 1);

        let secret_data = &response.data[0];
        assert_eq!(secret_data.format_version, 1);
        assert_eq!(secret_data.secrets.len(), 2);

        let first_secret = &secret_data.secrets[0];
        assert_eq!(first_secret.content, "current_secret_content");

        let second_secret = &secret_data.secrets[1];
        assert_eq!(second_secret.content, "next_secret_content");
    }

    #[test]
    fn extensions_bits_products_response_deserialization() {
        let json_data = json!({
            "data": [
                {
                    "sku": "premium_feature_001",
                    "cost": {
                        "amount": 100,
                        "type": "bits"
                    },
                    "display_name": "Premium Feature Pack",
                    "in_development": false,
                    "expiration": "2024-12-31T23:59:59Z",
                    "is_broadcast": true
                },
                {
                    "sku": "special_emote_pack",
                    "cost": {
                        "amount": 250,
                        "type": "bits"
                    },
                    "display_name": "Special Emote Pack",
                    "in_development": true,
                    "expiration": "2025-06-30T23:59:59Z",
                    "is_broadcast": false
                }
            ]
        });

        let response: ExtensionsBitsProductsResponse = serde_json::from_value(json_data).unwrap();

        assert_eq!(response.data.len(), 2);

        let first_product = &response.data[0];
        assert_eq!(first_product.sku, "premium_feature_001");
        assert_eq!(first_product.cost.amount, 100);
        assert_eq!(first_product.display_name, "Premium Feature Pack");
        assert!(!first_product.in_development);
        assert!(first_product.is_broadcast);

        let second_product = &response.data[1];
        assert_eq!(second_product.sku, "special_emote_pack");
        assert_eq!(second_product.cost.amount, 250);
        assert_eq!(second_product.display_name, "Special Emote Pack");
        assert!(second_product.in_development);
        assert!(!second_product.is_broadcast);
    }
}
