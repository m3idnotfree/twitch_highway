use asknothingx2_util::api::APIRequest;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use url::Url;

crate::impl_endpoint!(
/// https://dev.twitch.tv/docs/api/reference/#get-shared-chat-session
    GetSharedChatSession {
        broadcaster_id: String,
    };
    new = {
        params = {
            broadcaster_id: impl Into<String>,
        },
        init = {
            broadcaster_id: broadcaster_id.into(),
        }
    },
    url = ["shared_chat","session"]
);

impl APIRequest for GetSharedChatSession {
    crate::impl_api_request_method!(GET);
    crate::impl_api_request_header!();

    fn url(&self) -> Url {
        let mut url = self.get_url();
        url.query_pairs_mut()
            .append_pair("broadcaster_id", &self.broadcaster_id);
        url
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcasterId {
    pub broadcaster_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedChatSession {
    pub session_id: String,
    pub host_broadcaster_id: String,
    pub participants: Vec<BroadcasterId>,
    /// The UTC date and time (in RFC3339 format) for when the session was created.
    pub created_at: DateTime<FixedOffset>,
    /// The UTC date and time (in RFC3339 format) for when the session was last updated.
    pub updated_at: DateTime<FixedOffset>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedChatSessionResponse {
    pub data: Vec<SharedChatSession>,
}
