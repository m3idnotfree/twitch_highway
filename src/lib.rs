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

pub(crate) const TWITCH_API_BASE: &str = "https://api.twitch.tv/helix";
#[cfg(feature = "chat")]
mod chat;

#[cfg(feature = "chat")]
pub use chat::*;

#[cfg(feature = "eventsub")]
mod eventsub;
#[cfg(feature = "eventsub")]
pub use eventsub::*;

#[cfg(feature = "users")]
mod users;
#[cfg(feature = "users")]
pub use users::*;

mod error;
pub use error::*;

#[cfg(feature = "types")]
mod types;
#[cfg(feature = "types")]
pub use types::*;

mod serde_util;

pub(crate) mod impl_endpoint;
pub(crate) mod macros;

pub type Result<TR> = std::result::Result<TR, crate::Error>;

#[cfg(feature = "test")]
mod test_url;
#[cfg(feature = "test")]
pub use test_url::TestUrl;
