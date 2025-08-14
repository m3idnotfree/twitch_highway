mod endpoint_type;
mod no_content;

use std::{fmt, marker::PhantomData};

use asknothingx2_util::api::{
    preset, HeaderMap, HeaderMut, IntoRequestBuilder, Method, StatusCode,
};
use reqwest::{Client, RequestBuilder};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use url::Url;

use crate::{error, Error};

pub use endpoint_type::{EndpointType, TokenType};
pub use no_content::NoContent;

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

    async fn send(self) -> Result<reqwest::Response, Error> {
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

        client.send().await.map_err(Error::from)
    }

    pub async fn text(self) -> Result<String, Error> {
        let resp = self.send().await?;

        if !resp.status().is_success() {
            let status = resp.status();
            match resp.text().await {
                Ok(body) => {
                    return Err(error::api_error(format!("HTTP {status}: {body}")));
                }
                Err(e) => {
                    return Err(error::api_error(format!(
                        "HTTP {status} - Failed to read error response: {e}"
                    )));
                }
            }
        }

        resp.text().await.map_err(error::decode_error)
    }

    pub async fn json(self) -> Result<ResBody, crate::Error> {
        let resp = self.send().await?;

        if !resp.status().is_success() {
            let status = resp.status();
            match resp.text().await {
                Ok(body) => {
                    return Err(error::api_error(format!("HTTP {status}: {body}")));
                }
                Err(e) => {
                    return Err(error::api_error(format!(
                        "HTTP {status} - Failed to read error response: {e}"
                    )));
                }
            }
        }

        resp.json().await.map_err(error::decode_error)
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

#[cfg(any(
    feature = "channel-points",
    feature = "extensions",
    feature = "polls",
    feature = "predictions",
    feature = "schedule",
))]
#[derive(Serialize, Deserialize)]
pub struct RequestBody<Required, Optional> {
    #[serde(flatten)]
    required: Required,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    opt: Option<Optional>,
}

#[cfg(any(
    feature = "channel-points",
    feature = "extensions",
    feature = "polls",
    feature = "predictions",
    feature = "schedule",
))]
impl<Required, Optional> RequestBody<Required, Optional>
where
    Required: Serialize,
    Optional: Serialize,
{
    pub fn new(required: Required, opts: Option<Optional>) -> Self {
        Self {
            required,
            opt: opts,
        }
    }

    pub fn into_json(&self) -> Option<String> {
        Some(serde_json::to_string(self).unwrap())
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
