use asknothingx2_util::api::Method;
use request::CreatePredictionRequest;
use response::PredictionsResponse;
use serde_json::Value;
use types::PredictionStatus;

use crate::{
    base::TwitchAPIBase,
    request::RequestBody,
    types::{
        constants::{BROADCASTER_ID, ID},
        BroadcasterId, Id, PaginationQuery, Title,
    },
    EmptyBody, EndpointType, TwitchAPI, TwitchAPIRequest,
};

pub mod request;
pub mod response;
pub mod types;

pub trait PredictionsAPI: TwitchAPIBase {
    fn get_predictions(
        &self,
        broadcaster_id: BroadcasterId,
        id: Option<Vec<Id>>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, PredictionsResponse>;
    fn create_prediction(
        &self,
        broadcaster_id: BroadcasterId,
        title: &str,
        outcomes: Vec<Title>,
        prediction_window: u64,
    ) -> TwitchAPIRequest<CreatePredictionRequest, PredictionsResponse>;
    fn end_prediction(
        &self,
        broadcaster_id: BroadcasterId,
        id: Id,
        status: PredictionStatus,
        winning_outcome_id: Option<&str>, //request: EndPredictionRequest,
    ) -> TwitchAPIRequest<RequestBody<Value, EmptyBody>, PredictionsResponse>;
}

impl PredictionsAPI for TwitchAPI {
    fn get_predictions(
        &self,
        broadcaster_id: BroadcasterId,
        ids: Option<Vec<Id>>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, PredictionsResponse> {
        let mut url = self.build_url();
        url.path(["predictions"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query_opt_extend(ids.map(|id| id.into_iter().map(|id| (ID, id))))
            .query_opt_pairs(pagination);

        TwitchAPIRequest::new(
            EndpointType::GetPredictions,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn create_prediction(
        &self,
        broadcaster_id: BroadcasterId,
        title: &str,
        outcomes: Vec<Title>,
        prediction_window: u64,
    ) -> TwitchAPIRequest<CreatePredictionRequest, PredictionsResponse> {
        let mut url = self.build_url();
        url.path(["predictions"]);

        let mut headers = self.build_headers();
        headers.json();
        TwitchAPIRequest::new(
            EndpointType::CreatePrediction,
            url.build(),
            Method::POST,
            headers.build(),
            CreatePredictionRequest::new(
                broadcaster_id,
                title.to_string(),
                outcomes,
                prediction_window,
            ),
        )
    }
    fn end_prediction(
        &self,
        broadcaster_id: BroadcasterId,
        id: Id,
        status: PredictionStatus,
        winning_outcome_id: Option<&str>,
    ) -> TwitchAPIRequest<RequestBody<Value, EmptyBody>, PredictionsResponse> {
        let mut url = self.build_url();
        url.path(["predictions"]);

        let required = if winning_outcome_id.is_some() {
            serde_json::json!({
                "broadcaster_id": broadcaster_id,
                "id": id,
                "status":status,
                "winning_outcome_id": winning_outcome_id.unwrap()
            })
        } else {
            serde_json::json!({
                "broadcaster_id": broadcaster_id,
                "id": id,
                "status":status
            })
        };

        let request_body = RequestBody::new(required, None::<EmptyBody>);

        let mut headers = self.build_headers();
        headers.json();
        TwitchAPIRequest::new(
            EndpointType::EndPrediction,
            url.build(),
            Method::PATCH,
            headers.build(),
            request_body,
        )
    }
}
