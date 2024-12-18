use std::sync::Arc;

use asknothingx2_util::{
    api::{APIRequest, HeaderBuilder, HeaderMap, Method},
    oauth::{AccessToken, ClientId},
};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use url::Url;

/// https://dev.twitch.tv/docs/api/reference/#get-shared-chat-session
#[derive(Debug)]
pub struct GetSharedChatSession {
    access_token: Arc<AccessToken>,
    client_id: Arc<ClientId>,
    broadcaster_id: String,
    #[cfg(feature = "test")]
    test_url: Option<String>,
}

impl GetSharedChatSession {
    pub fn new<T: Into<String>>(
        access_token: Arc<AccessToken>,
        client_id: Arc<ClientId>,
        broadcaster_id: T,
    ) -> Self {
        Self {
            access_token,
            client_id,
            broadcaster_id: broadcaster_id.into(),
            #[cfg(feature = "test")]
            test_url: None,
        }
    }

    #[cfg(feature = "test")]
    pub fn with_url<T: Into<String>>(&mut self, url: T) -> &mut Self {
        self.test_url = Some(url.into());
        self
    }

    fn get_url(&self) -> Url {
        #[cfg(feature = "test")]
        if let Some(url) = &self.test_url {
            return Url::parse(url).unwrap();
        }

        let mut url = Url::parse(crate::TWITCH_API_BASE).unwrap();
        url.path_segments_mut()
            .unwrap()
            .push("shared_chat")
            .push("session");
        url
    }
}

impl APIRequest for GetSharedChatSession {
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
        let mut url = self.get_url();
        url.query_pairs_mut()
            .append_pair("broadcaster_id", &self.broadcaster_id);
        url
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcasterId {
    broadcaster_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedChatSession {
    session_id: String,
    host_broadcaster_id: String,
    participants: Vec<BroadcasterId>,
    /// The UTC date and time (in RFC3339 format) for when the session was created.
    created_at: DateTime<FixedOffset>,
    /// The UTC date and time (in RFC3339 format) for when the session was last updated.
    updated_at: DateTime<FixedOffset>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedChatSessionResponse {
    data: Vec<SharedChatSession>,
}

#[cfg(test)]
mod tests {
    use chrono::{Datelike, Timelike};

    use crate::{
        api_general, expect_APIRequest, expect_headers,
        get_shared_chat_session::SharedChatSessionResponse,
    };

    use super::GetSharedChatSession;
    #[test]
    fn get_shared_chat_session() {
        let get_shared_chat_session = api_general!(GetSharedChatSession, "198704263");

        expect_APIRequest!(
            GET,
            expect_headers!(),
            "https://api.twitch.tv/helix/shared_chat/session?broadcaster_id=198704263",
            json = None,
            text = None,
            urlencoded = None,
            get_shared_chat_session
        );
    }

    #[test]
    fn get_shared_chat_session_response() {
        let data ="{\n          \"data\": [\n            {\n              \"session_id\": \"359bce59-fa4e-41a5-bd6f-9bc0c8360485\",\n              \"host_broadcaster_id\": \"198704263\",\n              \"participants\": [{\n                  \"broadcaster_id\": \"198704263\"\n              }, {\n                  \"broadcaster_id\": \"487263401\"\n              }],\n              \"created_at\": \"2024-09-29T19:45:37Z\",\n              \"updated_at\": \"2024-09-29T19:45:37Z\"\n            }\n          ]\n        }";
        let shared_chat_sesseion_response: SharedChatSessionResponse =
            serde_json::from_str(data).unwrap();

        let first = shared_chat_sesseion_response.data.first().unwrap();
        assert_eq!(first.created_at.year(), 2024);
        assert_eq!(first.created_at.month(), 9);
        assert_eq!(first.created_at.day(), 29);
        assert_eq!(first.created_at.hour(), 19);
        assert_eq!(first.created_at.minute(), 45);
        assert_eq!(first.created_at.second(), 37);
    }

    #[cfg(feature = "test")]
    #[test]
    fn get_shared_chat_session_feature_test() {
        let mut get_shared_chat_session = api_general!(GetSharedChatSession, "198704263");
        get_shared_chat_session.with_url("https://test.url/shared_chat/session");
        expect_APIRequest!(
            GET,
            expect_headers!(),
            "https://test.url/shared_chat/session?broadcaster_id=198704263",
            json = None,
            text = None,
            urlencoded = None,
            get_shared_chat_session
        );
    }
}
