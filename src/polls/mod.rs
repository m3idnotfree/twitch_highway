mod builder;
mod response;
mod types;

pub use builder::{CreatePollBuilder, GetPollsBuilder};
pub use response::PollsResponse;
pub use types::{EndPollStatus, Poll, PollStatus};

use std::future::Future;

use types::EndPollBody;

use crate::{
    types::{constants::POLLS, BroadcasterId, PollId, Title},
    Client, Error,
};

pub trait PollsAPI {
    /// Gets a list of polls that the broadcaster created
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster that created the polls.
    ///
    /// # Returns
    ///
    /// Returns a [`GetPollsBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     polls::PollsAPI,
    ///     types::BroadcasterId,
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_polls(&BroadcasterId::from("1234"))
    ///     .send()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `channel:read:polls or channel:manage:polls`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-polls>
    fn get_polls<'a>(&'a self, broadcaster_id: &'a BroadcasterId) -> GetPollsBuilder<'a>;

    /// Creates a poll that viewers in the broadcaster’s channel can vote on
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster that’s running the poll.
    /// * `title` - The question that viewers will vote on.
    /// * `choices` - A list of choices that viewers may choose from.
    /// * `duration` - The length of time (min 15, max 1800).
    ///
    /// # Returns
    ///
    /// Returns a [`CreatePollBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     polls::PollsAPI,
    ///     types::{BroadcasterId, Title}
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .create_poll(
    ///         &BroadcasterId::from("1234"),
    ///         "title",
    ///         &[Title::new("title_1"), Title::new("title_2")],
    ///         300
    ///     )
    ///     .send()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `channel:manage:polls`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#create-poll>
    fn create_poll<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        title: &'a str,
        choices: &'a [Title],
        duration: u16,
    ) -> CreatePollBuilder<'a>;

    /// Ends an active poll
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster that’s running the poll.
    /// * `id` - The ID of the poll to update.
    /// * `status` - [`EndPollStatus`]
    ///
    /// # Returns
    ///
    /// Returns a [`PollsResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     polls::{EndPollStatus, PollsAPI},
    ///     types::{BroadcasterId, PollId},
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .end_poll(
    ///         &BroadcasterId::from("1234"),
    ///         &PollId::from("5678"),
    ///         EndPollStatus::TERMINATED,
    ///     )
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `channel:manage:polls`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#end-poll>
    fn end_poll(
        &self,
        broadcaster_id: &BroadcasterId,
        id: &PollId,
        status: EndPollStatus,
    ) -> impl Future<Output = Result<PollsResponse, Error>> + Send;
}

impl PollsAPI for Client {
    fn get_polls<'a>(&'a self, broadcaster_id: &'a BroadcasterId) -> GetPollsBuilder<'a> {
        GetPollsBuilder::new(self, broadcaster_id)
    }

    fn create_poll<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        title: &'a str,
        choices: &'a [Title],
        duration: u16,
    ) -> CreatePollBuilder<'a> {
        CreatePollBuilder::new(self, broadcaster_id, title, choices, duration)
    }

    async fn end_poll(
        &self,
        broadcaster_id: &BroadcasterId,
        id: &PollId,
        status: EndPollStatus,
    ) -> Result<PollsResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().push(POLLS);

        let req = self.http_client().patch(url).json(&EndPollBody {
            broadcaster_id,
            id,
            status,
        });
        self.json(req).await
    }
}
