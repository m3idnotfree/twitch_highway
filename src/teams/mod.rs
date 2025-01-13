use asknothingx2_util::api::Method;

use crate::{
    base::TwitchAPIBase,
    types::{BroadcasterId, Id, UserId, AFTER, BROADCASTER_ID, FIRST, ID, USER_ID},
    EmptyBody, EndpointType, TwitchAPI, TwitchAPIRequest,
};

pub mod response;
pub mod types;

pub trait TeamsAPI: TwitchAPIBase {
    fn get_channel_teams(&self, broadcaster_id: BroadcasterId) -> TwitchAPIRequest<EmptyBody>;
    fn get_teams(&self, name: Option<&str>, id: Option<Id>) -> TwitchAPIRequest<EmptyBody>;
}

impl TeamsAPI for TwitchAPI {
    fn get_channel_teams(&self, broadcaster_id: BroadcasterId) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path(["teams", "channel"])
            .query(BROADCASTER_ID, broadcaster_id);

        TwitchAPIRequest::new(
            EndpointType::GetChannelTeams,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn get_teams(&self, name: Option<&str>, id: Option<Id>) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path(["teams"])
            .query_opt("name", name)
            .query_opt(ID, id);

        TwitchAPIRequest::new(
            EndpointType::GetChannelTeams,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
}
