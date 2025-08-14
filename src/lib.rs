//! # I'm on the highway to hell
//!
//! By default, no API endpoints are enabled.
//!
//! <https://dev.twitch.tv/docs/api/reference/>
//!
//! # Usage
//! ```toml
//! [dependencies]
//! twitch_highway = { version = "0.1", features = ["users"] }
//! asknothingx2-util = { version = "0.0.28", features = ["oauth"] }
//! ```
//! ```rust,ignore
//! use asknothingx2_util::oauth::{AccessToken, ClientId};
//! use twitch_highway::{
//!     types::UserId,
//!     users::{request::BlockReason, UserAPI},
//!     TwitchAPI,
//! };
//!
//! #[tokio::main]
//! async fn main() {
//!     let api = TwitchAPI::new(
//!         AccessToken::new("access_token".to_string()),
//!         ClientId::new("client_id".to_string()),
//!     );
//!
//!     let user_block = api.block_user(UserId::new("user_id"), None, Some(BlockReason::Harassment));
//!     let response = user_block.request().await.unwrap();
//!     let response = response.parse_response().unwrap();
//! }
//! ```
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
//! - [`entitlements`][crate::entitlements::EntitlementsAPI]
//! - [`extensions`][crate::extensions::ExtensionsAPI]
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
//!
//! - Conduits: coming soom
//! - EventSub: coming soom
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
//!             eprintln!("Network error: {}", e);
//!         } else if e.is_api() {
//!             eprintln!("Twitch API error: {}", e);
//!         } else if e.is_decode() {
//!             eprintln!("JSON decode error: {}", e);
//!         }
//!     }
//! }
//! # }
//! ```

#![cfg_attr(docsrs, feature(doc_cfg))]

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
use types::JWTToken;
use url::Url;

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

    pub(crate) fn default_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        HeaderMut::new(&mut headers)
            .bearer_token(self.access_token.secret())
            .client_id(&self.client_id)
            .unwrap();
        headers
    }

    pub(crate) fn header_json(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        HeaderMut::new(&mut headers)
            .bearer_token(self.access_token.secret())
            .client_id(&self.client_id)
            .unwrap()
            .content_type_json();
        headers
    }

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
#[cfg_attr(docsrs, doc(cfg(feature = "ads")))]
pub mod ads;
#[cfg(feature = "analytics")]
#[cfg_attr(docsrs, doc(cfg(feature = "analytics")))]
pub mod analytics;
#[cfg(feature = "bits")]
#[cfg_attr(docsrs, doc(cfg(feature = "bits")))]
pub mod bits;
#[cfg(feature = "ccls")]
#[cfg_attr(docsrs, doc(cfg(feature = "ccls")))]
pub mod ccls;
#[cfg(feature = "channel-points")]
#[cfg_attr(docsrs, doc(cfg(feature = "channel-points")))]
pub mod channel_points;
#[cfg(feature = "channels")]
#[cfg_attr(docsrs, doc(cfg(feature = "channels")))]
pub mod channels;
#[cfg(feature = "charity")]
#[cfg_attr(docsrs, doc(cfg(feature = "charity")))]
pub mod charity;
#[cfg(feature = "chat")]
#[cfg_attr(docsrs, doc(cfg(feature = "chat")))]
pub mod chat;
#[cfg(feature = "clips")]
#[cfg_attr(docsrs, doc(cfg(feature = "clips")))]
pub mod clips;
#[cfg(feature = "entitlements")]
#[cfg_attr(docsrs, doc(cfg(feature = "entitlements")))]
pub mod entitlements;
//#[cfg(feature = "eventsub")]
//#[cfg_attr(docsrs, doc(cfg(feature = "eventsub")))]
//pub mod eventsub;
#[cfg(feature = "extensions")]
#[cfg_attr(docsrs, doc(cfg(feature = "extensions")))]
pub mod extensions;
#[cfg(feature = "games")]
#[cfg_attr(docsrs, doc(cfg(feature = "games")))]
pub mod games;
#[cfg(feature = "goals")]
#[cfg_attr(docsrs, doc(cfg(feature = "goals")))]
pub mod goals;
#[cfg(feature = "guest-star")]
#[cfg_attr(docsrs, doc(cfg(feature = "guest-star")))]
pub mod guest_star;
#[cfg(feature = "hype-train")]
#[cfg_attr(docsrs, doc(cfg(feature = "hype-train")))]
pub mod hype_train;
#[cfg(feature = "moderation")]
#[cfg_attr(docsrs, doc(cfg(feature = "moderation")))]
pub mod moderation;
#[cfg(feature = "polls")]
#[cfg_attr(docsrs, doc(cfg(feature = "polls")))]
pub mod polls;
#[cfg(feature = "predictions")]
#[cfg_attr(docsrs, doc(cfg(feature = "predictions")))]
pub mod predictions;
#[cfg(feature = "raid")]
#[cfg_attr(docsrs, doc(cfg(feature = "raid")))]
pub mod raid;
#[cfg(feature = "schedule")]
#[cfg_attr(docsrs, doc(cfg(feature = "schedule")))]
pub mod schedule;
#[cfg(feature = "search")]
#[cfg_attr(docsrs, doc(cfg(feature = "search")))]
pub mod search;
#[cfg(feature = "streams")]
#[cfg_attr(docsrs, doc(cfg(feature = "streams")))]
pub mod streams;
#[cfg(feature = "subscriptions")]
#[cfg_attr(docsrs, doc(cfg(feature = "subscriptions")))]
pub mod subscriptions;
#[cfg(feature = "teams")]
#[cfg_attr(docsrs, doc(cfg(feature = "teams")))]
pub mod teams;
#[cfg(feature = "users")]
#[cfg_attr(docsrs, doc(cfg(feature = "users")))]
pub mod users;
#[cfg(feature = "videos")]
#[cfg_attr(docsrs, doc(cfg(feature = "videos")))]
pub mod videos;
#[cfg(feature = "whispers")]
#[cfg_attr(docsrs, doc(cfg(feature = "whispers")))]
pub mod whispers;
