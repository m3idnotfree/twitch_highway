use std::sync::Arc;

use asknothingx2_util::{
    api::{APIRequest, HeaderBuilder, Method},
    oauth::{AccessToken, ClientId},
};
use url::Url;

#[derive(Debug)]
pub struct GetGlobalBadges<'a> {
    access_token: Arc<AccessToken>,
    client_id: Arc<ClientId>,
    url: &'a Url,
}

impl<'a> GetGlobalBadges<'a> {
    pub fn new(access_token: Arc<AccessToken>, client_id: Arc<ClientId>, url: &'a Url) -> Self {
        Self {
            access_token,
            client_id,
            url,
        }
    }
}

impl APIRequest for GetGlobalBadges<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn headers(&self) -> reqwest::header::HeaderMap {
        HeaderBuilder::new()
            .authorization("Bearer", self.access_token.secret().as_str())
            .append("Client-Id", self.client_id.as_str())
            .unwrap()
            .build()
    }

    fn url(&self) -> Url {
        let mut url = self.url.clone();
        url.set_path("global");
        url
    }
}
