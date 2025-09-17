use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::Deref,
    str::FromStr,
    sync::OnceLock,
};

use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    SessionWelcome,
    SessionKeepalive,
    Notification,
    SessionReconnect,
    Revocation,
}

impl MessageType {
    pub const fn as_str(&self) -> &'static str {
        match self {
            MessageType::SessionWelcome => "session_welcome",
            MessageType::SessionKeepalive => "session_keepalive",
            MessageType::Notification => "notification",
            MessageType::SessionReconnect => "session_reconnect",
            MessageType::Revocation => "revocation",
        }
    }

    pub fn detect(input: &str) -> Option<Self> {
        Self::all()
            .iter()
            .find(|&&msg_type| msg_type.matches(input))
            .copied()
    }

    pub fn is_session_welcome(input: &str) -> bool {
        Self::SessionWelcome.matches(input)
    }

    pub fn is_session_keepalive(input: &str) -> bool {
        Self::SessionKeepalive.matches(input)
    }

    pub fn is_notification(input: &str) -> bool {
        Self::Notification.matches(input)
    }

    pub fn is_session_reconnect(input: &str) -> bool {
        Self::SessionReconnect.matches(input)
    }

    pub fn is_revocation(input: &str) -> bool {
        Self::Revocation.matches(input)
    }

    const fn all() -> &'static [MessageType] {
        &[
            Self::SessionKeepalive,
            Self::Notification,
            Self::SessionWelcome,
            Self::SessionReconnect,
            Self::Revocation,
        ]
    }

    const fn strict_pattern(&self) -> &'static str {
        match self {
            Self::SessionWelcome => r#""message_type":"session_welcome""#,
            Self::SessionKeepalive => r#""message_type":"session_keepalive""#,
            Self::Notification => r#""message_type":"notification""#,
            Self::SessionReconnect => r#""message_type":"session_reconnect""#,
            Self::Revocation => r#""message_type":"revocation""#,
        }
    }

    fn regex_pattern(&self) -> &'static Regex {
        match self {
            Self::SessionWelcome => {
                static REGEX: OnceLock<Regex> = OnceLock::new();
                REGEX.get_or_init(|| {
                    Regex::new(r#""message_type"\s*:\s*"session_welcome""#).unwrap()
                })
            }
            Self::SessionKeepalive => {
                static REGEX: OnceLock<Regex> = OnceLock::new();
                REGEX.get_or_init(|| {
                    Regex::new(r#""message_type"\s*:\s*"session_keepalive""#).unwrap()
                })
            }
            Self::Notification => {
                static REGEX: OnceLock<Regex> = OnceLock::new();
                REGEX.get_or_init(|| Regex::new(r#""message_type"\s*:\s*"notification""#).unwrap())
            }
            Self::SessionReconnect => {
                static REGEX: OnceLock<Regex> = OnceLock::new();
                REGEX.get_or_init(|| {
                    Regex::new(r#""message_type"\s*:\s*"session_reconnect""#).unwrap()
                })
            }
            Self::Revocation => {
                static REGEX: OnceLock<Regex> = OnceLock::new();
                REGEX.get_or_init(|| Regex::new(r#""message_type"\s*:\s*"revocation""#).unwrap())
            }
        }
    }

    fn matches(&self, input: &str) -> bool {
        if input.contains(self.strict_pattern()) {
            return true;
        }

        self.regex_pattern().is_match(input)
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
            "session_welcome" => Ok(Self::SessionWelcome),
            "session_keepalive" => Ok(Self::SessionKeepalive),
            "notification" => Ok(Self::Notification),
            "session_reconnect" => Ok(Self::SessionReconnect),
            "revocation" => Ok(Self::Revocation),
            _ => Err(format!("Unknown message type: {}", s)),
        }
    }
}

impl Deref for MessageType {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}
