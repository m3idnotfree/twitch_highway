//! Twitch EventSub Webhook verification and utilities.
//!
//! # Features
//!
//! - `webhook-axum`: [`HeaderAccess`] implementation for axum via `axum-core`, `IntoResponse` for [`VerificationError`]
//! - `webhook-actix`: [`HeaderAccess`] implementation for actix-web via `actix-http`
//!
//! # Framework Integration
//!
//! The webhook verification works with any framework by implementing the [`HeaderAccess`] trait.
//! Built-in implementations are provided for common frameworks:
//!
//! ## Axum Example
//!
//! ```rust
//! use std::sync::Arc;
//!
//! use axum::{
//!     body::{Body, Bytes},
//!     extract::State,
//!     http::{HeaderMap, StatusCode},
//!     response::Response,
//!     routing::post,
//!     Router,
//! };
//! use tokio::sync::RwLock;
//! use twitch_highway::eventsub::{
//!     events::channels::follow::ChannelFollow,
//!     webhook::{
//!         get_message_type, get_subscription_type, verify_event_message, Challenge, MessageType,
//!         Notification, Revoke, VerificationError,
//!     },
//!     SubscriptionType,
//! };
//!
//! #[derive(Clone)]
//! struct AppState {
//!     pub secret: Arc<RwLock<String>>,
//! }
//!
//! async fn webhook_handler(
//!     headers: HeaderMap,
//!     State(state): State<AppState>,
//!     body: Bytes,
//! ) -> Result<Response, VerificationError> {
//!     let secret = state.secret.read().await;
//!
//!     verify_event_message(&headers, &body, &secret)?;
//!
//!     if let Ok(message_type) = get_message_type(&headers) {
//!         match message_type {
//!             MessageType::Verification => {
//!                 let challenge: Challenge = serde_json::from_slice(&body).unwrap();
//!
//!                 Ok(Response::builder()
//!                     .status(StatusCode::OK)
//!                     .header("Content-Type", "text/plain")
//!                     .body(Body::from(challenge.challenge))
//!                     .unwrap())
//!             },
//!             MessageType::Notification => {
//!                 let subscription_type = get_subscription_type(&headers);
//!                 if let Some(subscription_type) = subscription_type {
//!
//!                     match subscription_type {
//!                         SubscriptionType::ChannelFollow => {
//!                             let _notification: Notification<ChannelFollow> = serde_json::from_slice(&body).unwrap();
//!                         }
//!                         SubscriptionType::ChannelSubscribe => {/* */}
//!                         _ => {}
//!                     }
//!                 }
//!
//!                 Ok(Response::builder()
//!                     .status(StatusCode::NO_CONTENT)
//!                     .body(Body::empty())
//!                     .unwrap())
//!             },
//!             MessageType::Revocation => {
//!                 let _revocation: Revoke = serde_json::from_slice(&body).unwrap();
//!
//!                 Ok(Response::builder()
//!                     .status(StatusCode::NO_CONTENT)
//!                     .body(Body::empty())
//!                     .unwrap())
//!             },
//!         }
//!     } else {
//!         Ok(Response::builder()
//!             .status(StatusCode::NO_CONTENT)
//!             .body(Body::empty())
//!             .unwrap())
//!     }
//! }
//!
//! # async fn example() {
//! let state = AppState {
//!     secret: Arc::new(RwLock::new("".to_string())),
//! };
//!
//! let app = Router::new()
//!     .route("/webhook", post(webhook_handler))
//!     .with_state(state);
//!
//! # let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
//! # axum::serve(listener, app);
//! # }
//! ```
//!
//! ## Actix-web Example
//!
//! ```rust,no_run
//! use actix_web::{http::header, mime, post, web, HttpRequest, HttpResponse};
//! use twitch_highway::eventsub::{
//!     events::channels::raid::Raid,
//!     webhook::{
//!         get_message_type, get_subscription_type, verify_event_message, Challenge, MessageType,
//!         Notification, Revoke,
//!     },
//!     SubscriptionType,
//! };
//!
//! #[post("/webhook")]
//! async fn webhook_handler(req: HttpRequest, body: web::Bytes) -> HttpResponse {
//!     let secret = "your-webhook-secret";
//!
//!     if verify_event_message(req.headers(), &body, secret).is_err() {
//!         return HttpResponse::Forbidden().finish();
//!     }
//!
//!     if let Ok(msg_type) = get_message_type(req.headers()) {
//!         match msg_type {
//!             MessageType::Notification => {
//!                 let sub_type = get_subscription_type(req.headers());
//!                 if let Some(sub_type) = sub_type {
//!                     match sub_type {
//!                         SubscriptionType::ChannelRaid => {
//!                             let _channel_raid: Notification<Raid> =
//!                                 serde_json::from_slice(&body).unwrap();
//!                         }
//!                         SubscriptionType::ChannelSuspiciousUserMessage => {}
//!                         _ => {}
//!                     }
//!                 }
//!
//!                 HttpResponse::NoContent().finish()
//!             }
//!             MessageType::Verification => {
//!                 let challenge: Challenge = serde_json::from_slice(&body).unwrap();
//!
//!                 HttpResponse::Ok()
//!                     .insert_header(header::ContentType(mime::TEXT_PLAIN))
//!                     .body(challenge.challenge)
//!             }
//!             MessageType::Revocation => {
//!                 let _revocation: Revoke = serde_json::from_slice(&body).unwrap();
//!
//!                 HttpResponse::NoContent().finish()
//!             }
//!         }
//!     } else {
//!         HttpResponse::NoContent().finish()
//!     }
//! }
//! ```
//!
//! ## Custom Framework
//!
//! Implement [`HeaderAccess`] for your framework's header type:
//!
//! ```rust
//! use std::collections::HashMap;
//! use twitch_highway::eventsub::webhook::HeaderAccess;
//!
//! struct MyHeaders {
//!     inner: HashMap<String, String>,
//! }
//!
//! impl MyHeaders {
//!     pub fn get(&self, name: &str) -> Option<&str> {
//!         self.inner.get(name).map(|x| x.as_str())
//!     }
//! }
//!
//! impl HeaderAccess for MyHeaders {
//!     fn get_header(&self, name: &str) -> Option<&str> {
//!         self.get(name)
//!     }
//! }
//! ```

mod error;
mod header_access;
mod types;
mod verification;

pub use error::VerificationError;
pub use header_access::HeaderAccess;
pub use types::{Challenge, MessageType, Notification, Revoke};
pub use verification::verify_event_message;

use std::str::FromStr;

use crate::eventsub::{
    resolve_subscription_type,
    webhook::types::{MESSAGE_RETRY, MESSAGE_TYPE, SUBSCRIPTION_TYPE, SUBSCRIPTION_VERSION},
    SubscriptionType,
};

pub fn generate_secret() -> String {
    hex::encode(rand::random::<[u8; 32]>())
}

pub fn get_message_type<H: HeaderAccess>(headers: &H) -> Result<MessageType, VerificationError> {
    let s = headers
        .get_header(MESSAGE_TYPE)
        .ok_or(VerificationError::MissingHeader(MESSAGE_TYPE))?;

    MessageType::from_str(s).map_err(|_| VerificationError::UnknownMessageType(s.to_string()))
}

pub fn get_subscription_type<H: HeaderAccess>(headers: &H) -> Option<SubscriptionType> {
    let s = headers.get_header(SUBSCRIPTION_TYPE)?;
    let v = get_subscription_version(headers);

    let sub = SubscriptionType::from_str(s).ok()?;

    Some(resolve_subscription_type!(sub, opt v))
}

pub fn get_subscription_version<H: HeaderAccess>(headers: &H) -> Option<&str> {
    headers.get_header(SUBSCRIPTION_VERSION)
}
pub fn get_message_retry<H: HeaderAccess>(headers: &H) -> Option<&str> {
    headers.get_header(MESSAGE_RETRY)
}
