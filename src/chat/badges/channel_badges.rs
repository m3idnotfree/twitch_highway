use std::sync::Arc;

use asknothingx2_util::{
    api::{APIRequest, HeaderBuilder, HeaderMap, Method},
    oauth::{AccessToken, ClientId},
};
use url::Url;

/// https://dev.twitch.tv/docs/api/reference/#get-channel-chat-badges
#[derive(Debug)]
pub struct GetChannelBadge {
    access_token: Arc<AccessToken>,
    client_id: Arc<ClientId>,
    url: Url,
    broadcaster_id: String,
}

impl GetChannelBadge {
    pub fn new<T: Into<String>>(
        access_token: Arc<AccessToken>,
        client_id: Arc<ClientId>,
        broadcaster_id: T,
    ) -> Self {
        let mut url = Url::parse(crate::TWITCH_API_BASE).unwrap();
        url.path_segments_mut().unwrap().push("chat").push("badges");

        Self {
            access_token,
            client_id,
            url,
            broadcaster_id: broadcaster_id.into(),
        }
    }
}

impl APIRequest for GetChannelBadge {
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
            .append_pair("broadcaster_id", &self.broadcaster_id);
        url
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{api_general, expect_APIRequest, expect_headers};

    use super::GetChannelBadge;

    #[test]
    fn channel_badges() {
        let channel_badges = api_general!(GetChannelBadge, "135093069");

        let expected_headers = expect_headers!();

        expect_APIRequest!(
            GET,
            expected_headers,
            "https://api.twitch.tv/helix/chat/badges?broadcaster_id=135093069",
            json = None,
            text = None,
            urlencoded = None,
            channel_badges
        );
    }
}
