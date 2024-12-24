use asknothingx2_util::api::APIRequest;
use serde::{Deserialize, Serialize};
use url::Url;

use super::Images;

endpoint!(
/// https://dev.twitch.tv/docs/api/reference/#get-global-emotes
    GetGlobalEmotes {};
    new = {
        params = {},
        init = {}
    },
    url = ["chat","emotes"]
);

impl APIRequest for GetGlobalEmotes {
    impl_api_request_method!(GET);
    impl_api_request_header!();

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
