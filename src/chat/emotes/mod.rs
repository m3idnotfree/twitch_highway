mod channel_emotes;
pub use channel_emotes::*;
mod global_emotes;
pub use global_emotes::*;
mod emote_sets;
pub use emote_sets::*;

use std::sync::Arc;

use asknothingx2_util::oauth::{AccessToken, ClientId};

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct EmoteAPI {
    access_token: Arc<AccessToken>,
    client_id: Arc<ClientId>,
}

impl EmoteAPI {
    pub fn new(access_token: Arc<AccessToken>, client_id: Arc<ClientId>) -> Self {
        Self {
            access_token,
            client_id,
        }
    }

    pub fn get_channel<T: Into<String>>(&self, broadcaster_id: T) -> GetChannelEmotes {
        GetChannelEmotes::new(
            self.access_token.clone(),
            self.client_id.clone(),
            broadcaster_id.into(),
        )
    }

    pub fn get_global(&self) -> GetGlobalEmotes {
        GetGlobalEmotes::new(self.access_token.clone(), self.client_id.clone())
    }

    pub fn get_emote_sets(&self) -> GetEmoteSets {
        GetEmoteSets::new(self.access_token.clone(), self.client_id.clone())
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
