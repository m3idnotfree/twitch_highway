mod builder;
mod response;
mod types;

pub use builder::{EndPrediction, GetPredictions};
pub use response::PredictionsResponse;
pub use types::{Prediction, PredictionStatus};

use std::future::Future;

use types::CreatePredictionBody;

use crate::{
    types::{constants::PREDICTIONS, BroadcasterId, PredictionId, Title},
    Client, Error,
};

pub trait PredictionsAPI {
    /// Gets a list of Channel Points Predictions that the broadcaster created
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster whose predictions you want to get.
    ///
    /// # Returns
    ///
    /// Returns a [`GetPredictionsBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     predictions::PredictionsAPI,
    ///     types::BroadcasterId
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_predictions(&BroadcasterId::from("1234"))
    ///     .send()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `channel:read:predictions or channel:manage:predictions`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-predictions>
    fn get_predictions<'a>(&'a self, broadcaster_id: &'a BroadcasterId) -> GetPredictions<'a>;

    /// Creates a Channel Points Prediction
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster that’s running the prediction.
    /// * `title` - The question that the broadcaster is asking.
    /// * `outcomes` - The list of possible outcomes that the viewers may choose from.
    ///   - [`Title`]
    /// * `prediction_window` - The length of time (in seconds) that the prediction will run for. (min 30, max 1800).
    ///
    /// # Returns
    ///
    /// Returns a [`PredictionsResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     predictions::{PredictionsAPI},
    ///     types::{BroadcasterId,Title}
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .create_prediction(
    ///         &BroadcasterId::from("1234"),
    ///         "title",
    ///         &[Title::new("title-1")],
    ///         30
    ///     )
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `channel:manage:predictions`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#create-prediction>
    fn create_prediction(
        &self,
        broadcaster_id: &BroadcasterId,
        title: &str,
        outcomes: &[Title],
        prediction_window: u64,
    ) -> impl Future<Output = Result<PredictionsResponse, Error>> + Send;

    /// Locks, resolves, or cancels a Channel Points Prediction
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster that’s running the prediction.
    /// * `id` - The ID of the prediction to update.
    /// * `status` - [`PredictionStatus`]
    ///
    /// # Returns
    ///
    /// Returns a [`EndPredictionBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     predictions::{PredictionsAPI, PredictionStatus},
    ///     types::{BroadcasterId, PredictionId},
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .end_prediction(
    ///         &BroadcasterId::from("1234"),
    ///         &PredictionId::from("5678"),
    ///         PredictionStatus::ACTIVE,
    ///     )
    ///     .send()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `channel:manage:predictions`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#end-prediction>
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
