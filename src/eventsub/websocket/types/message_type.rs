use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::Deref,
    str::FromStr,
};

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
