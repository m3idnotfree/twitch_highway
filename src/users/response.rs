use serde::{Deserialize, Serialize};

use super::{BlockUser, User, UserActiveExtensions, UserExtension};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsersInfoResponse {
    pub data: Vec<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockUserListResponse {
    pub data: Vec<BlockUser>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUsersResponse {
    pub data: Vec<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserExtensionsResponse {
    pub data: Vec<UserExtension>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserActiveExtensionsResponse {
    pub data: UserActiveExtensions,
}
