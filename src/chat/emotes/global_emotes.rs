use asknothingx2_util::api::APIRequest;
use serde::{Deserialize, Serialize};
use url::Url;

use super::Images;

crate::impl_endpoint!(
/// https://dev.twitch.tv/docs/api/reference/#get-global-emotes
    GetGlobalEmotes {};
    new = {
        params = {},
        init = {}
    },
    url = ["chat","emotes"]
);

impl APIRequest for GetGlobalEmotes {
    crate::impl_api_request_method!(GET);
    crate::impl_api_request_header!();

    fn url(&self) -> Url {
        let mut url = self.get_url();
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
