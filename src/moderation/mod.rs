use serde::{Deserialize, Serialize};

use crate::{APIBase, APIPostBoby, Result};

#[derive(Debug)]
pub struct ModerationAPI {
    data: APIBase,
}
impl ModerationAPI {
    pub fn new<T: Into<String>>(access_token: T, client_id: T) -> ModerationAPI {
        ModerationAPI {
            data: APIBase::new(
                access_token.into(),
                client_id.into(),
                "https://api.twitch.tv/helix/moderation".into(),
            ),
        }
    }
    pub async fn ban<T: Into<String>>(
        &self,
        broadcaster_id: T,
        moderator_id: T,
        body: &APIPostBoby,
    ) -> Result<String> {
        self.data
            .api_post(
                self.data.url_endpoint_qureys(
                    "bans",
                    vec![
                        ("broadcaster_id", &broadcaster_id.into()),
                        ("moderator_id", &moderator_id.into()),
                    ],
                ),
                body,
            )
            .await
    }
    pub async fn ban_json<T: Into<String>>(
        &self,
        broadcaster_id: T,
        moderator_id: T,
        body: &APIPostBoby,
    ) -> Result<BanResponse> {
        self.data
            .api_post_json(
                self.data.url_endpoint_qureys(
                    "bans",
                    vec![
                        ("broadcaster_id", &broadcaster_id.into()),
                        ("moderator_id", &moderator_id.into()),
                    ],
                ),
                body,
            )
            .await
    }

    pub async fn unban<T: Into<String>>(
        &self,
        broadcaster_id: T,
        moderator_id: T,
        user_id: T,
    ) -> Result<String> {
        self.data
            .api_delete(self.data.url_endpoint_qureys(
                "bans",
                vec![
                    ("broadcaster_id", &broadcaster_id.into()),
                    ("moderator_id", &moderator_id.into()),
                    ("user_id", &user_id.into()),
                ],
            ))
            .await
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BanResponse {
    pub data: Vec<Ban>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ban {
    pub broadcaster_id: String,
    pub moderator_id: String,
    pub user_id: String,
    pub created_at: Option<String>,
    pub end_time: Option<String>,
}
