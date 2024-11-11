mod channel_emotes;
pub use channel_emotes::*;
mod global_emotes;
pub use global_emotes::*;
mod emote_sets;
pub use emote_sets::*;

use std::sync::Arc;

use asknothingx2_util::oauth::{AccessToken, ClientId};

use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug)]
pub struct EmoteAPI {
    access_token: Arc<AccessToken>,
    client_id: Arc<ClientId>,
    url: Url,
}

impl EmoteAPI {
    pub fn new(access_token: Arc<AccessToken>, client_id: Arc<ClientId>, url: Url) -> Self {
        Self {
            access_token,
            client_id,
            url,
        }
    }
    pub fn get_channel<'a>(&'a self, broadcaster_id: &'a str) -> GetChannelEmotes<'a> {
        GetChannelEmotes::new(
            self.access_token.clone(),
            self.client_id.clone(),
            &self.url,
            broadcaster_id,
        )
    }

    pub fn get_global(&self) -> GetGlobalEmotes<'_> {
        GetGlobalEmotes::new(self.access_token.clone(), self.client_id.clone(), &self.url)
    }

    pub fn get_emote_sets(&self) -> GetEmoteSets<'_> {
        GetEmoteSets::new(self.access_token.clone(), self.client_id.clone(), &self.url)
    }
}

// #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
// #[serde(tag = "type", rename = "emote")]
// pub struct Emote {
//     pub id: String,
//     pub name: String,
//     pub images: Images,
//     pub format: Vec<String>,
//     pub scale: Vec<String>,
//     pub theme_mode: Vec<String>,
// }
//
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Images {
    pub url_1x: String,
    pub url_2x: String,
    pub url_4x: String,
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// enum EmotesResponse {
//     Global(EmoteGlobalResponse),
//     Channel(EmoteChannelResponse),
//     Set(EmoteSetsResponse),
// }
