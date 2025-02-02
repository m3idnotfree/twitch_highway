use asknothingx2_util::api::Method;
use response::HypeTrainResponse;

use crate::{
    base::TwitchAPIBase,
    request::{EmptyBody, EndpointType, TwitchAPIRequest},
    types::{constants::BROADCASTER_ID, BroadcasterId, PaginationQuery},
    TwitchAPI,
};

pub mod response;
pub mod types;

#[cfg_attr(docsrs, doc(cfg(feature = "hype-train")))]
pub trait HypeTrainAPI: TwitchAPIBase {
    /// <https://dev.twitch.tv/docs/api/reference/#get-hype-train-events>
    fn get_hype_train_events(
        &self,
        broadcaster_id: BroadcasterId,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, HypeTrainResponse>;
}

impl HypeTrainAPI for TwitchAPI {
    fn get_hype_train_events(
        &self,
        broadcaster_id: BroadcasterId,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, HypeTrainResponse> {
        let mut url = self.build_url();
        url.path(["hypetrain", "events"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query_opt_pairs(pagination);

        TwitchAPIRequest::new(
            EndpointType::GetHypeTrainEvents,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
}
