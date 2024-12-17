use std::sync::Arc;

use asknothingx2_util::oauth::{AccessToken, ClientId};
use badges::BadgeAPI;
use emotes::EmoteAPI;
use get_chatters::GetChatters;
use once_cell::sync::Lazy;
use url::Url;

pub mod emotes;

pub mod badges;
pub mod get_chatters;

#[derive(Debug)]
pub struct ChatAPI {
    access_token: Arc<AccessToken>,
    client_id: Arc<ClientId>,
    emote_url: Arc<Url>,
    badge_url: Arc<Url>,
    chatters_url: Arc<Url>,
}

const TWITCH_EMOTE_URL: &str = "https://api.twitch.tv/helix/chat/emotes";
const TWITCH_BADGE_URL: &str = "https://api.twitch.tv/helix/chat/badges";
const TWITCH_CHAT_CHATTERS: &str = "https://api.twitch.tv/helix/chat/chatters";

static EMOTE_URL: Lazy<Arc<Url>> = Lazy::new(|| Arc::new(Url::parse(TWITCH_EMOTE_URL).unwrap()));
static BADGE_URL: Lazy<Arc<Url>> = Lazy::new(|| Arc::new(Url::parse(TWITCH_BADGE_URL).unwrap()));
static CHATTERS_URL: Lazy<Arc<Url>> =
    Lazy::new(|| Arc::new(Url::parse(TWITCH_CHAT_CHATTERS).unwrap()));

impl ChatAPI {
    pub fn new(access_token: &AccessToken, client_id: &ClientId) -> Self {
        Self {
            access_token: Arc::new(access_token.clone()),
            client_id: Arc::new(client_id.clone()),
            emote_url: EMOTE_URL.clone(),
            badge_url: BADGE_URL.clone(),
            chatters_url: CHATTERS_URL.clone(),
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

    pub fn get_chatters<T: Into<String>>(&self, broadcaster_id: T, moderator_id: T) -> GetChatters {
        GetChatters::new(
            self.access_token.clone(),
            self.client_id.clone(),
            self.chatters_url.clone(),
            broadcaster_id.into(),
            moderator_id.into(),
        )
    }

    pub fn set_emote_url<T: Into<String>>(&mut self, emote_url: T) {
        self.emote_url = Arc::new(Url::parse(&emote_url.into()).unwrap());
    }

    pub fn set_badge_url<T: Into<String>>(&mut self, badge_url: T) {
        self.badge_url = Arc::new(Url::parse(&badge_url.into()).unwrap());
    }
}
