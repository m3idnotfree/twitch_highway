use asknothingx2_util::api::APIRequest;
use serde::{Deserialize, Serialize};
use url::Url;

use super::EmoteGlobal;

crate::impl_endpoint!(
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
    crate::impl_api_request_method!(GET);
    crate::impl_api_request_header!();

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

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        api_general, emotes::EmoteSetsResponse, expect_APIRequest, expect_headers,
        expect_response_json,
    };

    use super::GetEmoteSets;

    #[test]
    fn emote_sets() {
        let emote_sets = api_general!(GetEmoteSets);
        let emote_sets = emote_sets.add_emote_set_id("1234");

        let expected_headers = expect_headers!();

        expect_APIRequest!(
            GET,
            expected_headers,
            "https://api.twitch.tv/helix/chat/emotes/set?emote_set_id=1234",
            json = None,
            text = None,
            urlencoded = None,
            emote_sets
        );
    }
    #[test]
    fn emotes_sets_id_vec() {
        let emote_sets = api_general!(GetEmoteSets);
        let emote_sets = emote_sets.add_emote_set_ids(vec!["1234", "4567"]);

        let expected_headers = expect_headers!();

        expect_APIRequest!(
            GET,
            expected_headers,
            "https://api.twitch.tv/helix/chat/emotes/set?emote_set_id=1234&emote_set_id=4567",
            json = None,
            text = None,
            urlencoded = None,
            emote_sets
        );
    }

    #[test]
    fn emote_sets_response() {
        expect_response_json!("{\n  \"data\": [\n    {\n      \"id\": \"304456832\",\n      \"name\": \"twitchdevPitchfork\",\n      \"images\": {\n        \"url_1x\": \"https://static-cdn.jtvnw.net/emoticons/v2/304456832/static/light/1.0\",\n        \"url_2x\": \"https://static-cdn.jtvnw.net/emoticons/v2/304456832/static/light/2.0\",\n        \"url_4x\": \"https://static-cdn.jtvnw.net/emoticons/v2/304456832/static/light/3.0\"\n      },\n      \"emote_type\": \"subscriptions\",\n      \"emote_set_id\": \"301590448\",\n      \"owner_id\": \"141981764\",\n      \"format\": [\n        \"static\"\n      ],\n      \"scale\": [\n        \"1.0\",\n        \"2.0\",\n        \"3.0\"\n      ],\n      \"theme_mode\": [\n        \"light\",\n        \"dark\"\n      ]\n    }\n  ],\n  \"template\": \"https://static-cdn.jtvnw.net/emoticons/v2/{{id}}/{{format}}/{{theme_mode}}/{{scale}}\"\n}",
        EmoteSetsResponse);
    }
}
