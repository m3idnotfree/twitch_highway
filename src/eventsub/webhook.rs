use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};

use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use subtle::ConstantTimeEq;

use crate::eventsub::{Subscription, SubscriptionType};

const MESSAGE_ID: &str = "twitch-eventsub-message-id";
const MESSAGE_TIMESTAMP: &str = "twitch-eventsub-message-timestamp";
const MESSAGE_SIGNATURE: &str = "twitch-eventsub-message-signature";

#[allow(dead_code)]
const MESSAGE_RETRY: &str = "twitch-eventsub-message-retry";

const SUBSCRIPTION_TYPE: &str = "twitch-eventsub-subscription-type";
const SUBSCRIPTION_VERSION: &str = "twitch-eventsub-subscription-version";

const MESSAGE_TYPE: &str = "twitch-eventsub-message-type";
const MESSAGE_TYPE_VERIFICATION: &str = "webhook_callback_verification";
const MESSAGE_TYPE_NOTIFICATION: &str = "notification";
const MESSAGE_TYPE_REVOCATION: &str = "revocation";

const HMAC_PREFIX: &str = "sha256=";

type HmacSha256 = Hmac<Sha256>;

pub fn generate_secret() -> String {
    hex::encode(rand::random::<[u8; 32]>())
}

pub fn verify_event_message<H: HeaderAccess>(
    headers: &H,
    body: &[u8],
    secret: &str,
) -> Result<(), VerificationError> {
    let signature = headers
        .get_header(MESSAGE_SIGNATURE)
        .ok_or(VerificationError::MissingHeader(MESSAGE_SIGNATURE))?;

    let hmac_signature = hmac_signature(headers, body, secret)?;

    if verify_message(&hmac_signature, signature) {
        Ok(())
    } else {
        Err(VerificationError::InvalidSignature)
    }
}

pub fn get_message_type<H: HeaderAccess>(headers: &H) -> Result<MessageType, VerificationError> {
    let s = headers
        .get_header(MESSAGE_TYPE)
        .ok_or(VerificationError::MissingHeader(MESSAGE_TYPE))?;

    MessageType::from_str(s).map_err(|_| VerificationError::UnknownMessageType(s.to_string()))
}

pub fn get_subscription_type<H: HeaderAccess>(
    headers: &H,
) -> Result<SubscriptionType, VerificationError> {
    let s = headers
        .get_header(SUBSCRIPTION_TYPE)
        .ok_or(VerificationError::MissingHeader(SUBSCRIPTION_TYPE))?;

    SubscriptionType::from_str(s)
        .map_err(|_| VerificationError::UnknownSubscriptionType(s.to_string()))
}

pub fn get_subscription_version<H: HeaderAccess>(headers: &H) -> &str {
    headers.get_header(SUBSCRIPTION_VERSION).unwrap_or("")
}

fn get_hmac_message<H: HeaderAccess>(
    headers: &H,
    body: &[u8],
) -> Result<Vec<u8>, VerificationError> {
    let message_id = headers
        .get_header(MESSAGE_ID)
        .ok_or(VerificationError::MissingHeader(MESSAGE_ID))?;

    let message_timestamp = headers
        .get_header(MESSAGE_TIMESTAMP)
        .ok_or(VerificationError::MissingHeader(MESSAGE_TIMESTAMP))?;

    let mut message = Vec::with_capacity(message_id.len() + message_timestamp.len() + body.len());

    message.extend_from_slice(message_id.as_bytes());
    message.extend_from_slice(message_timestamp.as_bytes());
    message.extend_from_slice(body);
    Ok(message)
}

fn get_hmac(secret: &str, message: &[u8]) -> String {
    let mut mac =
        HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");
    mac.update(message);

    format!(
        "{}{}",
        HMAC_PREFIX,
        hex::encode(mac.finalize().into_bytes())
    )
}

fn verify_message(hmac: &str, signature: &str) -> bool {
    if hmac.len() != signature.len() {
        return false;
    }

    hmac.as_bytes().ct_eq(signature.as_bytes()).into()
}

fn hmac_signature<H: HeaderAccess>(
    headers: &H,
    body: &[u8],
    secret: &str,
) -> Result<String, VerificationError> {
    Ok(get_hmac(secret, &get_hmac_message(headers, body)?))
}

pub trait HeaderAccess {
    fn get_header(&self, name: &str) -> Option<&str>;
}

#[cfg(feature = "webhook-http")]
impl HeaderAccess for http::HeaderMap {
    fn get_header(&self, name: &str) -> Option<&str> {
        self.get(name)?.to_str().ok()
    }
}

#[cfg(feature = "webhook-actix")]
impl HeaderAccess for actix_http::header::HeaderMap {
    fn get_header(&self, name: &str) -> Option<&str> {
        self.get(name)?.to_str().ok()
    }
}

impl HeaderAccess for &[(&str, &str)] {
    fn get_header(&self, name: &str) -> Option<&str> {
        self.iter()
            .find(|(key, _)| key.eq_ignore_ascii_case(name))
            .map(|(_, value)| *value)
    }
}

impl HeaderAccess for Vec<(String, String)> {
    fn get_header(&self, name: &str) -> Option<&str> {
        self.iter()
            .find(|(key, _)| key.eq_ignore_ascii_case(name))
            .map(|(_, value)| value.as_str())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub subscription: Subscription,
    pub event: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Challenge {
    pub challenge: String,
    pub subscription: Subscription,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Revoke {
    pub subscription: Subscription,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MessageType {
    Verification,
    Notification,
    Revocation,
}

impl MessageType {
    pub const fn as_str(&self) -> &'static str {
        match self {
            MessageType::Verification => MESSAGE_TYPE_VERIFICATION,
            MessageType::Notification => MESSAGE_TYPE_NOTIFICATION,
            MessageType::Revocation => MESSAGE_TYPE_REVOCATION,
        }
    }
}

impl Display for MessageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.as_str())
    }
}

impl FromStr for MessageType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            MESSAGE_TYPE_VERIFICATION => Ok(MessageType::Verification),
            MESSAGE_TYPE_NOTIFICATION => Ok(MessageType::Notification),
            MESSAGE_TYPE_REVOCATION => Ok(MessageType::Revocation),
            _ => Err(format!("unknown message type: {}", s)),
        }
    }
}

impl TryFrom<&str> for MessageType {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

#[derive(Debug, Clone)]
pub enum VerificationError {
    MissingHeader(&'static str),
    InvalidHeader(&'static str),
    InvalidSignature,
    UnknownMessageType(String),
    UnknownSubscriptionType(String),
}

impl Display for VerificationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::MissingHeader(header) => write!(f, "missing required header: {}", header),
            Self::InvalidHeader(header) => write!(f, "invalid header format: {}", header),
            Self::InvalidSignature => write!(f, "invalid webhook signature"),
            Self::UnknownMessageType(msg) => write!(f, "unknown message type: {}", msg),
            Self::UnknownSubscriptionType(msg) => write!(f, "unknown subscription type: {}", msg),
        }
    }
}

impl std::error::Error for VerificationError {}

#[cfg(feature = "webhook-axum")]
impl axum_core::response::IntoResponse for VerificationError {
    fn into_response(self) -> axum_core::response::Response {
        match self {
            VerificationError::MissingHeader(_) | VerificationError::InvalidHeader(_) => {
                http::StatusCode::BAD_REQUEST.into_response()
            }
            VerificationError::InvalidSignature => http::StatusCode::FORBIDDEN.into_response(),
            VerificationError::UnknownMessageType(_) | Self::UnknownSubscriptionType(_) => {
                http::StatusCode::NO_CONTENT.into_response()
            }
        }
    }
}
