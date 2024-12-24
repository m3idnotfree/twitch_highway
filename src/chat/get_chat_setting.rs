use asknothingx2_util::api::APIRequest;
use serde::{Deserialize, Serialize};
use url::Url;

endpoint!(
/// https://dev.twitch.tv/docs/api/reference/#get-chat-settings
    GetChatSetting {
        broadcaster_id: String,
        moderator_id: Option<String>,
    };
    new = {
        params = {
            broadcaster_id: impl Into<String>,
        },
        init = {
            broadcaster_id: broadcaster_id.into(),
            moderator_id: None,
        }
    },
    url = ["chat","settings"]
);

impl GetChatSetting {
    pub fn set_moderator_id<T: Into<String>>(&mut self, moderator_id: T) {
        self.moderator_id = Some(moderator_id.into());
    }
}

impl APIRequest for GetChatSetting {
    impl_api_request_method!(GET);
    impl_api_request_header!();

    fn url(&self) -> Url {
        let mut url = self.get_url();
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
    pub broadcaster_id: String,
    pub emote_mode: bool,
    pub follower_mode: bool,
    pub follower_mode_duration: Option<u64>,
    /// only includes the moderator:read:chat_settings scope
    pub moderator_id: Option<String>,
    /// only includes the moderator:read:chat_settings scope
    pub non_moderator_chat_delay: bool,
    /// The response includes this field only
    /// if the request specifies a user access token that includes the
    /// moderator:read:chat_settings scope
    /// and the user in the moderator_id query parameter is
    /// one of the broadcaster’s moderators.
    pub non_moderator_chat_delay_duration: Option<u64>,
    pub slow_mode: bool,
    pub show_mode_wait_time: Option<u64>,
    pub subscriber_mode: bool,
    pub unique_chat_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSettingResponse {
    pub data: Vec<ChatSetting>,
}
