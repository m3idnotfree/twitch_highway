use chrono::{DateTime, Utc};

use crate::{
    bits::{BitsLeaderboardResponse, ExtensionTransactionsResponse},
    types::{
        constants::{
            AFTER, BITS, COUNT, EXTENSIONS, EXTENSION_ID, FIRST, ID, LEADERBOARD, PERIOD,
            STARTED_AT, TRANSACTIONS, USER_ID,
        },
        ExtensionId, TransactionId, UserId,
    },
    Client, Error,
};

use crate::bits::Period;

#[derive(Debug)]
pub struct GetBitsLeaderboard<'a> {
    client: &'a Client,
    count: Option<u8>,
    period: Option<Period>,
    started_at: Option<&'a DateTime<Utc>>,
    user_id: Option<&'a UserId>,
}

impl<'a> GetBitsLeaderboard<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self {
            client,
            count: None,
            period: None,
            started_at: None,
            user_id: None,
        }
    }

    /// min 1, max 100, default 10
    pub fn count(mut self, value: u8) -> Self {
        self.count = Some(value);
        self
    }

    pub fn period(mut self, value: Period) -> Self {
        self.period = Some(value);
        self
    }

    pub fn started_at(mut self, value: &'a DateTime<Utc>) -> Self {
        self.started_at = Some(value);
        self
    }

    pub fn user_id(mut self, value: &'a UserId) -> Self {
        self.user_id = Some(value);
        self
    }

    pub async fn send(self) -> Result<BitsLeaderboardResponse, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut().unwrap().extend([BITS, LEADERBOARD]);

        if let Some(val) = self.count {
            url.query_pairs_mut().append_pair(COUNT, &val.to_string());
        }
        if let Some(val) = self.period {
            url.query_pairs_mut().append_pair(PERIOD, val.as_ref());
        }
        if let Some(val) = self.started_at {
            url.query_pairs_mut().append_pair(
                STARTED_AT,
                &val.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
            );
        }
        if let Some(val) = self.user_id {
            url.query_pairs_mut().append_pair(USER_ID, val);
        }

        let req = self.client.http_client().get(url);
        self.client.json(req).await
    }
}

#[derive(Debug)]
pub struct GetExtensionTransactions<'a> {
    client: &'a Client,
    extension_id: &'a ExtensionId,
    ids: Option<&'a [TransactionId]>,
    first: Option<u8>,
    after: Option<&'a str>,
}

impl<'a> GetExtensionTransactions<'a> {
    pub fn new(client: &'a Client, extension_id: &'a ExtensionId) -> Self {
        Self {
            client,
            extension_id,
            ids: None,
            first: None,
            after: None,
        }
    }

    pub fn ids(mut self, value: &'a [TransactionId]) -> Self {
        self.ids = Some(value);
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

    pub async fn send(self) -> Result<ExtensionTransactionsResponse, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([EXTENSIONS, TRANSACTIONS]);

        url.query_pairs_mut()
            .append_pair(EXTENSION_ID, self.extension_id.as_ref());
        if let Some(ids) = self.ids {
            url.query_pairs_mut()
                .extend_pairs(ids.iter().map(|id| (ID, id)));
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
