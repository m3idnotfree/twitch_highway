mod response;
mod types;

pub use response::{ChannelTeamsResponse, TeamsResponse};
pub use types::{BroadcasterTeam, Team, TeamUser};

use crate::{
    request::TwitchAPIRequest,
    types::{
        constants::{BROADCASTER_ID, CHANNEL, ID, NAME, TEAMS},
        BroadcasterId, TeamId,
    },
    Client,
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
    /// # async fn example(api: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .get_channel_teams(&BroadcasterId::from("1234"))
    ///     .json()
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
    ) -> TwitchAPIRequest<ChannelTeamsResponse>;

    /// Gets information about the specified Twitch team by name
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the team to get.
    ///
    /// # Returns
    ///
    /// Returns a [`TeamsResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::teams::TeamsAPI;
    ///
    /// # async fn example(api: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .get_teams_by_name("name")
    ///     .json()
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
    /// <https://dev.twitch.tv/docs/api/reference/#get-teams>
    fn get_teams_by_name(&self, name: &str) -> TwitchAPIRequest<TeamsResponse>;

    /// Gets information about the specified Twitch team by id
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the team to get.
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
    /// # async fn example(api: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .get_teams_by_id(&TeamId::from("1234"))
    ///     .json()
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
    /// <https://dev.twitch.tv/docs/api/reference/#get-teams>
    fn get_teams_by_id(&self, id: &TeamId) -> TwitchAPIRequest<TeamsResponse>;
}

impl TeamsAPI for Client {
    simple_endpoint!(
        fn get_channel_teams(
            broadcaster_id: &BroadcasterId [key = BROADCASTER_ID],
        ) -> ChannelTeamsResponse;
            endpoint: GetChannelTeams,
            method: GET,
            path: [TEAMS, CHANNEL],
    );
    simple_endpoint!(
        fn get_teams_by_name(
            name: &str [key = NAME]
        ) -> TeamsResponse;
            endpoint: GetTeams,
            method: GET,
            path: [TEAMS],
    );
    simple_endpoint!(
        fn get_teams_by_id(
            id: &TeamId [key = ID]
        ) -> TeamsResponse;
            endpoint: GetTeams,
            method: GET,
            path: [TEAMS],
    );
}
