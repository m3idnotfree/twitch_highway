use asknothingx2_util::api::Method;
use response::GoalsResponse;

use crate::{
    base::TwitchAPIBase,
    types::{constants::BROADCASTER_ID, BroadcasterId},
    EmptyBody, EndpointType, TwitchAPI, TwitchAPIRequest,
};

pub mod response;
pub mod types;

pub trait GoalsAPI: TwitchAPIBase {
    /// <https://dev.twitch.tv/docs/api/reference/#get-creator-goals>
    fn get_creator_goals(
        &self,
        broadcaster_id: BroadcasterId,
    ) -> TwitchAPIRequest<EmptyBody, GoalsResponse>;
}

impl GoalsAPI for TwitchAPI {
    fn get_creator_goals(
        &self,
        broadcaster_id: BroadcasterId,
    ) -> TwitchAPIRequest<EmptyBody, GoalsResponse> {
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
