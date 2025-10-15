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
# Add the endpoints you need: "chat", "users", etc.
twitch_highway = { version = "0.3", features = ["moderation"] }
tokio = { version = "1", features = ["full"] }
asknothingx2-util = { version = "0.1", features = ["oauth"] }
# or
# twitch_oauth_token = { version = "2" }
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
twitch_highway = { version = "0.3", features = ["eventsub", "webhook-axum"] }
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
twitch_highway = { version = "0.3", features = ["eventsub", "websocket"] }
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

## Feature Flags

Enable only the API modules you need:

- ads
- analytics
- bits
- ccls
- channel-points
- channels
- charity
- chat
- clips
- conduits
- entitlements
- extensions
- eventsub
- games
- goals
- guest-star
- hype-train
- moderation
- polls
- predictions
- raid
- schedule
- search
- streams
- subscriptions
- teams
- users
- videos
- whisper
- full

### EventSub Features

- **`webhook`**: All webhook features (verification, framework integrations)
- **`webhook-verify`**: Core signature verification
- **`webhook-axum`**: Axum framework integration
- **`webhook-actix`**: Actix-web framework integration
- **`websocket`**: WebSocket client with routing
- **`websocket-client`**: WebSocket client only
- **`websocket-router`**: Event router with middleware

## License

Licensed under either of

- Apache License, Version 2.0
- MIT license
