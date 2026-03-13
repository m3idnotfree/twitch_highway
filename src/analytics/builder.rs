use chrono::{DateTime, Utc};

use crate::{
    analytics::{AnalyticsType, ExtensionAnalyticsResponse, GameAnalyticsResponse},
    types::{
        constants::{
            AFTER, ANALYTICS, ENDED_AT, EXTENSIONS, EXTENSION_ID, FIRST, GAMES, GAME_ID,
            STARTED_AT, TYPE,
        },
        ExtensionId, GameId,
    },
    Client, Error,
};

#[derive(Debug)]
pub struct GetExtensionAnalytics<'a> {
    client: &'a Client,
    extension_id: Option<&'a ExtensionId>,
    kind: Option<AnalyticsType>,
    started_at: Option<&'a DateTime<Utc>>,
    ended_at: Option<&'a DateTime<Utc>>,
    first: Option<u8>,
    after: Option<&'a str>,
}

impl<'a> GetExtensionAnalytics<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self {
            client,
            extension_id: None,
            kind: None,
            started_at: None,
            ended_at: None,
            first: None,
            after: None,
        }
    }

    pub fn extension_id(mut self, value: &'a ExtensionId) -> Self {
        self.extension_id = Some(value);
        self
    }

    pub fn kind(mut self, value: AnalyticsType) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn started_at(mut self, value: &'a DateTime<Utc>) -> Self {
        self.started_at = Some(value);
        self
    }

    pub fn ended_at(mut self, value: &'a DateTime<Utc>) -> Self {
        self.ended_at = Some(value);
        self
    }

    pub fn first(mut self, value: u8) -> Self {
        self.first = Some(value);
        self
    }

    pub fn after(mut self, value: &'a str) -> Self {
        self.after = Some(value);
        self
    }

    pub async fn send(self) -> Result<ExtensionAnalyticsResponse, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([ANALYTICS, EXTENSIONS]);

        if let Some(val) = self.extension_id {
            url.query_pairs_mut().append_pair(EXTENSION_ID, val);
        }
        if let Some(val) = self.kind {
            url.query_pairs_mut().append_pair(TYPE, val.as_ref());
        }
        if let Some(val) = self.started_at {
            url.query_pairs_mut().append_pair(
                STARTED_AT,
                &val.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
            );
        }
        if let Some(val) = self.ended_at {
            url.query_pairs_mut().append_pair(
                ENDED_AT,
                &val.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
            );
        }
        if let Some(val) = self.first {
            url.query_pairs_mut().append_pair(FIRST, &val.to_string());
        }
        if let Some(val) = self.after {
            url.query_pairs_mut().append_pair(AFTER, val);
        }

        let req = self.client.http_client().get(url);
        self.client.json(req).await
    }
}

#[derive(Debug)]
pub struct GetGameAnalytics<'a> {
    client: &'a Client,
    game_id: Option<&'a GameId>,
    kind: Option<AnalyticsType>,
    started_at: Option<&'a DateTime<Utc>>,
    ended_at: Option<&'a DateTime<Utc>>,
    first: Option<u8>,
    after: Option<&'a str>,
}

impl<'a> GetGameAnalytics<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self {
            client,
            game_id: None,
            kind: None,
            started_at: None,
            ended_at: None,
            first: None,
            after: None,
        }
    }

    pub fn game_id(mut self, value: &'a GameId) -> Self {
        self.game_id = Some(value);
        self
    }

    pub fn kind(mut self, value: AnalyticsType) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn started_at(mut self, value: &'a DateTime<Utc>) -> Self {
        self.started_at = Some(value);
        self
    }

    pub fn ended_at(mut self, value: &'a DateTime<Utc>) -> Self {
        self.ended_at = Some(value);
        self
    }

    pub fn first(mut self, value: u8) -> Self {
        self.first = Some(value);
        self
    }

    pub fn after(mut self, value: &'a str) -> Self {
        self.after = Some(value);
        self
    }

    pub async fn send(self) -> Result<GameAnalyticsResponse, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut().unwrap().extend([ANALYTICS, GAMES]);

        if let Some(val) = self.game_id {
            url.query_pairs_mut().append_pair(GAME_ID, val);
        }

        if let Some(val) = self.kind {
            url.query_pairs_mut().append_pair(TYPE, val.as_ref());
        }

        if let Some(val) = self.started_at {
            url.query_pairs_mut().append_pair(
                STARTED_AT,
                &val.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
            );
        }
        if let Some(val) = self.ended_at {
            url.query_pairs_mut().append_pair(
                ENDED_AT,
                &val.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
            );
        }
        if let Some(val) = self.first {
            url.query_pairs_mut().append_pair(FIRST, &val.to_string());
        }
        if let Some(val) = self.after {
            url.query_pairs_mut().append_pair(AFTER, val);
        }

        let req = self.client.http_client().get(url);
        self.client.json(req).await
    }
}
