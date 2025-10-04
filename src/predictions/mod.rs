mod request;
mod response;
mod types;

pub use request::CreatePredictionRequest;
pub use response::PredictionsResponse;
pub use types::{Prediction, PredictionColor, PredictionOutComes, PredictionStatus, TopPredictor};

use crate::{
    request::{NoContent, RequestBody},
    types::{
        constants::{BROADCASTER_ID, ID},
        BroadcasterId, Id, PaginationQuery, Title,
    },
};

endpoints! {
    PredictionsAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#get-predictions>
        fn get_predictions(
            &self,
            broadcaster_id: &BroadcasterId,
            id: Option<&[Id]>,
            pagination: Option<PaginationQuery>,
        ) -> PredictionsResponse {
            endpoint_type: GetPredictions,
            method: GET,
            path: ["predictions"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                extend(id.unwrap_or(&[]).iter().map(|id| (ID, id))),
                pagination(pagination)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#create-prediction>
        fn create_prediction(
            &self,
            broadcaster_id: &BroadcasterId,
            title: &str,
            outcomes: &[Title],
            prediction_window: u64,
        ) -> PredictionsResponse {
            endpoint_type: CreatePrediction,
            method: POST,
            path: ["predictions"],
            headers: [json],
            body: CreatePredictionRequest::new(broadcaster_id, title, outcomes, prediction_window).into_json()
        }

        /// <https://dev.twitch.tv/docs/api/reference/#end-prediction>
        fn end_prediction(
            &self,
            broadcaster_id: &BroadcasterId,
            id: &Id,
            status: PredictionStatus,
            winning_outcome_id: Option<&str>,
        ) -> PredictionsResponse {
            endpoint_type: EndPrediction,
            method: PATCH,
            path: ["predictions"],
            headers: [json],
            body: {
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

                RequestBody::new(required, None::<NoContent>).into_json()
            }
        }
    }
}
