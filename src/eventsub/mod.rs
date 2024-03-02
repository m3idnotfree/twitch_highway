use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{APIBase, Result};

#[derive(Debug)]
pub struct EventSubAPI {
    data: APIBase,
}

impl EventSubAPI {
    pub fn new<T: Into<String>>(access_token: T, client_id: T) -> EventSubAPI {
        EventSubAPI {
            data: APIBase::new(
                access_token.into(),
                client_id.into(),
                "https://api.twitch.tv/helix/eventsub/subscriptions".into(),
            ),
        }
    }

    pub async fn create<T: Into<String>>(
        &self,
        kind: T,
        version: T,
        condition: HashMap<String, String>,
        session_id: T,
    ) -> Result<String> {
        let body = EventSubCreateRequestBody::websocket(kind, version, condition, session_id);
        self.data.api_post(self.data.url(), &body).await
    }
    pub async fn delete<T: Into<String>>(&self, id: T) -> Result<String> {
        self.data
            .api_delete(self.data.url_qurey("id", &id.into()))
            .await
    }
    pub fn get() {}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventSubCreateRequestBody {
    #[serde(rename = "type")]
    pub kind: String,
    pub version: String,
    pub condition: HashMap<String, String>,
    pub transport: Transport,
}
impl EventSubCreateRequestBody {
    pub fn body() {}
    pub fn webhook() {}

    pub fn websocket<T: Into<String>>(
        kind: T,
        version: T,
        condition: HashMap<String, String>,
        session_id: T,
    ) -> String {
        let transport = Transport {
            method: "websocket".into(),
            callback: None,
            secret: None,
            session_id: Some(session_id.into()),
            conduit_id: None,
        };

        let body = EventSubCreateRequestBody {
            kind: kind.into(),
            version: version.into(),
            condition,
            transport,
        };
        let body = serde_json::to_string(&body).unwrap();
        body
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transport {
    pub method: String,
    pub callback: Option<String>,
    pub secret: Option<String>,
    pub session_id: Option<String>,
    pub conduit_id: Option<String>,
}
