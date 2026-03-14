use serde::{Deserialize, Serialize};

use crate::{BroadcasterId, TransactionId, UserId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitsTransactionCreate {
    pub extension_client_id: String,
    pub id: TransactionId,
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    pub protuct: Product,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub name: String,
    pub bits: u64,
    pub sku: String,
    pub in_development: bool,
}
