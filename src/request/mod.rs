use std::{fmt, marker::PhantomData};

use asknothingx2_util::api::{
    preset, HeaderMap, HeaderMut, IntoRequestBuilder, Method, StatusCode,
};
use reqwest::{Client, RequestBuilder};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use url::Url;

mod endpoint_type;
mod no_content;

pub use endpoint_type::{EndpointType, TokenType};
pub use no_content::NoContent;

#[cfg(any(
    feature = "channel-points",
    feature = "extensions",
    feature = "polls",
    feature = "predictions",
    feature = "schedule",
))]
mod request_body;
#[cfg(any(
    feature = "channel-points",
    feature = "extensions",
    feature = "polls",
    feature = "predictions",
    feature = "schedule",
))]
pub use request_body::RequestBody;

// https://rust-lang.github.io/rust-clippy/master/index.html#wrong_self_convention
#[derive(Debug)]
pub struct TwitchAPIRequest<ResBody> {
    kind: EndpointType,
    url: Url,
    method: Method,
    headers: HeaderMap,
    body: Option<String>,
    client: Client,
    _phantom: PhantomData<ResBody>,
}

impl<ResBody> TwitchAPIRequest<ResBody>
where
    ResBody: DeserializeOwned,
{
    pub fn new(
        kind: EndpointType,
        url: Url,
        method: Method,
        headers: HeaderMap,
        body: Option<String>,
    ) -> Self {
        Self {
            kind,
            headers,
            method,
            url,
            body,
            client: preset::rest_api("twitch-highway/1.0")
                .build_client()
                .unwrap(),
            _phantom: PhantomData,
        }
    }

    pub fn kind(&self) -> &EndpointType {
        &self.kind
    }

    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    pub fn headers_mut(&mut self) -> HeaderMut<'_> {
        HeaderMut::new(&mut self.headers)
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn url(&self) -> &Url {
        &self.url
    }

    pub fn body(&self) -> &Option<String> {
        &self.body
    }

    pub async fn send(self) -> Result<reqwest::Response, crate::Error> {
        let Self {
            kind: _,
            url,
            method,
            headers,
            body,
            client,
            _phantom,
        } = self;

        let mut client = client.request(method, url).headers(headers);
        if let Some(body) = body {
            client = client.body(body);
        }

        client.send().await.map_err(crate::Error::from)
    }

    pub async fn json(self) -> Result<ResBody, crate::Error> {
        let client = self.client.clone();
        self.into_request_builder(&client)
            .unwrap()
            .send()
            .await
            .map_err(crate::Error::from)?
            .json()
            .await
            .map_err(crate::Error::from)
    }
}

#[cfg(test)]
impl<ResBody> TwitchAPIRequest<ResBody>
where
    ResBody: DeserializeOwned,
{
    pub fn set_url(mut self, url: Url) -> Self {
        self.url = url;
        self
    }

    pub fn set_client(mut self, client: Client) -> Self {
        self.client = client;
        self
    }
}

impl<ResBody> IntoRequestBuilder for TwitchAPIRequest<ResBody>
where
    ResBody: DeserializeOwned,
{
    type Error = crate::Error;

    fn into_request_builder(self, client: &Client) -> Result<RequestBuilder, Self::Error> {
        let mut client = client.request(self.method, self.url).headers(self.headers);
        if let Some(body) = self.body {
            client = client.body(body);
        }
        Ok(client)
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
