use serde::{Deserialize, Serialize};

use crate::IntoRequestBody;

use super::types::{BlockUser, User, UserActiveExtensions, UserExtension};

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersInfoResponse {
    pub data: Vec<User>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockUserListResponse {
    pub data: Vec<BlockUser>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUsersResponse {
    pub data: Vec<User>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserExtensionsResponse {
    pub data: Vec<UserExtension>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserActiveExtensionsResponse {
    pub data: UserActiveExtensions,
}

impl IntoRequestBody for UserActiveExtensionsResponse {
    fn as_body(&self) -> Option<String> {
        Some(serde_json::to_string(self).unwrap())
    }
}
