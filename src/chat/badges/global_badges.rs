use std::sync::Arc;

use asknothingx2_util::{
    api::{APIRequest, HeaderBuilder, HeaderMap, Method},
    oauth::{AccessToken, ClientId},
};
use url::Url;

#[derive(Debug)]
pub struct GetGlobalBadges {
    access_token: Arc<AccessToken>,
    client_id: Arc<ClientId>,
    url: Arc<Url>,
}

impl GetGlobalBadges {
    pub fn new(access_token: Arc<AccessToken>, client_id: Arc<ClientId>, url: Arc<Url>) -> Self {
        Self {
            access_token,
            client_id,
            url,
        }
    }
}

impl APIRequest for GetGlobalBadges {
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
        url.path_segments_mut().unwrap().push("global");
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

    use super::GetGlobalBadges;

    #[test]
    fn global_badges() {
        let global_badges =
            api_general!(GetGlobalBadges, "https://api.twitch.tv/helix/chat/badges");

        let expected_headers = expect_headers!();

        expect_APIRequest!(
            GET,
            expected_headers,
            "https://api.twitch.tv/helix/chat/badges/global",
            global_badges
        );
    }
}
