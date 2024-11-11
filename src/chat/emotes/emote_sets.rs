use std::sync::Arc;

use asknothingx2_util::{
    api::{APIRequest, HeaderBuilder, HeaderMap, Method},
    oauth::{AccessToken, ClientId},
};
use serde::{Deserialize, Serialize};
use url::Url;

use super::EmoteGlobal;

#[derive(Debug)]
pub struct GetEmoteSets<'a> {
    access_token: Arc<AccessToken>,
    client_id: Arc<ClientId>,
    url: &'a Url,
    emote_set_ids: Vec<&'a str>,
}

impl<'a> GetEmoteSets<'a> {
    pub fn new(access_token: Arc<AccessToken>, client_id: Arc<ClientId>, url: &'a Url) -> Self {
        Self {
            access_token,
            client_id,
            url,
            emote_set_ids: Vec::new(),
        }
    }
    pub fn add_emote_set_id(mut self, id: &'a str) -> Self {
        self.emote_set_ids.push(id);
        self
    }

    pub fn add_emote_set_ids(mut self, ids: Vec<&'a str>) -> Self {
        self.emote_set_ids.extend(ids);
        self
    }
}

impl APIRequest for GetEmoteSets<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn headers(&self) -> HeaderMap {
        HeaderBuilder::new()
            .authorization("Bearer", self.access_token.secret().as_str())
            .append("Client-Id", self.client_id.as_str())
            .unwrap()
            .build()
    }

    fn url(&self) -> Url {
        let mut url = self.url.clone();
        url.set_path("set");
        url.query_pairs_mut().extend_pairs(
            self.emote_set_ids
                .clone()
                .into_iter()
                .map(|x| ("emote_set_id", x))
                .collect::<Vec<(&str, &str)>>(),
        );

        url
    }
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(tag = "type", rename = "emote")]
// pub struct EmoteSets {
//     pub id: String,
//     pub name: String,
//     pub images: Images,
//     pub emote_type: String,
//     pub emote_set_id: String,
//     pub owner_id: String,
//     pub format: Vec<String>,
//     pub scale: Vec<String>,
//     pub theme_mode: Vec<String>,
// }

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmoteSetsResponse {
    pub data: Vec<EmoteGlobal>,
}
