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

pub type Result<TR> = std::result::Result<TR, crate::Error>;
