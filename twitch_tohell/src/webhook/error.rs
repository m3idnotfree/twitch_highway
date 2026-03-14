use std::fmt::{Display, Formatter, Result as FmtResult};

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
