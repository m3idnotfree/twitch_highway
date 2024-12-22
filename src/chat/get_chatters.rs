use asknothingx2_util::api::APIRequest;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{impl_endpoint, Pagination};

impl_endpoint!(
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
    crate::impl_api_request_method!(GET);
    crate::impl_api_request_header!();

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

#[cfg(test)]
mod tests {
    use crate::{api_general, expect_APIRequest, expect_headers, expect_response_json};

    use super::{ChattersResponse, GetChatters};

    #[test]
    fn get_chatters() {
        let get_chatters = api_general!(GetChatters, "123456", "654321");

        let expected_headers = expect_headers!();

        expect_APIRequest!(
            GET,
            expected_headers,
            "https://api.twitch.tv/helix/chat/chatters?broadcaster_id=123456&moderator_id=654321",
            json = None,
            text = None,
            urlencoded = None,
            get_chatters
        );
    }

    #[test]
    fn get_chatters_set_first() {
        let mut get_chatters = api_general!(GetChatters, "123456", "654321");

        get_chatters.set_first(40);

        let expected_headers = expect_headers!();

        expect_APIRequest!(
            GET,
            expected_headers,
            "https://api.twitch.tv/helix/chat/chatters?broadcaster_id=123456&moderator_id=654321&first=40",
            json = None,
            text = None,
            urlencoded = None,
            get_chatters
        );
    }
    #[test]
    fn get_chatters_set_after() {
        let mut get_chatters = api_general!(GetChatters, "123456", "654321");

        get_chatters.set_after("eyJiIjpudWxsLCJhIjp7Ik9mZnNldCI6NX19");

        let expected_headers = expect_headers!();

        expect_APIRequest!(
            GET,
            expected_headers,
            "https://api.twitch.tv/helix/chat/chatters?broadcaster_id=123456&moderator_id=654321&after=eyJiIjpudWxsLCJhIjp7Ik9mZnNldCI6NX19",
            json = None,
            text = None,
            urlencoded = None,
            get_chatters
        );
    }

    #[test]
    fn get_chatters_set_first_after() {
        let mut get_chatters = api_general!(GetChatters, "123456", "654321");

        get_chatters.set_first(40);
        get_chatters.set_after("eyJiIjpudWxsLCJhIjp7Ik9mZnNldCI6NX19");

        let expected_headers = expect_headers!();

        expect_APIRequest!(
            GET,
            expected_headers,
            "https://api.twitch.tv/helix/chat/chatters?broadcaster_id=123456&moderator_id=654321&first=40&after=eyJiIjpudWxsLCJhIjp7Ik9mZnNldCI6NX19",
            json = None,
            text = None,
            urlencoded = None,
            get_chatters
        );
    }

    #[test]
    fn chatters() {
        expect_response_json!(
            "{\n    \"data\": [\n        {\n            \"user_id\": \"128393656\",\n            \"user_login\": \"smittysmithers\",\n            \"user_name\": \"smittysmithers\"\n        }\n    ],\n    \"pagination\": {\n        \"cursor\": \"eyJiIjpudWxsLCJhIjp7Ik9mZnNldCI6NX19\"\n    },\n    \"total\": 8\n}",
            ChattersResponse
        );
    }
}
