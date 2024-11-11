mod chat;
pub use chat::*;
mod error;
pub use error::*;
pub mod eventsub;
pub mod users;

pub type Result<TR> = std::result::Result<TR, crate::Error>;
