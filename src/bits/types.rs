#![allow(non_snake_case)]
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
    //pub id: ID,
    pub id: String,
    pub color: String,
    pub images: Images,
    pub can_cheer: bool,
    pub show_in_bits_card: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ID {
    #[serde(rename = "1")]
    One,
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

#[derive(Debug, Serialize, Deserialize)]
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
#[derive(Debug, Serialize, Deserialize)]
pub struct Imagess {
    #[serde(rename(serialize = "1", deserialize = "1"))]
    One: String,
    #[serde(rename(serialize = "1.5", deserialize = "1.5"))]
    OneDotFive: String,
    #[serde(rename(serialize = "2", deserialize = "2"))]
    Two: String,
    #[serde(rename(serialize = "3", deserialize = "3"))]
    Three: String,
    #[serde(rename(serialize = "4", deserialize = "4"))]
    Four: String,
}

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
#[derive(Debug, Deserialize)]
pub enum ProductType {
    BITS_IN_EXTENSION,
}
