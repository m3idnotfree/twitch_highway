#![allow(non_snake_case)]
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::types::{BroadcasterId, UserId};

#[derive(Debug, Deserialize)]
pub struct BitsLeaderboard {
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    pub rank: u64,
    pub score: u64,
}

#[derive(Debug, Deserialize)]
pub struct ExtensionTransaction {
    pub id: String,
    pub timestamp: String,
    pub broadcaster_id: BroadcasterId,
    pub broadcaster_login: String,
    pub broadcaster_name: String,
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    pub product_type: String,
    pub product_data: ProductData,
}

#[derive(Debug, Deserialize)]
pub struct Cheermotes {
    pub prefix: String,
    pub tiers: Vec<CheermotesTiers>,
    #[serde(rename = "type")]
    pub kind: CheermotesTypes,
    pub order: u64,
    pub last_updated: DateTime<FixedOffset>,
    pub is_charitable: bool,
}

#[derive(Debug, Deserialize)]
pub struct CheermotesTiers {
    pub min_bits: u64,
    pub id: CheermotesID,
    pub color: String,
    pub images: CheermotesImages,
    pub can_cheer: bool,
    pub show_in_bits_card: bool,
}

#[derive(Debug, Deserialize)]
pub enum CheermotesID {
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
pub enum CheermotesTypes {
    GlobalFirstParty,
    GlobalThirdParty,
    ChannelCustom,
    DisplayOnly,
    Sponsored,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheermotesImages {
    pub dark: CheermotesDark,
    pub light: CheermotesLight,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CheermotesDark {
    pub animated: CheermotesImagess,
    #[serde(rename = "static")]
    pub static_image: CheermotesImagess,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CheermotesLight {
    pub animated: CheermotesImagess,
    #[serde(rename = "static")]
    pub static_image: CheermotesImagess,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CheermotesImagess {
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

#[derive(Debug, Deserialize)]
pub struct Cost {
    pub amount: u64,
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Debug, Deserialize)]
pub struct ProductData {
    pub domain: String,
    pub sku: String,
    pub cost: Cost,
    pub inDevelopment: bool,
    pub displayName: String,
    pub expiration: String,
    pub broadcast: bool,
}

#[derive(Debug, Deserialize)]
pub struct ExtensionTransactions {
    pub id: String,
    pub timestamp: DateTime<FixedOffset>,
    pub broadcaster_id: BroadcasterId,
    pub broadcaster_login: String,
    pub broadcaster_name: String,
    pub user_id: UserId,
    pub user_login: String,
    pub product_type: ExtensionTransactionsProductType,
    pub product_data: ExtensionTransactionsProduct,
}

#[derive(Debug, Deserialize)]
pub struct ExtensionTransactionsProduct {
    pub sku: String,
    pub domain: String,
    pub cost: ExtensionTransactionsCost,
    pub inDevelopment: bool,
    pub displayName: String,
    pub expiration: String,
    pub broadcast: bool,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize)]
pub enum ExtensionTransactionsProductType {
    BITS_IN_EXTENSION,
}

#[derive(Debug, Deserialize)]
pub struct ExtensionTransactionsCost {
    pub amount: u64,
    #[serde(rename = "type")]
    pub kind: String,
}
