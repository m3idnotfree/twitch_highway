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

#[cfg(feature = "test")]
#[macro_use]
mod test_url;
#[cfg(feature = "test")]
pub use test_url::{TestUrl, TestUrlHold};

const TWITCH_API_BASE: &str = "https://api.twitch.tv/helix";

#[cfg(feature = "chat")]
pub mod chat;

#[cfg(feature = "eventsub")]
pub mod eventsub;

#[cfg(feature = "users")]
pub mod users;

#[cfg(any(
    feature = "chat",
    feature = "eventsub",
    feature = "users",
    feature = "test"
))]
mod error;
#[cfg(any(
    feature = "chat",
    feature = "eventsub",
    feature = "users",
    feature = "test"
))]
pub use error::*;

#[cfg(any(
    feature = "chat",
    feature = "eventsub",
    feature = "users",
    feature = "types",
    feature = "test",
))]
pub mod types;

#[cfg(any(
    feature = "chat",
    feature = "eventsub",
    feature = "users",
    feature = "test"
))]
pub type Result<TR> = std::result::Result<TR, crate::Error>;

#[cfg(any(
    feature = "chat",
    feature = "eventsub",
    feature = "users",
    feature = "test"
))]
pub use chrono::{Datelike, Timelike};

mod request;
pub use request::{AsBody, EmptyBody, EndpointType, TokenType, TwitchAPIRequest};

#[cfg(any(
    feature = "chat",
    feature = "eventsub",
    feature = "users",
    feature = "test"
))]
mod base;
#[cfg(any(
    feature = "chat",
    feature = "eventsub",
    feature = "users",
    feature = "test"
))]
pub use base::TwitchAPI;
