//! implement Endpoint
//!
//! Users
//! - get users
//!
//! EventSubAPI
//! - create
//! - delete
//!
//! Chat
//! - emotes
//!  - channel
//!  - global
//!  - emote sets
//! - badges
//!  - channel
//!  - global
//! - chatters

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
