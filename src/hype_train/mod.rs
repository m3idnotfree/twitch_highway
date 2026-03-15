mod response;
mod types;

pub use response::{HypeTrainResponse, HypeTrainStatusResponse};
pub use types::{
    AllTimeHigh, Current, HypeTrain, HypeTrainContribution, HypeTrainEvent, HypeTrainStatus,
    HypeTrainType, SharedAllTimeHigh, SharedTrainParticipant,
};

use std::future::Future;

use crate::{
    Client, Error,
    types::{
        BroadcasterId,
        constants::{BROADCASTER_ID, HYPE_TRAIN, STATUS},
    },
};

pub trait HypeTrainAPI {
    // See <https://dev.twitch.tv/docs/api/reference/#get-hype-train-status>
    fn get_hype_train_status(
        &self,
        broadcaster_id: &BroadcasterId,
    ) -> impl Future<Output = Result<HypeTrainStatusResponse, Error>> + Send;
}

impl HypeTrainAPI for Client {
    async fn get_hype_train_status(
        &self,
        broadcaster_id: &BroadcasterId,
    ) -> Result<HypeTrainStatusResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([HYPE_TRAIN, STATUS]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id);

        self.json(self.http_client().get(url)).await
    }
}
