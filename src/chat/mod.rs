use std::sync::Arc;

use asknothingx2_util::oauth::{AccessToken, ClientId};
use badges::BadgeAPI;
use emotes::EmoteAPI;
use url::Url;

pub mod emotes;

pub mod badges;

#[derive(Debug)]
pub struct ChatAPI {
    access_token: Arc<AccessToken>,
    client_id: Arc<ClientId>,
    emote_url: Arc<Url>,
    badge_url: Arc<Url>,
}

impl ChatAPI {
    pub fn new(access_token: &AccessToken, client_id: &ClientId) -> Self {
        Self {
            access_token: Arc::new(access_token.clone()),
            client_id: Arc::new(client_id.clone()),
            emote_url: Arc::new(Url::parse("https://api.twitch.tv/helix/chat/emotes").unwrap()),
            badge_url: Arc::new(Url::parse("https://api.twitch.tv/helix/chat/badges").unwrap()),
        }
    }

    pub fn emotes(&self) -> EmoteAPI {
        EmoteAPI::new(
            self.access_token.clone(),
            self.client_id.clone(),
            self.emote_url.clone(),
        )
    }

    pub fn badges(&self) -> BadgeAPI {
        BadgeAPI::new(
            self.access_token.clone(),
            self.client_id.clone(),
            self.badge_url.clone(),
        )
    }

    pub fn set_emote_url(&mut self, emote_url: &str) {
        self.emote_url = Arc::new(Url::parse(emote_url).unwrap());
    }

    pub fn set_badge_url(&mut self, badge_url: &str) {
        self.badge_url = Arc::new(Url::parse(badge_url).unwrap());
    }
}
