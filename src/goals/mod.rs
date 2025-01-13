use asknothingx2_util::api::Method;

use crate::{
    base::TwitchAPIBase,
    types::{BroadcasterId, BROADCASTER_ID},
    EmptyBody, EndpointType, TwitchAPI, TwitchAPIRequest,
};

pub mod response;
pub mod types;

pub trait GoalsAPI: TwitchAPIBase {
    fn get_creator_goals(&self, broadcaster_id: BroadcasterId) -> TwitchAPIRequest<EmptyBody>;
}

impl GoalsAPI for TwitchAPI {
    fn get_creator_goals(&self, broadcaster_id: BroadcasterId) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path(["goals"]).query(BROADCASTER_ID, broadcaster_id);

        TwitchAPIRequest::new(
            EndpointType::GetCreatorGoals,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
}
