use serde::Serialize;

use crate::{
    predictions::{PredictionStatus, PredictionsResponse},
    types::{
        constants::{AFTER, BROADCASTER_ID, FIRST, ID, PREDICTIONS},
        BroadcasterId, PredictionId,
    },
    Client, Error,
};

#[derive(Debug)]
pub struct GetPredictions<'a> {
    client: &'a Client,
    broadcaster_id: &'a BroadcasterId,
    ids: Option<&'a [PredictionId]>,
    first: Option<u8>,
    after: Option<&'a str>,
}

impl<'a> GetPredictions<'a> {
    pub fn new(client: &'a Client, broadcaster_id: &'a BroadcasterId) -> Self {
        Self {
            client,
            broadcaster_id,
            ids: None,
            first: None,
            after: None,
        }
    }

    pub fn ids(mut self, value: &'a [PredictionId]) -> Self {
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

    pub async fn send(self) -> Result<PredictionsResponse, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut().unwrap().push(PREDICTIONS);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, self.broadcaster_id);
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

#[derive(Debug, Serialize)]
pub struct EndPrediction<'a> {
    #[serde(skip)]
    client: &'a Client,
    broadcaster_id: &'a BroadcasterId,
    id: &'a PredictionId,
    status: PredictionStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    winning_outcome_id: Option<&'a str>,
}

impl<'a> EndPrediction<'a> {
    pub fn new(
        client: &'a Client,
        broadcaster_id: &'a BroadcasterId,
        id: &'a PredictionId,
        status: PredictionStatus,
    ) -> Self {
        Self {
            client,
            broadcaster_id,
            id,
            status,
            winning_outcome_id: None,
        }
    }

    pub fn winning_outcome_id(mut self, value: &'a str) -> Self {
        self.winning_outcome_id = Some(value);
        self
    }

    pub async fn send(self) -> Result<PredictionsResponse, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut().unwrap().push(PREDICTIONS);

        let req = self.client.http_client().patch(url).json(&self);
        self.client.json(req).await
    }
}
