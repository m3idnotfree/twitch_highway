use serde::de;

use crate::{APIBase, Result};

#[derive(Debug)]
pub struct UserAPI {
    data: APIBase,
}

impl UserAPI {
    pub fn new<T: Into<String>>(access_token: T, client_id: T) -> UserAPI {
        UserAPI {
            data: APIBase::new(
                access_token.into(),
                client_id.into(),
                "https://api.twitch.tv/helix/users".into(),
            ),
        }
    }
    pub async fn get_users_text<T: Into<String>>(&self, id: T) -> Result<String> {
        self.data
            .api_request_text(self.data.url_qurey("broadcaster_id", &id.into()))
            .await
    }
    pub async fn get_users_json<I: Into<String>, T: de::DeserializeOwned>(
        &self,
        id: I,
    ) -> Result<T> {
        self.data
            .api_request_json(self.data.url_qurey("broadcaster_id", &id.into()))
            .await
    }
}
