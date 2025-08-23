pub mod request;
pub mod response;
pub mod types;

use asknothingx2_util::api::Method;
use request::TeamSelector;
use response::{ChannelTeamsResponse, TeamsResponse};

use crate::{
    request::EndpointType,
    types::{constants::BROADCASTER_ID, BroadcasterId},
};

const TEAMS: &str = "teams";

endpoints! {
    TeamsAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#get-channel-teams>
        fn get_channel_teams(
            &self,
            broadcaster_id: &BroadcasterId,
        ) -> ChannelTeamsResponse {
            endpoint_type: EndpointType::GetChannelTeams,
            method: Method::GET,
            path: [TEAMS, "channel"],
            query_params: {
                    query(BROADCASTER_ID, broadcaster_id)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#get-teams>
        fn get_teams(&self, team_selector: TeamSelector) -> TeamsResponse {
            endpoint_type: EndpointType::GetTeams,
            method: Method::GET,
            path: [TEAMS],
            query_params: {
                    into_query(team_selector)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        teams::{request::TeamSelector, TeamsAPI},
        types::{BroadcasterId, Id},
    };

    api_test!(get_channel_teams, [&BroadcasterId::from("96909659")]);
    api_test!(get_teams, [TeamSelector::by_id(&Id::from("6358"))]);
}
