#[cfg(feature = "websocket-router")]
mod router;

#[cfg(feature = "websocket-client")]
mod client;

mod common;
mod messages;
mod scanner;
mod types;

#[cfg(feature = "websocket-client")]
pub use client::{client, Client, Config, Error, WithGracefulShutdown};

#[cfg(feature = "websocket-router")]
pub use router::{extract, layers, routes, Router};

pub use common::{IntoResponse, Request, Response};
pub use messages::*;
pub use scanner::{ScanError, Scanner};
pub use types::*;
