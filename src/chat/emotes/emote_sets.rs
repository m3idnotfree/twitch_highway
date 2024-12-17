use std::sync::Arc;

use asknothingx2_util::{
    api::{APIRequest, HeaderBuilder, HeaderMap, Method},
    oauth::{AccessToken, ClientId},
};
use serde::{Deserialize, Serialize};
use url::Url;

use super::EmoteGlobal;

#[derive(Debug)]
pub struct GetEmoteSets {
    access_token: Arc<AccessToken>,
    client_id: Arc<ClientId>,
    url: Arc<Url>,
    emote_set_ids: Vec<String>,
}

impl GetEmoteSets {
    pub fn new(access_token: Arc<AccessToken>, client_id: Arc<ClientId>, url: Arc<Url>) -> Self {
        Self {
            access_token,
            client_id,
            url,
            emote_set_ids: Vec::new(),
        }
    }
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
        url.path_segments_mut().unwrap().push("set");
        url.query_pairs_mut().extend_pairs(
            self.emote_set_ids
                .iter()
                .map(|x| ("emote_set_id", x.as_str())),
        );

        url
    }
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(tag = "type", rename = "emote")]
// pub struct EmoteSets {
//     pub id: String,
//     pub name: String,
//     pub images: Images,
//     pub emote_type: String,
//     pub emote_set_id: String,
//     pub owner_id: String,
//     pub format: Vec<String>,
//     pub scale: Vec<String>,
//     pub theme_mode: Vec<String>,
// }

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmoteSetsResponse {
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

    use super::GetEmoteSets;

    #[test]
    fn emote_sets() {
        let emote_sets = api_general!(GetEmoteSets, "https://api.twitch.tv/helix/chat/emotes");
        let emote_sets = emote_sets.add_emote_set_id("1234");

        let expected_headers = expect_headers!();

        expect_APIRequest!(
            GET,
            expected_headers,
            "https://api.twitch.tv/helix/chat/emotes/set?emote_set_id=1234",
            emote_sets
        );
    }
    #[test]
    fn emotes_sets_id_vec() {
        let emote_sets = api_general!(GetEmoteSets, "https://api.twitch.tv/helix/chat/emotes");
        let emote_sets = emote_sets.add_emote_set_ids(vec!["1234", "4567"]);

        let expected_headers = expect_headers!();

        expect_APIRequest!(
            GET,
            expected_headers,
            "https://api.twitch.tv/helix/chat/emotes/set?emote_set_id=1234&emote_set_id=4567",
            emote_sets
        );
    }
}
