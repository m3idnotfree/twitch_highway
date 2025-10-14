mod response;
mod types;

pub use response::GoalsResponse;
pub use types::{Goal, GoalType};

use crate::{
    request::TwitchAPIRequest,
    types::{
        constants::{BROADCASTER_ID, GOALS},
        BroadcasterId,
    },
    TwitchAPI,
};

pub trait GoalsAPI {
    /// Gets the broadcaster’s list of active goals
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster that created the goals.
    ///
    /// # Returns
    ///
    /// Returns a [`GoalsResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::TwitchAPI;
    /// use twitch_highway::{
    ///     goals::GoalsAPI,
    ///     types::BroadcasterId
    /// };
    ///
    /// # async fn example(api: TwitchAPI) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .get_creator_goals(&BroadcasterId::from("1234"))
    ///     .json()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `channel:read:goals`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-creator-goals>
    fn get_creator_goals(&self, broadcaster_id: &BroadcasterId) -> TwitchAPIRequest<GoalsResponse>;
}

impl GoalsAPI for TwitchAPI {
    simple_endpoint!(
        fn get_creator_goals(
            broadcaster_id: &BroadcasterId [key = BROADCASTER_ID]
        ) -> GoalsResponse;
            endpoint: GetCreatorGoals,
            method: GET,
            path: [GOALS],
    );
}
