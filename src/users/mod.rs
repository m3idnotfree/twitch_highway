use std::sync::Arc;

use asknothingx2_util::oauth::{AccessToken, ClientId};

mod get_user;
pub use get_user::*;

#[derive(Debug)]
pub struct UserAPI {
    access_token: Arc<AccessToken>,
    client_id: Arc<ClientId>,
}

impl UserAPI {
    pub fn new(access_token: &AccessToken, client_id: &ClientId) -> Self {
        Self {
            access_token: Arc::new(access_token.clone()),
            client_id: Arc::new(client_id.clone()),
        }
    }

    pub fn get_users(&self) -> GetUsers {
        GetUsers::new(self.access_token.clone(), self.client_id.clone())
    }
}
