use std::sync::Arc;

use asknothingx2_util::{
    api::{APIRequest, HeaderBuilder, HeaderMap, Method},
    oauth::{AccessToken, ClientId},
};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::Pagination;

/// https://dev.twitch.tv/docs/api/reference/#get-chatters
/// Authorization
/// Requires a user access token that includes the moderator:read:chatters scope.
#[derive(Debug)]
pub struct GetChatters {
    access_token: Arc<AccessToken>,
    client_id: Arc<ClientId>,
    url: Url,
    broadcaster_id: String,
    moderator_id: String,
    first: Option<u64>,
    after: Option<String>,
}

impl GetChatters {
    pub fn new<T: Into<String>>(
        access_token: Arc<AccessToken>,
        client_id: Arc<ClientId>,
        broadcaster_id: T,
        moderator_id: T,
    ) -> Self {
        let mut url = Url::parse(crate::TWITCH_API_BASE).unwrap();
        url.path_segments_mut()
            .unwrap()
            .push("chat")
            .push("chatters");

        Self {
            access_token,
            client_id,
            url,
            broadcaster_id: broadcaster_id.into(),
            moderator_id: moderator_id.into(),
            first: None,
            after: None,
        }
    }

    pub fn set_first(&mut self, first: u64) {
        self.first = Some(first);
    }

    pub fn set_after<T: Into<String>>(&mut self, after: T) {
        self.after = Some(after.into());
    }
}

impl APIRequest for GetChatters {
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
pub struct Chatters {
    pub data: Vec<ChattersData>,
    pub pagination: Option<Pagination>,
    pub total: u64,
}

#[cfg(test)]
mod tests {
    use crate::{api_general, expect_APIRequest, expect_headers, expect_response_json};

    use super::{Chatters, GetChatters};

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
            Chatters
        );
    }
}
