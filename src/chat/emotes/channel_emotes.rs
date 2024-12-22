use asknothingx2_util::{
    api::{APIRequest, HeaderBuilder, HeaderMap, Method},
    oauth::{AccessToken, ClientId},
};
use serde::{Deserialize, Serialize};
use url::Url;

use super::Images;

/// https://dev.twitch.tv/docs/api/reference/#get-channel-emotes
#[derive(Debug)]
pub struct GetChannelEmotes {
    access_token: AccessToken,
    client_id: ClientId,
    broadcaster_id: String,
}

impl GetChannelEmotes {
    pub fn new<T: Into<String>>(
        access_token: AccessToken,
        client_id: ClientId,
        broadcaster_id: T,
    ) -> Self {
        let mut url = Url::parse(crate::TWITCH_API_BASE).unwrap();
        url.path_segments_mut().unwrap().push("chat").push("emotes");

        Self {
            access_token,
            client_id,
            url,
            broadcaster_id: broadcaster_id.into(),
        }
    }
}

impl APIRequest for GetChannelEmotes {
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
        let mut url = self.url.clone();
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

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        api_general, emotes::EmoteChannelResponse, expect_APIRequest, expect_headers,
        expect_response_json,
    };

    use super::GetChannelEmotes;

    #[test]
    fn channel_emotes() {
        let channel_emotes = api_general!(GetChannelEmotes, "141981764");

        let expected_headers = expect_headers!();

        expect_APIRequest!(
            GET,
            expected_headers,
            "https://api.twitch.tv/helix/chat/emotes?broadcaster_id=141981764",
            json = None,
            text = None,
            urlencoded = None,
            channel_emotes
        );
    }

    #[test]
    fn channel_emotes_response() {
        expect_response_json!("{\n  \"data\": [\n    {\n      \"id\": \"304456832\",\n      \"name\": \"twitchdevPitchfork\",\n      \"images\": {\n        \"url_1x\": \"https://static-cdn.jtvnw.net/emoticons/v2/304456832/static/light/1.0\",\n        \"url_2x\": \"https://static-cdn.jtvnw.net/emoticons/v2/304456832/static/light/2.0\",\n        \"url_4x\": \"https://static-cdn.jtvnw.net/emoticons/v2/304456832/static/light/3.0\"\n      },\n      \"tier\": \"1000\",\n      \"emote_type\": \"subscriptions\",\n      \"emote_set_id\": \"301590448\",\n      \"format\": [\n        \"static\"\n      ],\n      \"scale\": [\n        \"1.0\",\n        \"2.0\",\n        \"3.0\"\n      ],\n      \"theme_mode\": [\n        \"light\",\n        \"dark\"\n      ]\n    },\n    {\n      \"id\": \"emotesv2_4c3b4ed516de493bbcd2df2f5d450f49\",\n      \"name\": \"twitchdevHyperPitchfork\",\n      \"images\": {\n        \"url_1x\": \"https://static-cdn.jtvnw.net/emoticons/v2/emotesv2_4c3b4ed516de493bbcd2df2f5d450f49/static/light/1.0\",\n        \"url_2x\": \"https://static-cdn.jtvnw.net/emoticons/v2/emotesv2_4c3b4ed516de493bbcd2df2f5d450f49/static/light/2.0\",\n        \"url_4x\": \"https://static-cdn.jtvnw.net/emoticons/v2/emotesv2_4c3b4ed516de493bbcd2df2f5d450f49/static/light/3.0\"\n      },\n      \"tier\": \"1000\",\n      \"emote_type\": \"subscriptions\",\n      \"emote_set_id\": \"318939165\",\n      \"format\": [\n        \"static\",\n        \"animated\"\n      ],\n      \"scale\": [\n        \"1.0\",\n        \"2.0\",\n        \"3.0\"\n      ],\n      \"theme_mode\": [\n        \"light\",\n        \"dark\"\n      ]\n    }\n  ],\n  \"template\": \"https://static-cdn.jtvnw.net/emoticons/v2/{{id}}/{{format}}/{{theme_mode}}/{{scale}}\"\n}",
EmoteChannelResponse);
    }
}
