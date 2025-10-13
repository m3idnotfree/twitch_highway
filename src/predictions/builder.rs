use serde::Serialize;

use crate::{
    predictions::{PredictionStatus, PredictionsResponse},
    request::TwitchAPIRequest,
    types::{
        constants::{AFTER, BROADCASTER_ID, FIRST, ID, PREDICTIONS},
        BroadcasterId, PredictionId,
    },
    TwitchAPI,
};

define_request_builder! {
    #[derive(Debug)]
    GetPredictionsBuilder<'a> {
        req: {broadcaster_id: &'a BroadcasterId [key = BROADCASTER_ID]},
        opts: {
            ids: &'a [PredictionId] [key = ID, convert = extend],
            first: u8 [key = FIRST, convert = to_string],
            after: &'a str [key = AFTER]
        }
    } -> PredictionsResponse;
    endpoint_type: GetPredictions,
    method: GET,
    path: [PREDICTIONS],
}

#[derive(Debug, Serialize)]
pub struct EndPredictionBuilder<'a> {
    #[serde(skip)]
    api: &'a TwitchAPI,
    broadcaster_id: &'a BroadcasterId,
    id: &'a PredictionId,
    status: PredictionStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    winning_outcome_id: Option<&'a str>,
}

impl<'a> EndPredictionBuilder<'a> {
    pub fn new(
        api: &'a TwitchAPI,
        broadcaster_id: &'a BroadcasterId,
        id: &'a PredictionId,
        status: PredictionStatus,
    ) -> Self {
        Self {
            api,
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

    pub fn build(self) -> TwitchAPIRequest<PredictionsResponse> {
        let mut url = self.api.build_url();

        url.path_segments_mut().unwrap().extend(&[PREDICTIONS]);

        let body = serde_json::to_string(&self).ok();

        TwitchAPIRequest::new(
            crate::request::EndpointType::EndPrediction,
            url,
            reqwest::Method::PATCH,
            self.api.header_json(),
            body,
            self.api.client.clone(),
        )
    }

    pub async fn send(self) -> Result<reqwest::Response, crate::Error> {
        self.build().send().await
    }

    pub async fn json(self) -> Result<PredictionsResponse, crate::Error> {
        self.build().json().await
    }
}
