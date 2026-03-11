mod response;
mod types;

pub use response::{ChannelTeamsResponse, TeamsResponse};
pub use types::{BroadcasterTeam, Team, TeamUser};

use types::TeamSelect;

use std::future::Future;

use crate::{
    types::{
        constants::{BROADCASTER_ID, CHANNEL, TEAMS},
        BroadcasterId,
    },
    Client, Error,
};

pub trait TeamsAPI {
    /// Gets the list of Twitch teams that the broadcaster is a member of
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The list of teams that the broadcaster is a member of.
    ///
    /// # Returns
    ///
    /// Returns a [`ChannelTeamsResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     teams::TeamsAPI,
    ///     types::BroadcasterId,
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_channel_teams(&BroadcasterId::from("1234"))
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// No scope required
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-channel-teams>
    fn get_channel_teams(
        &self,
        broadcaster_id: &BroadcasterId,
    ) -> impl Future<Output = Result<ChannelTeamsResponse, Error>> + Send;

    /// Gets information about the specified Twitch team by name
    ///
    /// # Arguments
    ///
    /// * `select` - The filter to use. Pass `&str` (name) or [TeamId](crate::types::TeamId).
    ///
    /// # Returns
    ///
    /// Returns a [`TeamsResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     teams::TeamsAPI,
    ///     types::TeamId,
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// // By name
    /// let response = api.get_teams("name").await?;
    ///
    /// // By team ID
    /// let response = api.get_teams(&TeamId::from("1234")).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// No scope required
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-teams>
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
