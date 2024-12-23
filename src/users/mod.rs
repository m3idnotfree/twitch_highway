use asknothingx2_util::oauth::{AccessToken, ClientId};
use get_user::GetUsers;

pub mod get_user;

#[derive(Debug)]
pub struct UserAPI {
    access_token: AccessToken,
    client_id: ClientId,
}

impl UserAPI {
    pub fn new(access_token: AccessToken, client_id: ClientId) -> Self {
        Self {
            access_token,
            client_id,
        }
    }

    pub fn get_users(&self) -> GetUsers {
        GetUsers::new(self.access_token.clone(), self.client_id.clone())
    }
}
