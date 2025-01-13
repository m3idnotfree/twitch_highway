use asknothingx2_util::api::Method;

use crate::{
    base::TwitchAPIBase,
    types::{BroadcasterId, AFTER, BROADCASTER_ID, FIRST},
    EmptyBody, EndpointType, TwitchAPI, TwitchAPIRequest,
};

pub mod response;
pub mod types;

pub trait HypeTrainAPI: TwitchAPIBase {
    fn get_hype_train_events(
        &self,
        broadcaster_id: BroadcasterId,
        first: Option<u64>,
        after: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody>;
}

impl HypeTrainAPI for TwitchAPI {
    fn get_hype_train_events(
        &self,
        broadcaster_id: BroadcasterId,
        first: Option<u64>,
        after: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path(["hypetrain", "events"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query_opt(FIRST, first.map(|x| x.to_string()))
            .query_opt(AFTER, after);

        TwitchAPIRequest::new(
            EndpointType::GetHypeTrainEvents,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
}
