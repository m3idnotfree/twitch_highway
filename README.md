# twitch_highway

[![crates.io](https://img.shields.io/crates/v/twitch_highway.svg)](https://crates.io/crates/twitch_highway)
[![Documentation](https://docs.rs/twitch_highway/badge.svg)](https://docs.rs/twitch_highway)
[![MIT/Apache-2 licensed](https://img.shields.io/crates/l/twitch_highway.svg)]

A Rust library for the Twitch API with compile-time safety and comprehensive response support.

- Comprehensive Twitch Helix API endpoint support
- Type-safe IDs and request builders (BroadcasterId, ModeratorId, etc.)
- Detailed error handling (API, network, and JSON parsing errors)
- EventSub Webhook verification with framework integrations (axum, actix-web)
- EventSub WebSocket client with automatic reconnection and routing

## Quick Start

```toml
[dependencies]
twitch_highway = "0.4"
tokio = { version = "1", features = ["full"] }
asknothingx2-util = { version = "0.5", features = ["oauth"] }
# or
# twitch_oauth_token = { version = "4" }
```

```rust
use asknothingx2_util::oauth::{AccessToken, ClientId};
// or
// use twitch_oauth_token::oauth::{AccessToken, ClientId};

use twitch_highway::{
    moderation::ModerationAPI,
    types::{BroadcasterId, ModeratorId, UserId},
    TwitchAPI,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = TwitchAPI::new(
        AccessToken::from("access_token".to_string()),
        ClientId::from("client_id".to_string()),
    );

    let broadcaster_id = BroadcasterId::from("broadcaster_id");
    let moderator_id = ModeratorId::from("moderator_id");
    let user_id = UserId::from("9876");

    let response = api.ban_user(&broadcaster_id, &moderator_id, &user_id)
        .reason("no reason")
        .duration(600) // 10 minutes
        .json()
        .await;

    match response {
        Ok(success) => { /* */ }
        Err(e) => {
            if e.is_api() {
                println!("API error: {}", e);
            } else if e.is_request() {
                println!("Request error: {}", e);
            } else if e.is_decode() {
                println!("JSON decode error: {}", e);
            } else {
                println!("Unexpected error: {}", e)
            }
        }
    }

    Ok(())
}
```

## EventSub Support

### Webhook Verification (axum)

```toml
[dependencies]
twitch_highway = { version = "0.4", features = ["webhook-axum"] }
axum = "0.8"
```

```rust
use axum::{
    body::{Body, Bytes},
    http::{header::HeaderMap, StatusCode},
    response::Response,
    routing::post,
    Router,
};
use twitch_highway::eventsub::webhook::{verify_event_message, VerificationError};

async fn webhook_handler(headers: HeaderMap, body: Bytes) -> Result<Response, VerificationError> {
    verify_event_message(&headers, &body, "your_webhook_secret")?;

    Ok(Response::builder()
        .status(StatusCode::NO_CONTENT)
        .body(Body::empty())
        .unwrap())

}

let app = Router::new().route("/webhook", post(webhook_handler));
```

### WebSocket Client

```toml
[dependencies]
twitch_highway = { version = "0.4", features = ["websocket"] }
```

```rust
use twitch_highway::eventsub::{
    events::channels::follow::ChannelFollow,
    websocket::{self, extract::Event, routes::channel_follow, Router},
};

async fn on_follow(Event(event): Event<ChannelFollow>) {
    println!("New follower: {}", event.user_name);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error> {
    let app = Router::new().route(channel_follow(on_follow));

    websocket::client("wss://eventsub.wss.twitch.tv/ws", app).await?;
    Ok(())
}

```

### EventSub Features

- **`webhook-axum`**: Axum framework integration
- **`webhook-actix`**: Actix-web framework integration
- **`websocket`**: WebSocket client with routing

## Twitch API Coverage

Synced to Twitch changelog: 2026‑02‑05 - [Twitch change log](https://dev.twitch.tv/docs/change-log)

## License

Licensed under either of

- Apache License, Version 2.0
- MIT license
