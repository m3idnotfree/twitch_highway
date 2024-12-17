use std::sync::Arc;

use asknothingx2_util::oauth::{AccessToken, ClientId};
use badges::BadgeAPI;
use emotes::EmoteAPI;
use get_chat_setting::GetChatSetting;
use get_chatters::GetChatters;

pub mod emotes;

pub mod badges;
pub mod get_chat_setting;
pub mod get_chatters;

#[derive(Debug)]
pub struct ChatAPI {
    access_token: Arc<AccessToken>,
    client_id: Arc<ClientId>,
}

// const TWITCH_EMOTE_URL: &str = "https://api.twitch.tv/helix/chat/emotes";
// const TWITCH_BADGE_URL: &str = "https://api.twitch.tv/helix/chat/badges";
// const TWITCH_CHAT_CHATTERS: &str = "https://api.twitch.tv/helix/chat/chatters";
// const TWITCH_CHAT_SETTING: &str = "https://api.twitch.tv/helix/chat/settings";
//
// pub(crate) const TWITCH_API_CHAT_BASE: &str = "https://api.twitch.tv/helix/chat";
// mod paths {
//     pub(crate) const CHATTERS: &str = "chatters";
//     pub(crate) const EMOTES: &str = "emotes";
//     pub(crate) const BADGES: &str = "badges";
//     pub(crate) const SETTINGS: &str = "settings";
// }
//
// static EMOTE_URL: Lazy<Arc<Url>> = Lazy::new(|| Arc::new(Url::parse(TWITCH_EMOTE_URL).unwrap()));
// static BADGE_URL: Lazy<Arc<Url>> = Lazy::new(|| Arc::new(Url::parse(TWITCH_BADGE_URL).unwrap()));
// static CHATTERS_URL: Lazy<Arc<Url>> =
//     Lazy::new(|| Arc::new(Url::parse(TWITCH_CHAT_CHATTERS).unwrap()));
// static CHAT_SETTING_URL: Lazy<Arc<Url>> =
//     Lazy::new(|| Arc::new(Url::parse(TWITCH_CHAT_SETTING).unwrap()));

impl ChatAPI {
    pub fn new(access_token: &AccessToken, client_id: &ClientId) -> Self {
        Self {
            access_token: Arc::new(access_token.clone()),
            client_id: Arc::new(client_id.clone()),
        }
    }

    pub fn emotes(&self) -> EmoteAPI {
        EmoteAPI::new(self.access_token.clone(), self.client_id.clone())
    }

    pub fn badges(&self) -> BadgeAPI {
        BadgeAPI::new(self.access_token.clone(), self.client_id.clone())
    }

    pub fn get_chatters<T: Into<String>>(&self, broadcaster_id: T, moderator_id: T) -> GetChatters {
        GetChatters::new(
            self.access_token.clone(),
            self.client_id.clone(),
            broadcaster_id.into(),
            moderator_id.into(),
        )
    }

    pub fn get_chat_settings<T: Into<String>>(&self, broadcaster_id: T) -> GetChatSetting {
        GetChatSetting::new(
            self.access_token.clone(),
            self.client_id.clone(),
            broadcaster_id.into(),
        )
    }
}
