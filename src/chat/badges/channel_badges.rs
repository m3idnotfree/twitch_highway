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
    url: Arc<Url>,
    broadcaster_id: &'a str,
}

impl<'a> GetChannelBadge<'a> {
    pub fn new(
        access_token: Arc<AccessToken>,
        client_id: Arc<ClientId>,
        url: Arc<Url>,
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
            .client_id(self.client_id.as_str())
            .build()
    }

    fn url(&self) -> Url {
        let mut url = Url::parse(self.url.as_str()).unwrap();
        url.query_pairs_mut()
            .append_pair("broadcaster_id", self.broadcaster_id);
        url
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use asknothingx2_util::{
        api::{APIRequest, HeaderBuilder, Method},
        oauth::{AccessToken, ClientId},
    };
    use pretty_assertions::assert_eq;
    use url::Url;

    use crate::{api_general, expect_APIRequest, expect_headers};

    use super::GetChannelBadge;

    #[test]
    fn channel_badges() {
        let channel_badges = api_general!(
            GetChannelBadge,
            "https://api.twitch.tv/helix/chat/badges",
            "135093069"
        );

        let expected_headers = expect_headers!();

        expect_APIRequest!(
            GET,
            expected_headers,
            "https://api.twitch.tv/helix/chat/badges?broadcaster_id=135093069",
            channel_badges
        );
    }
}
