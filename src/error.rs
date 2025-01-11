use asknothingx2_util::api;

use crate::request::APIError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("URL parse error {0}")]
    UrlParseError(#[from] url::ParseError),
    #[error("Missing Test URL")]
    MissingTestUrl,
    #[error("Failed to deserialize response: {0}")]
    DeserializationError(String),
    #[error("API request failed: {0}")]
    APIRequestFailed(#[from] api::ReqwestError),
    #[error("Failed to deserialize successful response: {0}")]
    ResponseDeserializeError(#[from] serde_json::Error),
    #[error("API returned error response: {0}")]
    TwitchAPIError(APIError),
}
