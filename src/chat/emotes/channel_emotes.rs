use asknothingx2_util::api::APIRequest;
use serde::{Deserialize, Serialize};
use url::Url;

use super::Images;

endpoint!(
/// https://dev.twitch.tv/docs/api/reference/#get-channel-emotes
    GetChannelEmotes {
        broadcaster_id: String,
    };
    new = {
        params = {
            broadcaster_id: impl Into<String>,
        },
        init = {
            broadcaster_id: broadcaster_id.into(),
        }
    },
    url = ["chat","emotes"]
);

impl APIRequest for GetChannelEmotes {
    impl_api_request_method!(GET);
    impl_api_request_header!();

    fn url(&self) -> Url {
        let mut url = self.get_url();
        url.query_pairs_mut()
            .append_pair("broadcaster_id", self.broadcaster_id.as_str());
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
