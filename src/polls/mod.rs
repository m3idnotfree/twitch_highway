mod builder;
mod response;
mod types;

pub use builder::{CreatePoll, GetPolls};
pub use response::PollsResponse;
pub use types::{EndPollStatus, Poll, PollStatus};

use std::future::Future;

use types::EndPollBody;

use crate::{
    types::{constants::POLLS, BroadcasterId, PollId, Title},
    Client, Error,
};

pub trait PollsAPI {
    /// See <https://dev.twitch.tv/docs/api/reference/#get-polls>
    fn get_polls<'a>(&'a self, broadcaster_id: &'a BroadcasterId) -> GetPolls<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#create-poll>
    fn create_poll<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        title: &'a str,
        choices: &'a [Title],
        duration: u16,
    ) -> CreatePoll<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#end-poll>
    fn end_poll(
        &self,
        broadcaster_id: &BroadcasterId,
        id: &PollId,
        status: EndPollStatus,
    ) -> impl Future<Output = Result<PollsResponse, Error>> + Send;
}

impl PollsAPI for Client {
    fn get_polls<'a>(&'a self, broadcaster_id: &'a BroadcasterId) -> GetPolls<'a> {
        GetPolls::new(self, broadcaster_id)
    }

    fn create_poll<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        title: &'a str,
        choices: &'a [Title],
        duration: u16,
    ) -> CreatePoll<'a> {
        CreatePoll::new(self, broadcaster_id, title, choices, duration)
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
