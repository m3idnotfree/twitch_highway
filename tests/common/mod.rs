#![allow(unused_imports)]
#![allow(dead_code)]

#[macro_use]
mod macros;

#[cfg(any(feature = "webhook-actix",feature = "webhook-axum"))]
mod axum_server;
#[cfg(any(feature = "webhook-actix",feature = "webhook-axum"))]
pub use axum_server::axum_server;

#[cfg(feature = "websocket")]
pub mod websocket;

mod config;
mod fixtures;
mod http_mocks;
mod mock_data;
mod triggers;

pub use config::{CliConfig, CliError};
pub use fixtures::TwitchFixture;
pub use http_mocks::HttpMock;
pub use mock_data::get_stream_data;
pub use triggers::{trigger_webhook_event, trigger_websocket_event};

use tokio::process::Child;

pub async fn mock_api_start() -> Result<Child, CliError> {
    mock_api_start_with_config(CliConfig::mock_api()).await
}

pub async fn mock_api_start_with_config(config: CliConfig) -> Result<Child, CliError> {
    config.spawn_wait_server().await
}
