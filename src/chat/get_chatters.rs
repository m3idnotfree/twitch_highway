use asknothingx2_util::api::APIRequest;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::types::Pagination;

endpoint!(
/// https://dev.twitch.tv/docs/api/reference/#get-chatters
/// Authorization
/// Requires a user access token that includes the moderator:read:chatters scope.
    GetChatters {
        broadcaster_id: String,
        moderator_id: String,
        first: Option<u64>,
        after: Option<String>,
    };
    new = {
        params = {
            broadcaster_id: impl Into<String>,
            moderator_id: impl Into<String>,
        },
        init = {
            broadcaster_id: broadcaster_id.into(),
            moderator_id: moderator_id.into(),
            first: None,
            after: None,
        }
    },
    url = ["chat","chatters"]
);

impl GetChatters {
    pub fn set_first(&mut self, first: u64) {
        self.first = Some(first);
    }

    pub fn set_after<T: Into<String>>(&mut self, after: T) {
        self.after = Some(after.into());
    }
}

impl APIRequest for GetChatters {
    impl_api_request_method!(GET);
    impl_api_request_header!();

    fn url(&self) -> Url {
        let mut url = self.get_url();
        url.query_pairs_mut()
            .append_pair("broadcaster_id", &self.broadcaster_id)
            .append_pair("moderator_id", &self.moderator_id);
        if let Some(first) = self.first {
            url.query_pairs_mut()
                .append_pair("first", &first.to_string());
        };

        if let Some(after) = &self.after {
            url.query_pairs_mut()
                .append_pair("after", &after.to_string());
        };

        url
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChattersData {
    pub user_id: String,
    pub user_login: String,
    pub user_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChattersResponse {
    pub data: Vec<ChattersData>,
    pub pagination: Option<Pagination>,
    pub total: u64,
}
