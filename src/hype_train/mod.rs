use asknothingx2_util::api::Method;
use response::HypeTrainResponse;

use crate::{
    base::TwitchAPIBase,
    types::{constants::BROADCASTER_ID, BroadcasterId, PaginationQuery},
    EmptyBody, EndpointType, TwitchAPI, TwitchAPIRequest,
};

pub mod response;
pub mod types;

pub trait HypeTrainAPI: TwitchAPIBase {
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
