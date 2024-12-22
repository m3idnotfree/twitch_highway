use asknothingx2_util::oauth::{AccessToken, ClientId};

mod get_user;
pub use get_user::*;

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
