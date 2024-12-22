use asknothingx2_util::{
    api::{APIRequest, HeaderBuilder, HeaderMap, Method},
    oauth::{AccessToken, ClientId},
};
use serde::{Deserialize, Serialize};
use url::Url;

use super::Images;

/// https://dev.twitch.tv/docs/api/reference/#get-global-emotes
#[derive(Debug)]
pub struct GetGlobalEmotes {
    access_token: AccessToken,
    client_id: ClientId,
    #[cfg(feature = "test")]
    test_url: crate::test_url::TestUrlHold,
}

impl GetGlobalEmotes {
    pub fn new(access_token: AccessToken, client_id: ClientId) -> Self {
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
    use pretty_assertions::assert_eq;

    use crate::{
        api_general, emotes::EmoteGlobalResponse, expect_APIRequest, expect_headers,
        expect_response_json,
    };

    use super::GetGlobalEmotes;

    #[test]
    fn global_emtoes() {
        let global_emotes = api_general!(GetGlobalEmotes);

        let expected_headers = expect_headers!();

        expect_APIRequest!(
            GET,
            expected_headers,
            "https://api.twitch.tv/helix/chat/emotes/global",
            json = None,
            text = None,
            urlencoded = None,
            global_emotes
        );
    }

    #[test]
    fn global_emotes_response() {
        expect_response_json!("{\n  \"data\": [\n    {\n      \"id\": \"196892\",\n      \"name\": \"TwitchUnity\",\n      \"images\": {\n        \"url_1x\": \"https://static-cdn.jtvnw.net/emoticons/v2/196892/static/light/1.0\",\n        \"url_2x\": \"https://static-cdn.jtvnw.net/emoticons/v2/196892/static/light/2.0\",\n        \"url_4x\": \"https://static-cdn.jtvnw.net/emoticons/v2/196892/static/light/3.0\"\n      },\n      \"format\": [\n        \"static\"\n      ],\n      \"scale\": [\n        \"1.0\",\n        \"2.0\",\n        \"3.0\"\n      ],\n      \"theme_mode\": [\n        \"light\",\n        \"dark\"\n      ]\n    }\n  ],\n  \"template\": \"https://static-cdn.jtvnw.net/emoticons/v2/{{id}}/{{format}}/{{theme_mode}}/{{scale}}\"\n}",
        EmoteGlobalResponse);
    }
}
