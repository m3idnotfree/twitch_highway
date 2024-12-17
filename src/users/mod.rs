use std::sync::Arc;

use asknothingx2_util::oauth::{AccessToken, ClientId};
use once_cell::sync::Lazy;
use url::Url;

mod get_user;
pub use get_user::*;

#[derive(Debug)]
pub struct UserAPI {
    access_token: Arc<AccessToken>,
    client_id: Arc<ClientId>,
    url: Arc<Url>,
}

const TWITCH_USERS_URL: &str = "https://api.twitch.tv/helix/users";

static USERS_URL: Lazy<Arc<Url>> = Lazy::new(|| Arc::new(Url::parse(TWITCH_USERS_URL).unwrap()));

impl UserAPI {
    pub fn new(access_token: &AccessToken, client_id: &ClientId) -> Self {
        Self {
            access_token: Arc::new(access_token.clone()),
            client_id: Arc::new(client_id.clone()),
            url: USERS_URL.clone(),
        }
    }

    pub fn set_url(mut self, url: Url) -> Self {
        self.url = Arc::new(url);
        self
    }

    pub fn get_users(&self) -> GetUsers {
        GetUsers::new(
            self.access_token.clone(),
            self.client_id.clone(),
            self.url.clone(),
        )
    }
}
