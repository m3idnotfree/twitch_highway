mod builder;
mod response;
mod types;

pub use builder::{EndPrediction, GetPredictions};
pub use response::PredictionsResponse;
pub use types::{Prediction, PredictionStatus};

use std::future::Future;

use types::CreatePredictionBody;

use crate::{
    Client, Error,
    types::{BroadcasterId, PredictionId, Title, constants::PREDICTIONS},
};

pub trait PredictionsAPI {
    /// See <https://dev.twitch.tv/docs/api/reference/#get-predictions>
    fn get_predictions<'a>(&'a self, broadcaster_id: &'a BroadcasterId) -> GetPredictions<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#create-prediction>
    fn create_prediction(
        &self,
        broadcaster_id: &BroadcasterId,
        title: &str,
        outcomes: &[Title],
        prediction_window: u64,
    ) -> impl Future<Output = Result<PredictionsResponse, Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#end-prediction>
    fn end_prediction<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        id: &'a PredictionId,
        status: PredictionStatus,
    ) -> EndPrediction<'a>;
}

impl PredictionsAPI for Client {
    fn get_predictions<'a>(&'a self, broadcaster_id: &'a BroadcasterId) -> GetPredictions<'a> {
        GetPredictions::new(self, broadcaster_id)
    }

    async fn create_prediction(
        &self,
        broadcaster_id: &BroadcasterId,
        title: &str,
        outcomes: &[Title],
        prediction_window: u64,
    ) -> Result<PredictionsResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().push(PREDICTIONS);

        let req = self.http_client().post(url).json(&CreatePredictionBody {
            broadcaster_id,
            title,
            outcomes,
            prediction_window,
        });
        self.json(req).await
    }

    fn end_prediction<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        id: &'a PredictionId,
        status: PredictionStatus,
    ) -> EndPrediction<'a> {
        EndPrediction::new(self, broadcaster_id, id, status)
    }
}
