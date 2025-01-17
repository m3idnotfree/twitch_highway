use std::{fmt, marker::PhantomData};

use asknothingx2_util::api::{api_request, APIRequest, APIResponse, HeaderMap, Method, StatusCode};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use url::Url;

use crate::{Error, Response};

mod empty_body;
mod endpoint_type;

pub use empty_body::EmptyBody;
pub use endpoint_type::{EndpointType, TokenType};

#[cfg(any(
    feature = "ccls",
    feature = "channel_points",
    feature = "channels",
    feature = "charity",
    feature = "chat",
    feature = "clips",
    feature = "entitlements",
    feature = "extensions",
    feature = "polls",
    feature = "predictions",
    feature = "schedule",
))]
mod request_body;
#[cfg(any(
    feature = "ccls",
    feature = "channel_points",
    feature = "channels",
    feature = "charity",
    feature = "chat",
    feature = "clips",
    feature = "entitlements",
    feature = "extensions",
    feature = "polls",
    feature = "predictions",
    feature = "schedule",
))]
pub use request_body::RequestBody;

// https://rust-lang.github.io/rust-clippy/master/index.html#wrong_self_convention
pub trait IntoRequestBody {
    fn as_body(&self) -> Option<String> {
        None
    }
}

pub struct TwitchAPIRequest<T, D> {
    kind: EndpointType,
    url: Url,
    method: Method,
    header: HeaderMap,
    body: T,
    _phatom: PhantomData<D>,
    #[cfg(feature = "test")]
    pub test_url: crate::test_url::TestUrlHold,
}

impl<T, D> TwitchAPIRequest<T, D>
where
    T: IntoRequestBody,
    D: DeserializeOwned,
{
    pub fn new(kind: EndpointType, url: Url, method: Method, header: HeaderMap, body: T) -> Self {
        Self {
            kind,
            header,
            method,
            url,
            body,
            _phatom: PhantomData,
            #[cfg(feature = "test")]
            test_url: crate::test_url::TestUrlHold::default(),
        }
    }
    pub fn kind(&self) -> &EndpointType {
        &self.kind
    }
    pub fn header(&self) -> &HeaderMap {
        &self.header
    }
    pub fn method(&self) -> &Method {
        &self.method
    }
    pub fn url(&self) -> Url {
        #[cfg(feature = "test")]
        if let Ok(url) = self.test_url.from_url(&self.url) {
            return url;
        }
        self.url.clone()
    }
    pub fn body(&self) -> &T {
        &self.body
    }

    pub async fn request(self) -> Result<Response<D>, Error> {
        let response = api_request(self).await?;
        Ok(Response::new(APIResponse::from_response(response).await?))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct APIError {
    #[serde(with = "http_serde::status_code")]
    status: StatusCode,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

impl APIError {
    pub fn status(&self) -> StatusCode {
        self.status
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn error_details(&self) -> Option<&str> {
        self.error.as_deref()
    }
}

impl fmt::Display for APIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "API error ({}): {}{}",
            self.status,
            self.message,
            self.error
                .as_ref()
                .map(|e| format!(" - {}", e))
                .unwrap_or_default()
        )
    }
}

impl<T, D> APIRequest for TwitchAPIRequest<T, D>
where
    T: IntoRequestBody,
    D: DeserializeOwned,
{
    fn url(&self) -> Url {
        self.url()
    }
    fn method(&self) -> Method {
        self.method.clone()
    }
    fn headers(&self) -> HeaderMap {
        self.header.clone()
    }
    fn json(&self) -> Option<String> {
        self.body.as_body()
    }
    fn text(&self) -> Option<Vec<u8>> {
        self.body.as_body().map(|x| x.into_bytes())
    }
    fn urlencoded(&self) -> Option<Vec<u8>> {
        self.body.as_body().map(|x| x.into_bytes())
    }
}

#[cfg(feature = "test")]
impl<L, D> crate::test_url::TestUrl for TwitchAPIRequest<L, D> {
    fn with_url(mut self, port: Option<u16>, endpoint: Option<String>) -> Self {
        self.test_url.with_endpoint(endpoint);
        self.test_url.with_port(port);

        self
    }
}
