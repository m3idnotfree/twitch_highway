//! implement Endpoint
//!
//!
//!
//!  ㄱ: Implement
//!  ㄴ: Pass twitch mack-api
//!  ㅐ: Failed twitch mack-api
//!  ㅠ: Pass real world API
//! Users
//!  [ㄴ] Get Users
//!  [ ] Update User
//!  [ ] Get User Block List
//!  [ ] Block User
//!  [ ] Unblock User
//!  [ ] Get User Extensions
//!  [ ] Get User Active Extensions
//!  [ ] Update User Extensions
//!
//! EventSubAPI
//!  [ㅐ] Create EventSub Subscription
//!  [ㅐ] Delete EventSub Subscription
//!  [ ] Get EventSub Subscriptions
//!
//! Chat
//!  [ㄴ] Get Chatters
//!  [ㄴ] Get Channel Emotes
//!  [ㅐ] Get Global Emotes
//!  [ㅐ] Get Emote Sets
//!  [ㄴ] Get Channel Chat Badges
//!  [ㅐ] Get Global Chat Badges
//!  [ㄴ] Get Chat Settings
//!  [ㅐ] Get Shared Chat Session
//!  [ ] Get User Emotes
//!  [ ] Update Chat Settings
//!  [ ] Send Chat Announcement
//!  [ ] Send a Shoutout
//!  [ㅐ] Send Chat Message
//!  [ ] Get User Chat Color
//!  [ ] Update User Chat Color
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
//! Entitlements
//! Extensions
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

#[doc(hidden)]
mod serde_util;

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
