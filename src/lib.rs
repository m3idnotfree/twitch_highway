//! # I'm on the highway to hell
//!
//! By default, no API endpoints are enabled.
//! https://dev.twitch.tv/docs/api/reference/
//!
//! # Usage
//! ```toml
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
//! }
//! ```
//!
//! # Features
//! - [`AdsAPI`]
//! - [`AnalyticsAPI`]
//! - [`BitsAPI`]
//! - [`CclsAPI`]
//! - [`ChannelPointsAPI`]
//! - [`ChannelsAPI`]
//! - [`CharityAPI`]
//! - [`ChatAPI`]
//! - [`ClipsAPI`]
//! - [x] Conduits
//! - [`CclsAPI`]
//! - [`EntitlementsAPI`]
//! - [`ExtensionsAPI`]
//! - [x] EventSub
//! - [`GamesAPI`]
//! - [`GoalsAPI`]
//! - [`GuestStarAPI`]
//! - [`HypeTrainAPI`]
//! - [`ModerationAPI`]
//! - [`PollsAPI`]
//! - [`PredictionsAPI`]
//! - [`RaidAPI`]
//! - [`ScheduleAPI`]
//! - [`SearchAPI`]
//! - [`StreamsAPI`]
//! - [`SubscriptionsAPI`]
//! - [x] Tags
//! - [`TeamsAPI`]
//! - [`UserAPI`]
//! - [`VideosAPI`]
//! - [`WhisperAPI`]

#[macro_use]
mod macros;

pub mod types;

mod error;
mod request;
mod response;

#[cfg(any(
    feature = "ads",
    feature = "analytics",
    feature = "bits",
    feature = "ccls",
    feature = "channel-points",
    feature = "channels",
    feature = "charity",
    feature = "chat",
    feature = "clips",
    feature = "entitlements",
    feature = "extensions",
    feature = "games",
    feature = "goals",
    feature = "guest-star",
    feature = "hype-train",
    feature = "moderation",
    feature = "polls",
    feature = "predictions",
    feature = "raid",
    feature = "schedule",
    feature = "search",
    feature = "streams",
    feature = "subscriptions",
    feature = "teams",
    feature = "users",
    feature = "videos",
    feature = "whispers",
))]
mod base;

#[cfg(any(
    feature = "ads",
    feature = "analytics",
    feature = "bits",
    feature = "ccls",
    feature = "channel-points",
    feature = "channels",
    feature = "charity",
    feature = "chat",
    feature = "clips",
    feature = "entitlements",
    feature = "extensions",
    feature = "games",
    feature = "goals",
    feature = "guest-star",
    feature = "hype-train",
    feature = "moderation",
    feature = "polls",
    feature = "predictions",
    feature = "raid",
    feature = "schedule",
    feature = "search",
    feature = "streams",
    feature = "subscriptions",
    feature = "teams",
    feature = "users",
    feature = "videos",
    feature = "whispers",
))]
pub use base::TwitchAPI;
pub use error::Error;
pub use request::{
    APIError, EmptyBody, EndpointType, IntoRequestBody, TokenType, TwitchAPIRequest,
};
pub use response::Response;

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
#[cfg(feature = "entitlements")]
pub mod entitlements;
//#[cfg(feature = "eventsub")]
//pub mod eventsub;
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

#[cfg(feature = "test")]
pub mod test_url;
