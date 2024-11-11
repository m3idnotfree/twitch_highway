use std::sync::Arc;

use asknothingx2_util::{
    api::{APIRequest, HeaderBuilder, HeaderMap, Method},
    oauth::{AccessToken, ClientId},
};
use serde::{Deserialize, Serialize};
use url::Url;

use super::Images;

#[derive(Debug)]
pub struct GetChannelEmotes<'a> {
    access_token: Arc<AccessToken>,
    client_id: Arc<ClientId>,
    url: Arc<Url>,
    broadcaster_id: &'a str,
}

impl<'a> GetChannelEmotes<'a> {
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

impl APIRequest for GetChannelEmotes<'_> {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename = "emote")]
pub struct EmoteChannel {
    pub id: String,
    pub name: String,
    pub images: Images,
    pub tier: String,
    pub format: Vec<String>,
    pub scale: Vec<String>,
    pub theme_mode: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmoteChannelResponse {
    pub data: Vec<EmoteChannel>,
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

    use super::GetChannelEmotes;

    #[test]
    fn channel_emotes() {
        let channel_emotes = api_general!(
            GetChannelEmotes,
            "https://api.twitch.tv/helix/chat/emotes",
            "141981764"
        );

        let expected_headers = expect_headers!();

        expect_APIRequest!(
            GET,
            expected_headers,
            "https://api.twitch.tv/helix/chat/emotes?broadcaster_id=141981764",
            channel_emotes
        );
    }
}
