mod response;
mod types;

pub use response::StartRaidResponse;
pub use types::StartRaid;

use std::future::Future;

use crate::{
    Client, Error,
    types::{
        BroadcasterId,
        constants::{BROADCASTER_ID, FROM_BROADCASTER_ID, RAIDS, TO_BROADCASTER_ID},
    },
};

pub trait RaidAPI {
    /// See <https://dev.twitch.tv/docs/api/reference/#start-a-raid>
    fn start_raid(
        &self,
        from_broadcaster_id: &BroadcasterId,
        to_broadcaster_id: &BroadcasterId,
    ) -> impl Future<Output = Result<StartRaidResponse, Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#cancel-a-raid>
    fn cancel_raid(
        &self,
        broadcaster_id: &BroadcasterId,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

impl RaidAPI for Client {
    async fn start_raid(
        &self,
        from_broadcaster_id: &BroadcasterId,
        to_broadcaster_id: &BroadcasterId,
    ) -> Result<StartRaidResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().push(RAIDS);

        url.query_pairs_mut()
            .append_pair(FROM_BROADCASTER_ID, from_broadcaster_id)
            .append_pair(TO_BROADCASTER_ID, to_broadcaster_id);

        self.json(self.http_client().post(url)).await
    }

    async fn cancel_raid(&self, broadcaster_id: &BroadcasterId) -> Result<(), Error> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().push(RAIDS);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id);

        self.no_content(self.http_client().delete(url)).await
    }
}
