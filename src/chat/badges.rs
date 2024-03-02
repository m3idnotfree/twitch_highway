use serde::{Deserialize, Serialize};

use crate::{APIBase, Result};

#[derive(Debug)]
pub struct BadgesAPI {
    data: APIBase,
}

impl BadgesAPI {
    pub fn new<T: Into<String>>(access_token: T, client_id: T) -> BadgesAPI {
        BadgesAPI {
            data: APIBase::new(
                access_token.into(),
                client_id.into(),
                "https://api.twitch.tv/helix/chat/emotes".into(),
            ),
        }
    }

    pub async fn channel<T: Into<String>>(&self, broadcaster_id: T) -> Result<String> {
        self.data
            .api_request_text(
                self.data
                    .url_qurey("broadcaster_id".into(), broadcaster_id.into()),
            )
            .await
    }

    pub async fn channel_to_json<I: Into<String>>(
        &self,
        broadcaster_id: I,
    ) -> Result<BadgeResponse> {
        self.data
            .api_request_json(
                self.data
                    .url_qurey("broadcaster_id".into(), broadcaster_id.into()),
            )
            .await
    }

    pub async fn global(&self) -> Result<String> {
        self.data
            .api_request_text(self.data.url_endpoint("global"))
            .await
    }

    pub async fn global_to_json(&self) -> Result<BadgeResponse> {
        self.data
            .api_request_json(self.data.url_endpoint("global"))
            .await
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BadgeResponse {
    pub data: Vec<Badges>,
}

#[derive(Debug, Serialize, Deserialize)]
// #[serde(tag = "type", rename = "badge")]
pub struct Badges {
    pub set_id: String,
    pub versions: Vec<Versions>,
}

#[derive(Debug, Serialize, Deserialize)]
// #[serde(tag = "type", rename = "badge")]
pub struct Badge {
    pub set_id: String,
    pub versions: Versions,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Versions {
    pub id: String,
    pub image_url_1x: String,
    pub image_url_2x: String,
    pub image_url_4x: String,
    pub title: String,
    pub description: String,
    pub click_action: Option<String>,
    pub click_url: Option<String>,
}
