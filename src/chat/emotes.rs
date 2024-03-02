use serde::{Deserialize, Serialize};

use crate::{APIBase, Result};

#[derive(Debug, PartialEq)]
pub struct EmotesAPI {
    data: APIBase,
}

impl EmotesAPI {
    pub fn new<T: Into<String>>(access_token: T, client_id: T) -> EmotesAPI {
        EmotesAPI {
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
    ) -> Result<EmoteChannelResponse> {
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

    pub async fn global_to_json(&self) -> Result<EmoteGlobalResponse> {
        self.data
            .api_request_json(
                self.data.url_endpoint("global"), // format!("{}/global", self.data.base_url)
            )
            .await
    }

    pub async fn emotes_set<T: Into<String>>(&self, emote_set_id: T) -> Result<String> {
        self.data
            .api_request_text(self.data.url_endpoint_qurey(
                "set",
                "emote_set_id",
                &emote_set_id.into(),
            ))
            .await
    }

    pub async fn emotes_set_json<I: Into<String>>(
        &self,
        emote_set_id: I,
    ) -> Result<EmoteSetsResponse> {
        self.data
            .api_request_json(self.data.url_endpoint_qurey(
                "set",
                "emote_set_id",
                &emote_set_id.into(),
            ))
            .await
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename = "emote")]
pub struct Emote {
    pub id: String,
    pub name: String,
    pub images: Images,
    pub format: Vec<String>,
    pub scale: Vec<String>,
    pub theme_mode: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Images {
    pub url_1x: String,
    pub url_2x: String,
    pub url_4x: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename = "emote")]
pub struct EmoteChannel {
    pub id: String,
    pub name: String,
    pub images: Images,
    pub tier: String,
    pub format: Vec<String>,
    pub scale: Vec<String>,
    pub theme_mode: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum EmotesResponse {
    Global(EmoteGlobalResponse),
    Channel(EmoteChannelResponse),
    Set(EmoteSetsResponse),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename = "emote")]
pub struct EmoteGlobal {
    pub id: String,
    pub name: String,
    pub images: Images,
    pub format: Vec<String>,
    pub scale: Vec<String>,
    pub theme_mode: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmoteChannelResponse {
    pub data: Vec<EmoteChannel>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmoteGlobalResponse {
    pub data: Vec<EmoteGlobal>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmoteSetsResponse {
    pub data: Vec<EmoteGlobal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename = "emote")]
pub struct EmoteSets {
    pub id: String,
    pub name: String,
    pub images: Images,
    pub emote_type: String,
    pub emote_set_id: String,
    pub owner_id: String,
    pub format: Vec<String>,
    pub scale: Vec<String>,
    pub theme_mode: Vec<String>,
}
