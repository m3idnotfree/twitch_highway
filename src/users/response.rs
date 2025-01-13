use serde::{Deserialize, Serialize};

use super::types::{BlockUser, User, UserExtension};

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersInfoResponse {
    pub data: Vec<User>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockUserListResponse {
    data: Vec<BlockUser>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUsersResponse {
    pub data: Vec<User>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserExtensionsResponse {
    data: Vec<UserExtension>,
}

//#[derive(Debug, Serialize, Deserialize)]
//pub struct UserActiveExtensionsData {
//    data: Vec<UserActiveExtensions>,
//}

//impl RequestBody for UserActiveExtensionsData {
//    fn as_body(&self) -> Option<String> {
//        Some(serde_json::to_string(self).unwrap())
//    }
//}
