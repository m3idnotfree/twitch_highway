mod builder;
mod response;
mod types;

pub use builder::{EndPredictionBuilder, GetPredictionsBuilder};
pub use response::PredictionsResponse;
pub use types::{Prediction, PredictionStatus};

use crate::{
    request::TwitchAPIRequest,
    types::{constants::PREDICTIONS, BroadcasterId, PredictionId, Title},
    TwitchAPI,
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
    /// # use twitch_highway::TwitchAPI;
    /// use twitch_highway::{
    ///     predictions::PredictionsAPI,
    ///     types::BroadcasterId
    /// };
    ///
    /// # async fn example(api: TwitchAPI) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .get_predictions(&BroadcasterId::from("1234"))
    ///     .json()
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
    fn get_predictions<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
    ) -> GetPredictionsBuilder<'a>;

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
    /// # use twitch_highway::TwitchAPI;
    /// use twitch_highway::{
    ///     predictions::{PredictionsAPI},
    ///     types::{BroadcasterId,Title}
    /// };
    ///
    /// # async fn example(api: TwitchAPI) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .create_prediction(
    ///         &BroadcasterId::from("1234"),
    ///         "title",
    ///         &[Title::new("title-1")],
    ///         30
    ///     )
    ///     .json()
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
    ) -> TwitchAPIRequest<PredictionsResponse>;

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
    /// # use twitch_highway::TwitchAPI;
    /// use twitch_highway::{
    ///     predictions::{PredictionsAPI, PredictionStatus},
    ///     types::{BroadcasterId, PredictionId},
    /// };
    ///
    /// # async fn example(api: TwitchAPI) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .end_prediction(
    ///         &BroadcasterId::from("1234"),
    ///         &PredictionId::from("5678"),
    ///         PredictionStatus::ACTIVE,
    ///     )
    ///     .json()
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
    ) -> EndPredictionBuilder<'a>;
}

impl PredictionsAPI for TwitchAPI {
    fn get_predictions<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
    ) -> GetPredictionsBuilder<'a> {
        GetPredictionsBuilder::new(self, broadcaster_id)
    }
    simple_endpoint!(
    fn create_prediction(
        broadcaster_id: &BroadcasterId [skip],
        title: &str [skip],
        outcomes: &[Title] [skip],
        prediction_window: u64 [skip],
    ) -> PredictionsResponse;
        endpoint: CreatePrediction,
        method: POST,
        path: [PREDICTIONS],
        headers: [json],
        body: {
            Some(
                serde_json::json!({
                    "broadcaster_id":broadcaster_id,
                    "title":title,
                    "outcomes":outcomes,
                    "prediction_window":prediction_window
                }).to_string()
            )
        }
    );
    fn end_prediction<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        id: &'a PredictionId,
        status: PredictionStatus,
    ) -> EndPredictionBuilder<'a> {
        EndPredictionBuilder::new(self, broadcaster_id, id, status)
    }
}
