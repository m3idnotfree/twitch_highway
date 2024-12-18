use std::sync::Arc;

use asknothingx2_util::oauth::{AccessToken, ClientId};
use badges::BadgeAPI;
use emotes::EmoteAPI;
use get_chat_setting::GetChatSetting;
use get_chatters::GetChatters;
use get_shared_chat_session::GetSharedChatSession;

pub mod badges;
pub mod emotes;
pub mod get_chat_setting;
pub mod get_chatters;
pub mod get_shared_chat_session;

#[derive(Debug)]
pub struct ChatAPI {
    access_token: Arc<AccessToken>,
    client_id: Arc<ClientId>,
}

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

    pub fn get_shared_chat_session<T: Into<String>>(
        &self,
        broadcaster_id: T,
    ) -> GetSharedChatSession {
        GetSharedChatSession::new(
            self.access_token.clone(),
            self.client_id.clone(),
            broadcaster_id.into(),
        )
    }
}
