use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::types::{BroadcasterId, Cost, Id, UserId};

#[derive(Debug, Serialize, Deserialize)]
pub struct BitsLeaderboard {
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    pub rank: u64,
    pub score: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionTransaction {
    pub id: Id,
    pub timestamp: String,
    pub broadcaster_id: BroadcasterId,
    pub broadcaster_login: String,
    pub broadcaster_name: String,
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    pub product_type: String,
    pub product_data: TransactionProductData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cheermotes {
    pub prefix: String,
    pub tiers: Vec<Tier>,
    #[serde(rename = "type")]
    pub kind: Type,
    pub order: u64,
    pub last_updated: DateTime<FixedOffset>,
    pub is_charitable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tier {
    pub min_bits: u64,
    pub id: TierLevel,
    pub color: String,
    pub images: Images,
    pub can_cheer: bool,
    pub show_in_bits_card: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TierLevel {
    #[serde(rename = "1")]
    One,
    #[serde(rename = "10")]
    Ten,
    #[serde(rename = "100")]
    Hundred,
    #[serde(rename = "500")]
    FiveHundred,
    #[serde(rename = "1000")]
    Thousand,
    #[serde(rename = "5000")]
    FiveThousand,
    #[serde(rename = "10000")]
    TenThousand,
    #[serde(rename = "100000")]
    OneHundredThousand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    GlobalFirstParty,
    GlobalThirdParty,
    ChannelCustom,
    DisplayOnly,
    Sponsored,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Images {
    pub dark: Dark,
    pub light: Light,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dark {
    pub animated: Imagess,
    #[serde(rename = "static")]
    pub static_image: Imagess,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Light {
    pub animated: Imagess,
    #[serde(rename = "static")]
    pub static_image: Imagess,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Imagess {
    #[serde(rename(serialize = "1", deserialize = "1"))]
    pub One: String,
    #[serde(rename(serialize = "1.5", deserialize = "1.5"))]
    pub OneDotFive: String,
    #[serde(rename(serialize = "2", deserialize = "2"))]
    pub Two: String,
    #[serde(rename(serialize = "3", deserialize = "3"))]
    pub Three: String,
    #[serde(rename(serialize = "4", deserialize = "4"))]
    pub Four: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionProductData {
    pub domain: String,
    pub sku: String,
    pub cost: Cost,
    pub inDevelopment: bool,
    pub displayName: String,
    pub expiration: String,
    pub broadcast: bool,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub enum ProductType {
    BITS_IN_EXTENSION,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use chrono::{DateTime, FixedOffset, Timelike};
    use serde_json::json;

    use crate::bits::types::{Cheermotes, Images, TierLevel, TransactionProductData, Type};

    #[test]
    fn tier_level_enum() {
        let tiers = vec![
            (TierLevel::One, "1"),
            (TierLevel::Ten, "10"),
            (TierLevel::Hundred, "100"),
            (TierLevel::FiveHundred, "500"),
            (TierLevel::Thousand, "1000"),
            (TierLevel::FiveThousand, "5000"),
            (TierLevel::TenThousand, "10000"),
            (TierLevel::OneHundredThousand, "100000"),
        ];

        for (tier, expected_value) in tiers {
            let serialized = serde_json::to_string(&tier).unwrap();
            assert_eq!(serialized, format!("\"{}\"", expected_value));

            let deserialized: TierLevel = serde_json::from_str(&serialized).unwrap();
            let re_serialized = serde_json::to_string(&deserialized).unwrap();
            assert_eq!(serialized, re_serialized);
        }
    }

    #[test]
    fn cheermote_type_enum() {
        let types = vec![
            Type::GlobalFirstParty,
            Type::GlobalThirdParty,
            Type::ChannelCustom,
            Type::DisplayOnly,
            Type::Sponsored,
        ];

        for cheermote_type in types {
            let serialized = serde_json::to_string(&cheermote_type).unwrap();
            let deserialized: Type = serde_json::from_str(&serialized).unwrap();
            let re_serialized = serde_json::to_string(&deserialized).unwrap();
            assert_eq!(serialized, re_serialized);
        }
    }

    #[test]
    fn image_structure_naming_conventions() {
        let images_json = json!({
            "dark": {
                "animated": {
                    "1": "https://example.com/1.gif",
                    "1.5": "https://example.com/1.5.gif",
                    "2": "https://example.com/2.gif",
                    "3": "https://example.com/3.gif",
                    "4": "https://example.com/4.gif"
                },
                "static": {
                    "1": "https://example.com/1.png",
                    "1.5": "https://example.com/1.5.png",
                    "2": "https://example.com/2.png",
                    "3": "https://example.com/3.png",
                    "4": "https://example.com/4.png"
                }
            },
            "light": {
                "animated": {
                    "1": "https://example.com/light/1.gif",
                    "1.5": "https://example.com/light/1.5.gif",
                    "2": "https://example.com/light/2.gif",
                    "3": "https://example.com/light/3.gif",
                    "4": "https://example.com/light/4.gif"
                },
                "static": {
                    "1": "https://example.com/light/1.png",
                    "1.5": "https://example.com/light/1.5.png",
                    "2": "https://example.com/light/2.png",
                    "3": "https://example.com/light/3.png",
                    "4": "https://example.com/light/4.png"
                }
            }
        });

        let images: Images = serde_json::from_value(images_json).unwrap();

        assert_eq!(images.dark.animated.One, "https://example.com/1.gif");
        assert_eq!(
            images.dark.animated.OneDotFive,
            "https://example.com/1.5.gif"
        );
        assert_eq!(images.dark.static_image.Two, "https://example.com/2.png");
        assert_eq!(
            images.light.animated.Three,
            "https://example.com/light/3.gif"
        );
        assert_eq!(
            images.light.static_image.Four,
            "https://example.com/light/4.png"
        );
    }

    #[test]
    fn transaction_product_data_naming() {
        let product_data_json = json!({
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
        });

        let product_data: TransactionProductData =
            serde_json::from_value(product_data_json).unwrap();

        assert_eq!(product_data.domain, "twitch-ext");
        assert_eq!(product_data.sku, "test-sku");
        assert_eq!(product_data.cost.amount, 100);
        assert!(!product_data.inDevelopment);
        assert_eq!(product_data.displayName, "Test Product");
        assert!(product_data.broadcast);
    }

    #[test]
    fn date_time_parsing_in_cheermotes() {
        let datetime_str = "2023-12-01T15:30:00Z";
        let parsed = DateTime::<FixedOffset>::from_str(datetime_str).unwrap();

        assert_eq!(parsed.hour(), 15);
        assert_eq!(parsed.minute(), 30);

        let cheermote_json = json!({
            "prefix": "Cheer",
            "tiers": [],
            "type": "global_first_party",
            "order": 1,
            "last_updated": datetime_str,
            "is_charitable": false
        });

        let cheermote: Cheermotes = serde_json::from_value(cheermote_json).unwrap();
        assert_eq!(cheermote.last_updated.hour(), 15);
    }
}
