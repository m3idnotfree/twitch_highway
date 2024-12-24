use asknothingx2_util::api::APIRequest;
use serde::{Deserialize, Serialize};
use url::Url;

use super::EmoteGlobal;

endpoint!(
/// https://dev.twitch.tv/docs/api/reference/#get-emote-sets
    GetEmoteSets {
        emote_set_ids: Vec<String>,
    };
    new = {
        params = {},
        init = {
            emote_set_ids: Vec::new(),
        }
    },
    url = ["chat","emotes","set"]
);

impl GetEmoteSets {
    pub fn add_emote_set_id<T: Into<String>>(mut self, id: T) -> Self {
        self.emote_set_ids.push(id.into());
        self
    }

    pub fn add_emote_set_ids<T: Into<String>, L: IntoIterator<Item = T>>(mut self, ids: L) -> Self {
        self.emote_set_ids.extend(ids.into_iter().map(Into::into));
        self
    }
}

impl APIRequest for GetEmoteSets {
    impl_api_request_method!(GET);
    impl_api_request_header!();

    fn url(&self) -> Url {
        let mut url = self.get_url();
        url.query_pairs_mut().extend_pairs(
            self.emote_set_ids
                .iter()
                .map(|x| ("emote_set_id", x.as_str())),
        );

        url
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmoteSetsResponse {
    pub data: Vec<EmoteGlobal>,
}
