mod response;
mod types;

pub use response::{ChannelTeamsResponse, TeamsResponse};
pub use types::{BroadcasterTeam, Team, TeamUser};

use types::TeamSelect;

use std::future::Future;

use crate::{
    Client, Error,
    types::{
        BroadcasterId,
        constants::{BROADCASTER_ID, CHANNEL, TEAMS},
    },
};

pub trait TeamsAPI {
    /// See <https://dev.twitch.tv/docs/api/reference/#get-channel-teams>
    fn get_channel_teams(
        &self,
        broadcaster_id: &BroadcasterId,
    ) -> impl Future<Output = Result<ChannelTeamsResponse, Error>> + Send;

    /// - `&str` (team name) or `&TeamId`
    ///
    /// See <https://dev.twitch.tv/docs/api/reference/#get-teams>
    fn get_teams<'a>(
        &'a self,
        select: impl Into<TeamSelect<'a>> + Send,
    ) -> impl Future<Output = Result<TeamsResponse, Error>> + Send;
}

impl TeamsAPI for Client {
    async fn get_channel_teams(
        &self,
        broadcaster_id: &BroadcasterId,
    ) -> Result<ChannelTeamsResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().extend([TEAMS, CHANNEL]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id);

        self.json(self.http_client().get(url)).await
    }

    async fn get_teams<'a>(
        &'a self,
        select: impl Into<TeamSelect<'a>> + Send,
    ) -> Result<TeamsResponse, Error> {
        let select = select.into();
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().push(TEAMS);

        select.append_to_query(&mut url);

        self.json(self.http_client().get(url)).await
    }
}
