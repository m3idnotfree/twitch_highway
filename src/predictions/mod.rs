use asknothingx2_util::api::Method;
use request::{CreatePredictionRequest, EndPredictionRequest};

use crate::{
    base::TwitchAPIBase,
    types::{BroadcasterId, Id, AFTER, BROADCASTER_ID, FIRST, ID},
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
        first: Option<&str>,
        after: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody>;
    fn create_prediction(
        &self,
        request: CreatePredictionRequest,
    ) -> TwitchAPIRequest<CreatePredictionRequest>;
    fn end_prediction(
        &self,
        request: EndPredictionRequest,
    ) -> TwitchAPIRequest<EndPredictionRequest>;
}

impl PredictionsAPI for TwitchAPI {
    fn get_predictions(
        &self,
        broadcaster_id: BroadcasterId,
        ids: Option<Vec<Id>>,
        first: Option<&str>,
        after: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path(["predictions"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query_opt_extend(ids.map(|id| id.into_iter().map(|id| (ID, id))))
            .query_opt(FIRST, first)
            .query_opt(AFTER, after);

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
        request: CreatePredictionRequest,
    ) -> TwitchAPIRequest<CreatePredictionRequest> {
        let mut url = self.build_url();
        url.path(["predictions"]);

        let mut headers = self.build_headers();
        headers.json();
        TwitchAPIRequest::new(
            EndpointType::CreatePrediction,
            url.build(),
            Method::POST,
                headers.build(),
            request,
        )
    }
    fn end_prediction(
        &self,
        request: EndPredictionRequest,
    ) -> TwitchAPIRequest<EndPredictionRequest> {
        let mut url = self.build_url();
        url.path(["predictions"]);

        let mut headers = self.build_headers();
        headers.json();
        TwitchAPIRequest::new(
            EndpointType::EndPrediction,
            url.build(),
            Method::PATCH,
            headers.build(),
            request,
        )
    }
}
