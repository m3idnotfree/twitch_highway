//! # Twitch highway Library
//! <https://dev.twitch.tv/docs/api/reference/>
//!
//! **Important!** By default, no API endpoints are enabled.
//! You must specify the features you need in Cargo.toml
//!
//! Using traits as a resuable foundation, **TwitchAPI** organizes and implements endpoints separately.
//!
//! # Recommended Usage
//!
//! This library is designed to work seamlessly with [`twitch_oauth_token`] for OAuth token management and authentication.
//!
//! # Features
//! - [`ads`][crate::ads::AdsAPI]
//! - [`analytics`][crate::analytics::AnalyticsAPI]
//! - [`bits`][crate::bits::BitsAPI]
//! - [`channels`][crate::channels::ChannelsAPI]
//! - [`channel-points`][crate::channel_points::ChannelPointsAPI]
//! - [`charity`][crate::charity::CharityAPI]
//! - [`Chat`][crate::chat::ChatAPI]
//! - [`clips`][crate::clips::ClipsAPI]
//! - [`ccls`][crate::ccls::CclsAPI]
//! - [`conduits`][crate::conduits::ConduitsAPI]
//! - [`entitlements`][crate::entitlements::EntitlementsAPI]
//! - [`extensions`][crate::extensions::ExtensionsAPI]
//! - [`eventsub`][crate::eventsub::EventSubAPI]
//! - [`games`][crate::games::GamesAPI]
//! - [`goals`][crate::goals::GoalsAPI]
//! - [`guest-star`][crate::guest_star::GuestStarAPI]
//! - [`hype-train`][crate::hype_train::HypeTrainAPI]
//! - [`moderation`][crate::moderation::ModerationAPI]
//! - [`polls`][crate::polls::PollsAPI]
//! - [`predictions`][crate::predictions::PredictionsAPI]
//! - [`raid`][crate::raid::RaidAPI]
//! - [`schedule`][crate::schedule::ScheduleAPI]
//! - [`search`][crate::search::SearchAPI]
//! - [`streams`][crate::streams::StreamsAPI]
//! - [`subscriptions`][crate::subscriptions::SubscriptionsAPI]
//! - [`teams`][crate::teams::TeamsAPI]
//! - [`users`][crate::users::UserAPI]
//! - [`videos`][crate::videos::VideosAPI]
//! - [`whispers`][crate::whispers::WhisperAPI]
//! - Tags: deprecated
//!
//! ## Error Handling
//!
//! The library provides structured error handling
//!
//! ```rust
//! # use twitch_highway::request::TwitchAPIRequest;
//! # use serde::de::DeserializeOwned;
//! # async fn run<T: DeserializeOwned>(api: TwitchAPIRequest<T>) {
//! match api.json().await {
//!     Ok(response) => { /* success */ }
//!     Err(e) => {
//!         if e.is_request() {
//!             println!("Request error: {}", e);
//!         } else if e.is_api() {
//!             println!("API error: {}", e);
//!         } else if e.is_decode() {
//!             println!("JSON decode error: {}", e);
//!         }
//!     }
//! }
//! # }
//! ```

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[macro_use]
mod macros;

pub mod request;
pub mod types;

mod error;

pub use error::Error;

use std::sync::LazyLock;

use asknothingx2_util::{
    api::HeaderMut,
    oauth::{AccessToken, ClientId},
};
use reqwest::header::HeaderMap;
use url::Url;

#[cfg(feature = "extensions")]
use types::JWTToken;

const TWITCH_API_BASE: &str = "https://api.twitch.tv/helix";

static BASE_URL: LazyLock<Url> = LazyLock::new(|| url::Url::parse(TWITCH_API_BASE).unwrap());

#[derive(Debug)]
pub struct TwitchAPI {
    access_token: AccessToken,
    client_id: ClientId,
    url: Url,
}

impl TwitchAPI {
    pub fn new(access_token: AccessToken, client_id: ClientId) -> Self {
        Self {
            access_token,
            client_id,
            url: BASE_URL.clone(),
        }
    }

    pub fn with_url(access_token: AccessToken, client_id: ClientId, url: Url) -> Self {
        Self {
            access_token,
            client_id,
            url,
        }
    }

    pub fn set_access_token(mut self, access_token: AccessToken) -> Self {
        self.access_token = access_token;
        self
    }

    pub fn set_client_id(mut self, client_id: ClientId) -> Self {
        self.client_id = client_id;
        self
    }

    pub fn set_url(mut self, url: Url) -> Self {
        self.url = url;
        self
    }

    pub fn access_token(&self) -> &AccessToken {
        &self.access_token
    }

    pub fn client_id(&self) -> &ClientId {
        &self.client_id
    }

    #[allow(dead_code)]
    pub(crate) fn default_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        HeaderMut::new(&mut headers)
            .bearer_token(self.access_token.secret())
            .client_id(&self.client_id)
            .unwrap();
        headers
    }

    #[cfg(any(
        feature = "ads",
        feature = "channel-points",
        feature = "channels",
        feature = "chat",
        feature = "conduits",
        feature = "extensions",
        feature = "entitlements",
        feature = "eventsub",
        feature = "moderation",
        feature = "polls",
        feature = "predictions",
        feature = "schedule",
        feature = "streams",
        feature = "users",
        feature = "whispers",
    ))]
    pub(crate) fn header_json(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        HeaderMut::new(&mut headers)
            .bearer_token(self.access_token.secret())
            .client_id(&self.client_id)
            .unwrap()
            .content_type_json();
        headers
    }

    #[cfg(feature = "extensions")]
    pub(crate) fn build_jwt_headers(&self, jwt: &JWTToken) -> HeaderMap {
        let mut headers = HeaderMap::new();
        HeaderMut::new(&mut headers).bearer_token(jwt.as_str());
        headers
    }

    pub(crate) fn build_url(&self) -> Url {
        self.url.clone()
    }
}

#[cfg(feature = "ads")]
pub mod ads;
#[cfg(feature = "analytics")]
pub mod analytics;
#[cfg(feature = "bits")]
pub mod bits;
#[cfg(feature = "ccls")]
pub mod ccls;
#[cfg(feature = "channel-points")]
pub mod channel_points;
#[cfg(feature = "channels")]
pub mod channels;
#[cfg(feature = "charity")]
pub mod charity;
#[cfg(feature = "chat")]
pub mod chat;
#[cfg(feature = "clips")]
pub mod clips;
#[cfg(feature = "conduits")]
pub mod conduits;
#[cfg(feature = "entitlements")]
pub mod entitlements;
#[cfg(feature = "eventsub")]
pub mod eventsub;
#[cfg(feature = "extensions")]
pub mod extensions;
#[cfg(feature = "games")]
pub mod games;
#[cfg(feature = "goals")]
pub mod goals;
#[cfg(feature = "guest-star")]
pub mod guest_star;
#[cfg(feature = "hype-train")]
pub mod hype_train;
#[cfg(feature = "moderation")]
pub mod moderation;
#[cfg(feature = "polls")]
pub mod polls;
#[cfg(feature = "predictions")]
pub mod predictions;
#[cfg(feature = "raid")]
pub mod raid;
#[cfg(feature = "schedule")]
pub mod schedule;
#[cfg(feature = "search")]
pub mod search;
#[cfg(feature = "streams")]
pub mod streams;
#[cfg(feature = "subscriptions")]
pub mod subscriptions;
#[cfg(feature = "teams")]
pub mod teams;
#[cfg(feature = "users")]
pub mod users;
#[cfg(feature = "videos")]
pub mod videos;
#[cfg(feature = "whispers")]
pub mod whispers;

#[cfg(test)]
mod test_utils;
