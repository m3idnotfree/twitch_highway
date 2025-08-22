# twitch_highway

[![crates.io](https://img.shields.io/crates/v/twitch_highway.svg)](https://crates.io/crates/twitch_highway)
[![Documentation](https://docs.rs/twitch_highway/badge.svg)](https://docs.rs/twitch_highway)
[![license: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/m3idnotfree/twitch_highway/blob/main/LICENSE)

A Rust library for Twitch API with compile-time safety and comprehensive response support.

- Full support for Twitch API endpoints
- Full support for response types with proper error handling
- New-type pattern for compile-time safety with IDs (e.g: BroadcasterId, ModeratorId)
- Type-safe request builders
- Safe JSON conversion for 204 (No Content) responses

```toml
[dependencies]
# Add the endpoints you need: "chat", "users", etc.
twitch_highway = { version = "1", features = ["moderation"]}
tokio = { version = "1", features = ["full"] }
asknothingx2-util = { version = "0.1", features = ["oauth"] }
# or
# twitch_oauth_token = { version = "2.0.1" }
```

## Quick Start

```rust
use asknothingx2_util::oauth::{AccessToken, ClientId};
// or
// use twitch_oauth_token::oauth_types::{AccessToken, ClientId};

use twitch_highway::{
    moderation::{request::BanUserRequest, ModerationAPI},
    types::{BroadcasterId, ModeratorId, UserId},
    TwitchAPI,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error> {
    let api = TwitchAPI::new(
        AccessToken::new("access_token".to_string()),
        ClientId::new("client_id".to_string()),
    );

    let broadcaster_id = BroadcasterId::new("broadcaster_id");
    let moderator_id = ModeratorId::new("moderator_id");
    let user_id = UserId::new("9876");

    let ban_user = api.ban_user(
        &broadcaster_id,
        &moderator_id,
        BanUserRequest::new(&user_id).reason("no reason"),
    );

    let response = ban_user.json().await;

    match response {
        Ok(success) => { /* */ }
        Err(e) => {
            if e.is_api() {
                println!("API error: {}", e);
            } else if e.is_request() {
                println!("Request error: {}", e);
            } else if e.is_decode() {
                println!("JSON decode error: {}", e);
            }
        }
    }

    Ok(())
}
```

## Feature Flags

- ads
- analytics
- bits
- ccls
- channel-points
- channels
- charity
- chat
- clips
- entitlements
- extensions
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
- whispers
- full

Note: Conduits and EventSub are not implemented yet, but are planned for future releases.

## License

Licensed under the MIT license.
