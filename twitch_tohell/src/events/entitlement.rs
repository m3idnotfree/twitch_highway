use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::UserId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropEntitlementGrant {
    pub id: String,
    pub data: Vec<EntitlementData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitlementData {
    pub organization_id: String,
    pub category_id: String,
    pub category_name: String,
    pub campaign_id: String,
    pub user_id: UserId,
    pub user_name: String,
    pub user_login: String,
    pub entitlement_id: String,
    pub benefit_id: String,
    pub created_at: DateTime<FixedOffset>,
}
