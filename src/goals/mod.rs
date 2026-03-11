mod response;
mod types;

pub use response::GoalsResponse;
pub use types::{Goal, GoalType};

use std::future::Future;

use crate::{
    types::{
        constants::{BROADCASTER_ID, GOALS},
        BroadcasterId,
    },
    Client, Error,
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
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     goals::GoalsAPI,
    ///     types::BroadcasterId
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_creator_goals(&BroadcasterId::from("1234"))
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
    fn get_creator_goals(
        &self,
        broadcaster_id: &BroadcasterId,
    ) -> impl Future<Output = Result<GoalsResponse, Error>> + Send;
}

impl GoalsAPI for Client {
    async fn get_creator_goals(
        &self,
        broadcaster_id: &BroadcasterId,
    ) -> Result<GoalsResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().push(GOALS);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id);

        self.json(self.http_client().get(url)).await
    }
}
