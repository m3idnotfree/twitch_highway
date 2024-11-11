use std::sync::Arc;

use asknothingx2_util::{
    api::{APIRequest, HeaderBuilder, HeaderMap, Method},
    oauth::{AccessToken, ClientId},
};
use serde::{Deserialize, Serialize};
use url::Url;

use super::Images;

#[derive(Debug)]
pub struct GetGlobalEmotes {
    access_token: Arc<AccessToken>,
    client_id: Arc<ClientId>,
    url: Arc<Url>,
}
impl GetGlobalEmotes {
    pub fn new(access_token: Arc<AccessToken>, client_id: Arc<ClientId>, url: Arc<Url>) -> Self {
        Self {
            access_token,
            client_id,
            url,
        }
    }
}

impl APIRequest for GetGlobalEmotes {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename = "emote")]
pub struct EmoteGlobal {
    pub id: String,
    pub name: String,
    pub images: Images,
    pub format: Vec<String>,
    pub scale: Vec<String>,
    pub theme_mode: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmoteGlobalResponse {
    pub data: Vec<EmoteGlobal>,
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

    use super::GetGlobalEmotes;

    #[test]
    fn global_emtoes() {
        let global_emotes =
            api_general!(GetGlobalEmotes, "https://api.twitch.tv/helix/chat/emotes");

        let expected_headers = expect_headers!();

        expect_APIRequest!(
            GET,
            expected_headers,
            "https://api.twitch.tv/helix/chat/emotes/global",
            global_emotes
        );
    }
}
