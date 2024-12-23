mod channel_badges;
pub use channel_badges::*;
mod global_badges;
pub use global_badges::*;

use asknothingx2_util::oauth::{AccessToken, ClientId};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct BadgeAPI {
    access_token: AccessToken,
    client_id: ClientId,
}

impl BadgeAPI {
    pub fn new(access_token: AccessToken, client_id: ClientId) -> Self {
        Self {
            access_token,
            client_id,
        }
    }

    pub fn get_channel<T: Into<String>>(&self, broadcaster_id: T) -> GetChannelBadge {
        GetChannelBadge::new(
            self.access_token.clone(),
            self.client_id.clone(),
            broadcaster_id.into(),
        )
    }

    pub fn get_global(&self) -> GetGlobalBadges {
        GetGlobalBadges::new(self.access_token.clone(), self.client_id.clone())
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BadgeResponse {
    pub data: Vec<Badges>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Badges {
    pub set_id: String,
    pub versions: Vec<Versions>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Badge {
    pub set_id: String,
    pub versions: Versions,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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
