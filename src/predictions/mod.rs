use asknothingx2_util::api::Method;
use request::CreatePredictionRequest;
use response::PredictionsResponse;
use types::PredictionStatus;

use crate::{
    request::{EmptyBody, EndpointType, RequestBody, TwitchAPIRequest},
    types::{
        constants::{BROADCASTER_ID, ID},
        BroadcasterId, Id, PaginationQuery, Title,
    },
    TwitchAPI,
};

pub mod request;
pub mod response;
pub mod types;

#[cfg_attr(docsrs, doc(cfg(feature = "predictions")))]
pub trait PredictionsAPI {
    /// <https://dev.twitch.tv/docs/api/reference/#get-predictions>
    fn get_predictions(
        &self,
        broadcaster_id: BroadcasterId,
        id: Option<&[Id]>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<PredictionsResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#create-prediction>
    fn create_prediction(
        &self,
        broadcaster_id: BroadcasterId,
        title: &str,
        outcomes: &[Title],
        prediction_window: u64,
    ) -> TwitchAPIRequest<PredictionsResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#end-prediction>
    fn end_prediction(
        &self,
        broadcaster_id: BroadcasterId,
        id: Id,
        status: PredictionStatus,
        winning_outcome_id: Option<&str>, //request: EndPredictionRequest,
    ) -> TwitchAPIRequest<PredictionsResponse>;
}

impl PredictionsAPI for TwitchAPI {
    fn get_predictions(
        &self,
        broadcaster_id: BroadcasterId,
        ids: Option<&[Id]>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<PredictionsResponse> {
        let mut url = self.build_url();
        url.path(["predictions"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query_opt_extend(ids.map(|id| id.into_iter().map(|id| (ID, id))));
        if let Some(pagination) = pagination {
            pagination.apply_to_url(&mut url);
        }

        TwitchAPIRequest::new(
            EndpointType::GetPredictions,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            None,
        )
    }
    fn create_prediction(
        &self,
        broadcaster_id: BroadcasterId,
        title: &str,
        outcomes: &[Title],
        prediction_window: u64,
    ) -> TwitchAPIRequest<PredictionsResponse> {
        let mut url = self.build_url();
        url.path(["predictions"]);

        let mut headers = self.build_headers();
        headers.json();
        TwitchAPIRequest::new(
            EndpointType::CreatePrediction,
            url.build(),
            Method::POST,
            headers.build(),
            CreatePredictionRequest::new(broadcaster_id, title, outcomes, prediction_window)
                .to_json(),
        )
    }
    fn end_prediction(
        &self,
        broadcaster_id: BroadcasterId,
        id: Id,
        status: PredictionStatus,
        winning_outcome_id: Option<&str>,
    ) -> TwitchAPIRequest<PredictionsResponse> {
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
            request_body.to_json(),
        )
    }
}
