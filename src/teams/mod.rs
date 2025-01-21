use asknothingx2_util::api::Method;
use request::TeamFilter;
use response::{ChannelTeamsResponse, TeamsResponse};

use crate::{
    base::TwitchAPIBase,
    types::{
        constants::{BROADCASTER_ID, TEAMS},
        BroadcasterId,
    },
    EmptyBody, EndpointType, TwitchAPI, TwitchAPIRequest,
};

pub mod request;
pub mod response;
pub mod types;

#[cfg_attr(docsrs, doc(cfg(feature = "teams")))]
pub trait TeamsAPI: TwitchAPIBase {
    /// <https://dev.twitch.tv/docs/api/reference/#get-channel-teams>
    fn get_channel_teams(
        &self,
        broadcaster_id: BroadcasterId,
    ) -> TwitchAPIRequest<EmptyBody, ChannelTeamsResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#get-teams>
    fn get_teams(&self, team_filter: TeamFilter) -> TwitchAPIRequest<EmptyBody, TeamsResponse>;
}

impl TeamsAPI for TwitchAPI {
    fn get_channel_teams(
        &self,
        broadcaster_id: BroadcasterId,
    ) -> TwitchAPIRequest<EmptyBody, ChannelTeamsResponse> {
        let mut url = self.build_url();
        url.path([TEAMS, "channel"])
            .query(BROADCASTER_ID, broadcaster_id);

        TwitchAPIRequest::new(
            EndpointType::GetChannelTeams,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn get_teams(&self, team_filter: TeamFilter) -> TwitchAPIRequest<EmptyBody, TeamsResponse> {
        let mut url = self.build_url();
        url.path([TEAMS]).query_pairs(team_filter);

        TwitchAPIRequest::new(
            EndpointType::GetChannelTeams,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
}
