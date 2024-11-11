use std::sync::Arc;

use asknothingx2_util::{
    api::{APIRequest, HeaderBuilder, HeaderMap, Method},
    oauth::{AccessToken, ClientId},
};
use url::Url;

#[derive(Debug)]
pub struct GetChannelBadge<'a> {
    access_token: Arc<AccessToken>,
    client_id: Arc<ClientId>,
    url: &'a Url,
    broadcaster_id: &'a str,
}

impl<'a> GetChannelBadge<'a> {
    pub fn new(
        access_token: Arc<AccessToken>,
        client_id: Arc<ClientId>,
        url: &'a Url,
        broadcaster_id: &'a str,
    ) -> Self {
        Self {
            access_token,
            client_id,
            url,
            broadcaster_id,
        }
    }
}

impl APIRequest for GetChannelBadge<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn headers(&self) -> HeaderMap {
        HeaderBuilder::new()
            .authorization("Bearer", self.access_token.secret().as_str())
            .append("Client-Id", self.client_id.as_str())
            .unwrap()
            .build()
    }

    fn url(&self) -> Url {
        let mut url = self.url.clone();
        url.query_pairs_mut()
            .append_pair("broadcaster_id", self.broadcaster_id);
        url
    }
}
