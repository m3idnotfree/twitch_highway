use std::sync::Arc;

use asknothingx2_util::{
    api::{APIRequest, HeaderBuilder, HeaderMap, Method},
    oauth::{AccessToken, ClientId},
};
use serde::{Deserialize, Serialize};
use url::Url;

/// https://dev.twitch.tv/docs/api/reference/#get-chat-settings
#[derive(Debug)]
pub struct GetChatSetting {
    access_token: Arc<AccessToken>,
    client_id: Arc<ClientId>,
    url: Arc<Url>,
    broadcaster_id: String,
    moderator_id: Option<String>,
}

impl GetChatSetting {
    pub fn new<T: Into<String>>(
        access_token: Arc<AccessToken>,
        client_id: Arc<ClientId>,
        url: Arc<Url>,
        broadcaster_id: T,
    ) -> Self {
        Self {
            access_token,
            client_id,
            url,
            broadcaster_id: broadcaster_id.into(),
            moderator_id: None,
        }
    }

    pub fn set<T: Into<String>>(&mut self, moderator_id: T) {
        self.moderator_id = Some(moderator_id.into());
    }
}

impl APIRequest for GetChatSetting {
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
        url.query_pairs_mut()
            .append_pair("broadcaster_id", &self.broadcaster_id);

        if let Some(moderator_id) = &self.moderator_id {
            url.query_pairs_mut()
                .append_pair("moderator_id", moderator_id);
        }

        url
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSetting {
    broadcaster_id: String,
    emote_mode: bool,
    follower_mode: bool,
    follower_mode_duration: Option<u64>,
    /// moderator:read:chat_settings
    moderator_id: Option<String>,
    /// moderator:read:chat_settings
    non_moderator_chat_delay: bool,
    /// The response includes this field only
    /// if the request specifies a user access token
    /// that includes the moderator:read:chat_settings scope
    /// and the user in the moderator_id query parameter is
    /// one of the broadcaster’s moderators.
    non_moderator_chat_delay_duration: Option<u64>,
    slow_mode: bool,
    show_mode_wait_time: Option<u64>,
    subscriber_mode: bool,
    unique_chat_mode: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSettings {
    data: Vec<ChatSetting>,
}
#[cfg(test)]
mod tests {
    use asknothingx2_util::api::APIRequest;

    use crate::{
        api_general, expect_APIRequest, expect_headers, expect_response_json,
        get_chat_setting::ChatSettings,
    };

    use super::GetChatSetting;

    #[test]
    fn get_chat_setting() {
        let chat_setting = api_general!(
            GetChatSetting,
            "https://api.twitch.tv/helix/chat/settings",
            "1234"
        );

        let expected_headers = expect_headers!();

        expect_APIRequest!(
            GET,
            expected_headers,
            "https://api.twitch.tv/helix/chat/settings?broadcaster_id=1234",
            chat_setting
        );

        expect_response_json!("{\n  \"data\": [\n    {\n      \"broadcaster_id\": \"713936733\",\n      \"slow_mode\": false,\n      \"slow_mode_wait_time\": null,\n      \"follower_mode\": true,\n      \"follower_mode_duration\": 0,\n      \"subscriber_mode\": false,\n      \"emote_mode\": false,\n      \"unique_chat_mode\": false,\n      \"non_moderator_chat_delay\": true,\n      \"non_moderator_chat_delay_duration\": 4\n    }\n  ]\n}"
        , ChatSettings);
    }
}
