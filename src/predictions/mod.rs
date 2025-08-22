use asknothingx2_util::api::Method;
use request::CreatePredictionRequest;
use response::PredictionsResponse;
use types::PredictionStatus;

use crate::{
    request::{EndpointType, NoContent, RequestBody, TwitchAPIRequest},
    types::{
        constants::{BROADCASTER_ID, ID},
        BroadcasterId, Id, PaginationQuery, Title,
    },
    TwitchAPI,
};

pub mod request;
pub mod response;
pub mod types;

endpoints! {
    PredictionsAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#get-predictions>
        fn get_predictions(
            &self,
            broadcaster_id: &BroadcasterId,
            id: Option<&[Id]>,
            pagination: Option<PaginationQuery>,
        ) -> PredictionsResponse {
            endpoint_type: EndpointType::GetPredictions,
            method: Method::GET,
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
            endpoint_type: EndpointType::CreatePrediction,
            method: Method::POST,
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
            endpoint_type: EndpointType::EndPrediction,
            method: Method::PATCH,
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

#[cfg(test)]
mod tests {
    use crate::{
        predictions::{types::PredictionStatus, PredictionsAPI},
        types::{BroadcasterId, Id, Title},
    };

    api_test!(
        get_predictions,
        [
            &BroadcasterId::new("55696719"),
            Some(&[Id::from("d6676d5c-c86e-44d2-bfc4-100fb48f0656")]),
            None
        ]
    );
    api_test!(
        create_prediction,
        [
            &BroadcasterId::new("141981764"),
            "Any leeks in the stream?",
            &[
                Title::new("Yes, give it time."),
                Title::new("Yes, give it time.")
            ],
            120
        ]
    );
    api_test!(
        end_prediction,
        [
            &BroadcasterId::new("141981764"),
            &Id::new("bc637af0-7766-4525-9308-4112f4cbf178"),
            PredictionStatus::RESOLVED,
            Some("73085848-a94d-4040-9d21-2cb7a89374b7")
        ]
    );
}
