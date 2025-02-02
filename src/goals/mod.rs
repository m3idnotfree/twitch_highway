use asknothingx2_util::api::Method;
use response::GoalsResponse;

use crate::{
    base::TwitchAPIBase,
    request::{EmptyBody, EndpointType, TwitchAPIRequest},
    types::{constants::BROADCASTER_ID, BroadcasterId},
    TwitchAPI,
};

pub mod response;
pub mod types;

#[cfg_attr(docsrs, doc(cfg(feature = "goals")))]
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
