mod channel_badges;
pub use channel_badges::*;
mod global_badges;
pub use global_badges::*;

use std::sync::Arc;

use asknothingx2_util::oauth::{AccessToken, ClientId};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug)]
pub struct BadgeAPI {
    access_token: Arc<AccessToken>,
    client_id: Arc<ClientId>,
    url: Url,
}

impl BadgeAPI {
    pub fn new(access_token: Arc<AccessToken>, client_id: Arc<ClientId>, url: Url) -> Self {
        Self {
            access_token,
            client_id,
            url,
        }
    }
    pub fn get_channel<'a>(&'a self, broadcaster_id: &'a str) -> GetChannelBadge<'a> {
        GetChannelBadge::new(
            self.access_token.clone(),
            self.client_id.clone(),
            &self.url,
            broadcaster_id,
        )
    }

    pub fn get_global(&self) -> GetGlobalBadges<'_> {
        GetGlobalBadges::new(self.access_token.clone(), self.client_id.clone(), &self.url)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BadgeResponse {
    pub data: Vec<Badges>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
// #[serde(tag = "type", rename = "badge")]
pub struct Badges {
    pub set_id: String,
    pub versions: Vec<Versions>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
// #[serde(tag = "type", rename = "badge")]
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
