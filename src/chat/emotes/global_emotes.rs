use std::sync::Arc;

use asknothingx2_util::{
    api::{APIRequest, HeaderBuilder, Method},
    oauth::{AccessToken, ClientId},
};
use serde::{Deserialize, Serialize};
use url::Url;

use super::Images;

#[derive(Debug)]
pub struct GetGlobalEmotes<'a> {
    access_token: Arc<AccessToken>,
    client_id: Arc<ClientId>,
    url: &'a Url,
}
impl<'a> GetGlobalEmotes<'a> {
    pub fn new(access_token: Arc<AccessToken>, client_id: Arc<ClientId>, url: &'a Url) -> Self {
        Self {
            access_token,
            client_id,
            url,
        }
    }
}

impl APIRequest for GetGlobalEmotes<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn headers(&self) -> reqwest::header::HeaderMap {
        HeaderBuilder::new()
            .authorization("Bearer", self.access_token.secret().as_str())
            .append("Client-Id", self.client_id.as_str())
            .unwrap()
            .build()
    }

    fn url(&self) -> Url {
        let mut url = self.url.clone();
        url.set_path("global");
        url
    }
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
pub struct EmoteGlobalResponse {
    pub data: Vec<EmoteGlobal>,
}
