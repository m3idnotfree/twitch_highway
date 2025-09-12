mod request;
mod response;
mod types;

pub use request::TeamSelector;
pub use response::{ChannelTeamsResponse, TeamsResponse};
pub use types::{BroadcasterTeam, Team, TeamUser};

use crate::types::{constants::BROADCASTER_ID, BroadcasterId};

const TEAMS: &str = "teams";

endpoints! {
    TeamsAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#get-channel-teams>
        fn get_channel_teams(
            &self,
            broadcaster_id: &BroadcasterId,
        ) -> ChannelTeamsResponse {
            endpoint_type: GetChannelTeams,
            method: GET,
            path: [TEAMS, "channel"],
            query_params: {
                    query(BROADCASTER_ID, broadcaster_id)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#get-teams>
        fn get_teams(&self, team_selector: TeamSelector) -> TeamsResponse {
            endpoint_type: GetTeams,
            method: GET,
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
        teams::{TeamSelector, TeamsAPI},
        types::{BroadcasterId, Id},
    };

    api_test!(get_channel_teams, [&BroadcasterId::from("96909659")]);
    api_test!(get_teams, [TeamSelector::by_id(&Id::from("6358"))]);
}
