use std::sync::LazyLock;

use asknothingx2_util::{
    api::{preset, HeaderMut},
    oauth::{AccessToken, ClientId},
};
use reqwest::header::HeaderMap;
use serde::de::DeserializeOwned;
use url::Url;

use crate::{error, Error};

const TWITCH_API_BASE: &str = "https://api.twitch.tv/helix";

static BASE_URL: LazyLock<Url> = LazyLock::new(|| url::Url::parse(TWITCH_API_BASE).unwrap());

#[derive(Debug, Clone)]
pub struct Client {
    access_token: AccessToken,
    client_id: ClientId,
    url: Url,
    client: reqwest::Client,
}

impl Client {
    pub fn new(access_token: AccessToken, client_id: ClientId) -> Self {
        let mut client = preset::rest_api(concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION")
        ));

        client
            .default_headers_mut()
            .bearer_token(access_token.secret())
            .client_id(&client_id)
            .unwrap();

        Self {
            access_token,
            client_id,
            url: BASE_URL.clone(),
            client: client.build().unwrap(),
        }
    }

    pub fn with_client_builder(
        access_token: AccessToken,
        client_id: ClientId,
        builder: reqwest::ClientBuilder,
    ) -> Self {
        use reqwest::header;
        let mut headers = header::HeaderMap::new();
        HeaderMut::new(&mut headers)
            .bearer_token(access_token.secret())
            .client_id(&client_id)
            .unwrap();

        let client = builder.default_headers(headers);

        Self {
            access_token,
            client_id,
            url: BASE_URL.clone(),
            client: client.build().unwrap(),
        }
    }

    pub fn with_url(mut self, url: Url) -> Self {
        self.url = url;
        self
    }

    #[allow(unused)]
    pub(crate) fn default_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        HeaderMut::new(&mut headers)
            .bearer_token(self.access_token.secret())
            .client_id(&self.client_id)
            .unwrap();
        headers
    }

    pub(crate) fn header_json(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        HeaderMut::new(&mut headers)
            .bearer_token(self.access_token.secret())
            .client_id(&self.client_id)
            .unwrap()
            .content_type_json();
        headers
    }

    #[allow(unused)]
    pub(crate) fn base_url(&self) -> Url {
        self.url.clone()
    }

    #[allow(unused)]
    pub(crate) fn http_client(&self) -> &reqwest::Client {
        &self.client
    }

    pub(crate) async fn json<T: DeserializeOwned>(
        &self,
        req: reqwest::RequestBuilder,
    ) -> Result<T, Error> {
        self.execute(req)
            .await?
            .json()
            .await
            .map_err(error::decode_error)
    }

    pub(crate) async fn text(&self, req: reqwest::RequestBuilder) -> Result<String, Error> {
        self.execute(req)
            .await?
            .text()
            .await
            .map_err(error::decode_error)
    }

    pub(crate) async fn no_content(&self, req: reqwest::RequestBuilder) -> Result<(), Error> {
        self.execute(req).await?;
        Ok(())
    }

    pub(crate) async fn execute(
        &self,
        req: reqwest::RequestBuilder,
    ) -> Result<reqwest::Response, Error> {
        let resp = req.send().await.map_err(Error::from)?;
        if resp.status().is_success() {
            Ok(resp)
        } else {
            let status = resp.status();
            let v = resp.bytes().await?;
            let body = String::from_utf8_lossy(&v);
            Err(error::api_error(format!("HTTP {status}: {body}")))
        }
    }
}
