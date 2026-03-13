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
    /// See <https://dev.twitch.tv/docs/api/reference/#get-creator-goals>
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
