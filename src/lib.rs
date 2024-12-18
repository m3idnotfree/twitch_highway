//! implement Endpoint
//!
//! Users
//!  [x] Get Users
//!  [ ] Update User
//!  [ ] Get User Block List
//!  [ ] Block User
//!  [ ] Unblock User
//!  [ ] Get User Extensions
//!  [ ] Get User Active Extensions
//!  [ ] Update User Extensions
//!
//! EventSubAPI
//!  [x] Create EventSub Subscription
//!  [x] Delete EventSub Subscription
//!  [ ] Get EventSub Subscriptions
//!
//! Chat
//!  [x] Get Channel Emotes
//!  [x] Get Global Emotes
//!  [x] Get Emote Sets
//!  [x] Get Channel Chat Badges
//!  [x] Get Global Chat Badges
//!  [x] Get Chat Settings
//!  [x] Get Shared Chat Session
//!  [ ] Get User Emotes
//!  [ ] Update Chat Settings
//!  [ ] Send Chat Announcement
//!  [ ] Send a Shoutout
//!  [ ] Send Chat Message
//!  [ ] Get User Chat Color
//!  [ ] Update User Chat Color
//!
//! Todo
//! Ads
//! Bits
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

mod types;
pub use types::*;

pub(crate) mod macros;

pub type Result<TR> = std::result::Result<TR, crate::Error>;
