use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use crate::eventsub::Subscription;

pub(crate) const MESSAGE_ID: &str = "twitch-eventsub-message-id";
pub(crate) const MESSAGE_TIMESTAMP: &str = "twitch-eventsub-message-timestamp";
pub(crate) const MESSAGE_SIGNATURE: &str = "twitch-eventsub-message-signature";

pub(crate) const MESSAGE_RETRY: &str = "twitch-eventsub-message-retry";

pub(crate) const SUBSCRIPTION_TYPE: &str = "twitch-eventsub-subscription-type";
pub(crate) const SUBSCRIPTION_VERSION: &str = "twitch-eventsub-subscription-version";

pub(crate) const MESSAGE_TYPE: &str = "twitch-eventsub-message-type";
pub(crate) const MESSAGE_TYPE_VERIFICATION: &str = "webhook_callback_verification";
pub(crate) const MESSAGE_TYPE_NOTIFICATION: &str = "notification";
pub(crate) const MESSAGE_TYPE_REVOCATION: &str = "revocation";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification<Event> {
    pub subscription: Subscription,
    pub event: Event,
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
