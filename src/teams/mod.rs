use asknothingx2_util::api::Method;
use request::TeamSelector;
use response::{ChannelTeamsResponse, TeamsResponse};

use crate::{
    request::{EmptyBody, EndpointType, TwitchAPIRequest},
    types::{
        constants::{BROADCASTER_ID, TEAMS},
        BroadcasterId,
    },
    TwitchAPI,
};

pub mod request;
pub mod response;
pub mod types;

#[cfg_attr(docsrs, doc(cfg(feature = "teams")))]
pub trait TeamsAPI {
    /// <https://dev.twitch.tv/docs/api/reference/#get-channel-teams>
    fn get_channel_teams(
        &self,
        broadcaster_id: BroadcasterId,
    ) -> TwitchAPIRequest<EmptyBody, ChannelTeamsResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#get-teams>
    fn get_teams(&self, team_filter: TeamSelector) -> TwitchAPIRequest<EmptyBody, TeamsResponse>;
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
    fn get_teams(&self, team_filter: TeamSelector) -> TwitchAPIRequest<EmptyBody, TeamsResponse> {
        let mut url = self.build_url();
        url.path([TEAMS]);
        team_filter.apply_to_url(&mut url);

        TwitchAPIRequest::new(
            EndpointType::GetChannelTeams,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
}
