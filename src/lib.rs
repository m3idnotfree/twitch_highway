//! implement Endpoint
//!
//!
//!
//!  ㄱ: Implement
//!  ㄷ: Pass test
//!  ㄴ: Pass twitch mack-api
//!  ㅐ: Failed twitch mack-api
//!  ㅠ: Pass real world API
//! Users
//!  [ㄷ] Get Users
//!  [ㄷ] Update User
//!  [ㄷ] Get User Block List
//!  [ㄷ] Block User
//!  [ㄷ] Unblock User
//!  [ㄷ] Get User Extensions
//!  [ㄷ] Get User Active Extensions
//!     - [ ] Response
//!  [ ] Update User Extensions
//!
//! EventSubAPI: mock server except
//!  [ㅐ] Create EventSub Subscription
//!     [ㄱ] drop entitlement grant
//!     [ㄱ] extension bits transaction create
//!     [ㄱ] conduit shard disabled
//!     [ㄱ] channel raid
//!  [ㅐ] Delete EventSub Subscription
//!  [ ] Get EventSub Subscriptions
//!
//! Chat
//!  [ㄷ] Get Chatters
//!  [ㄷ] Get Channel Emotes
//!  [ㅐ] Get Global Emotes
//!  [ㅐ] Get Emote Sets
//!  [ㄷ] Get Channel Chat Badges
//!  [ㅐ] Get Global Chat Badges
//!  [ㄷ] Get Chat Settings
//!  [ㅐ] Get Shared Chat Session
//!  [ㄷ] Get User Emotes
//!  [ㄷ] Update Chat Settings
//!  [ ] Send Chat Announcement
//!  [ ] Send a Shoutout
//!  [ㅐ] Send Chat Message
//!  [ㄷ] Get User Chat Color
//!  [ㄷ] Update User Chat Color
//!
//! Todo
//! Ads
//! Bitsc
//! Channels
//! Channel Points
//! Charity
//! Clips
//! Conduits
//! CCLs
//! Entitlements: mock server except
//! Extensions: mock server except
//! Games
//! Goals
//! Guest Star
//! Hype Train
//! Moderation
//! Polls
//! Predictions
//! Raids
//! Schedule
//! Search
//! Streams
//! Subscriptions
//! Tags
//! Teams
//! Videos
//! Whispers
//! The mock server replicates a majority of the Twitch API endpoints except
//! for endpoints related to:
//! Extensions
//! Code entitlements
//! EventSub

#[macro_use]
mod macros;

pub mod types;

mod base;
mod error;
mod request;

pub use base::TwitchAPI;
pub use chrono::{Datelike, Timelike};
pub use error::Error;
pub use request::{AsBody, EmptyBody, EndpointType, TokenType, TwitchAPIRequest};

#[cfg(feature = "ads")]
pub mod ads;
#[cfg(feature = "analytics")]
pub mod analytics;
#[cfg(feature = "bits")]
pub mod bits;
#[cfg(feature = "channel_points")]
pub mod channel_points;
#[cfg(feature = "channels")]
pub mod channels;
#[cfg(feature = "charity")]
pub mod charity;
#[cfg(feature = "chat")]
pub mod chat;
#[cfg(feature = "clips")]
pub mod clips;
#[cfg(feature = "eventsub")]
pub mod eventsub;
#[cfg(feature = "hype-train")]
pub mod hype_train;
#[cfg(feature = "moderation")]
pub mod moderation;
#[cfg(feature = "polls")]
pub mod polls;
#[cfg(feature = "raid")]
pub mod raid;
#[cfg(feature = "streams")]
pub mod streams;
#[cfg(feature = "subscriptions")]
pub mod subscriptions;
#[cfg(feature = "users")]
pub mod users;
#[cfg(feature = "videos")]
pub mod videos;
#[cfg(feature = "whispers")]
pub mod whispers;

#[cfg(feature = "test")]
mod test_url;
#[cfg(feature = "test")]
pub use test_url::{TestUrl, TestUrlHold};
