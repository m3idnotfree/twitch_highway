use asknothingx2_util::api::Method;
use request::{EndPollRequest, PollsRequest};
use types::PollStatus;

use crate::{
    base::TwitchAPIBase,
    types::{BroadcasterId, Id, AFTER, BROADCASTER_ID, FIRST, ID},
    EmptyBody, EndpointType, TwitchAPI, TwitchAPIRequest,
};

pub mod request;
pub mod response;
pub mod types;

pub trait PollsAPI: TwitchAPIBase {
    fn get_polls(
        &self,
        broadcaster_id: BroadcasterId,
        id: Option<Id>,
        first: Option<&str>,
        after: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody>;
    fn create_poll(&self, request: PollsRequest) -> TwitchAPIRequest<PollsRequest>;
    fn end_poll(
        &self,
        broadcaster_id: BroadcasterId,
        id: Id,
        status: PollStatus,
    ) -> TwitchAPIRequest<EndPollRequest>;
}

impl PollsAPI for TwitchAPI {
    fn get_polls(
        &self,
        broadcaster_id: BroadcasterId,
        id: Option<Id>,
        first: Option<&str>,
        after: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path(["polls"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query_opt(ID, id)
            .query_opt(FIRST, first)
            .query_opt(AFTER, after);

        TwitchAPIRequest::new(
            EndpointType::GetPolls,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn create_poll(&self, request: PollsRequest) -> TwitchAPIRequest<PollsRequest> {
        let mut url = self.build_url();
        url.path(["polls"]);

        let mut headers = self.build_headers();
        headers.json();

        TwitchAPIRequest::new(
            EndpointType::CreatePoll,
            url.build(),
            Method::POST,
            headers.build(),
            request,
        )
    }
    fn end_poll(
        &self,
        broadcaster_id: BroadcasterId,
        id: Id,
        status: PollStatus,
    ) -> TwitchAPIRequest<EndPollRequest> {
        let mut url = self.build_url();
        url.path(["polls"]);

        let mut headers = self.build_headers();
        headers.json();

        TwitchAPIRequest::new(
            EndpointType::EndPoll,
            url.build(),
            Method::PATCH,
            headers.build(),
            EndPollRequest::new(broadcaster_id, id, status),
        )
    }
}
