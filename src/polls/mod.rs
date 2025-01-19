use asknothingx2_util::api::Method;
use request::{EndPollRequest, PollsRequest};
use response::PollsResponse;
use serde_json::Value;
use types::PollStatus;

use crate::{
    base::TwitchAPIBase,
    request::RequestBody,
    types::{
        constants::{BROADCASTER_ID, ID},
        BroadcasterId, Id, PaginationQuery, Title,
    },
    EmptyBody, EndpointType, TwitchAPI, TwitchAPIRequest,
};

pub mod request;
pub mod response;
pub mod types;

pub trait PollsAPI: TwitchAPIBase {
    /// https://dev.twitch.tv/docs/api/reference/#get-polls
    fn get_polls(
        &self,
        broadcaster_id: BroadcasterId,
        id: Option<Id>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, PollsResponse>;
    /// https://dev.twitch.tv/docs/api/reference/#create-poll
    fn create_poll(
        &self,
        broadcaster_id: BroadcasterId,
        title: &str,
        choices: Vec<Title>,
        duration: u64,
        opts: Option<PollsRequest>,
    ) -> TwitchAPIRequest<RequestBody<Value, PollsRequest>, PollsResponse>;
    /// https://dev.twitch.tv/docs/api/reference/#end-poll
    fn end_poll(
        &self,
        broadcaster_id: BroadcasterId,
        id: Id,
        status: PollStatus,
    ) -> TwitchAPIRequest<EndPollRequest, PollsResponse>;
}

impl PollsAPI for TwitchAPI {
    fn get_polls(
        &self,
        broadcaster_id: BroadcasterId,
        id: Option<Id>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, PollsResponse> {
        let mut url = self.build_url();
        url.path(["polls"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query_opt(ID, id)
            .query_opt_pairs(pagination);

        TwitchAPIRequest::new(
            EndpointType::GetPolls,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn create_poll(
        &self,
        broadcaster_id: BroadcasterId,
        title: &str,
        choices: Vec<Title>,
        duration: u64,
        opts: Option<PollsRequest>,
    ) -> TwitchAPIRequest<RequestBody<Value, PollsRequest>, PollsResponse> {
        let mut url = self.build_url();
        url.path(["polls"]);
        let required = serde_json::json!({
            "broadcaster_id": broadcaster_id,
            "title": title,
            "choices": choices,
            "duration": duration,
        });

        let request_body = RequestBody::new(required, opts);

        let mut headers = self.build_headers();
        headers.json();

        TwitchAPIRequest::new(
            EndpointType::CreatePoll,
            url.build(),
            Method::POST,
            headers.build(),
            request_body,
        )
    }
    fn end_poll(
        &self,
        broadcaster_id: BroadcasterId,
        id: Id,
        status: PollStatus,
    ) -> TwitchAPIRequest<EndPollRequest, PollsResponse> {
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
