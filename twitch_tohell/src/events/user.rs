use serde::{Deserialize, Serialize};

use crate::UserId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAuthorizationGrant {
    pub client_id: String,
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAuthorizationRevoke {
    pub client_id: String,
    pub user_id: UserId,
    pub user_login: Option<String>,
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserUpdate {
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    pub email: Option<String>,
    pub email_verified: bool,
    pub description: String,
}
