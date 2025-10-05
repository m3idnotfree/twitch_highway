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

    SubscriptionType::from_str(s).ok()
}

pub fn get_subscription_version<H: HeaderAccess>(headers: &H) -> Option<&str> {
    headers.get_header(SUBSCRIPTION_VERSION)
}
pub fn get_message_retry<H: HeaderAccess>(headers: &H) -> Option<&str> {
    headers.get_header(MESSAGE_RETRY)
}
