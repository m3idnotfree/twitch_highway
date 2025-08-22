use asknothingx2_util::api::Method;
use request::{EndPollRequest, PollsRequest};
use response::PollsResponse;
use types::PollStatus;

use crate::{
    request::{EndpointType, RequestBody, TwitchAPIRequest},
    types::{
        constants::{BROADCASTER_ID, ID},
        BroadcasterId, Id, PaginationQuery, Title,
    },
    TwitchAPI,
};

pub mod request;
pub mod response;
pub mod types;

endpoints! {
    PollsAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#get-polls>
        fn get_polls(
            &self,
            broadcaster_id: &BroadcasterId,
            id: Option<&Id>,
            pagination: Option<PaginationQuery>,
        ) -> PollsResponse {
            endpoint_type: EndpointType::GetPolls,
            method: Method::GET,
            path: ["polls"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                opt(ID, id),
                pagination(pagination)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#create-poll>
        fn create_poll(
            &self,
            broadcaster_id: &BroadcasterId,
            title: &str,
            choices: &[Title],
            duration: u64,
            opts: Option<PollsRequest>,
        ) -> PollsResponse {
            endpoint_type: EndpointType::CreatePoll,
            method: Method::POST,
            path: ["polls"],
            headers: [json],
            body: {
                let required = serde_json::json!({
                    "broadcaster_id": broadcaster_id,
                    "title": title,
                    "choices": choices,
                    "duration": duration,
                });

                RequestBody::new(required, opts).into_json()
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#end-poll>
        fn end_poll(
            &self,
            broadcaster_id: &BroadcasterId,
            id: &Id,
            status: PollStatus,
        ) -> PollsResponse {
            endpoint_type: EndpointType::EndPoll,
            method: Method::PATCH,
            path: ["polls"],
            headers: [json],
            body: EndPollRequest::new(broadcaster_id, id, status).into_json()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        polls::{request::PollsRequest, types::PollStatus, PollsAPI},
        types::{BroadcasterId, Id, Title},
    };

    api_test!(
        get_polls,
        [
            &BroadcasterId::new("141981764"),
            Some(&Id::from("ed961efd-8a3f-4cf5-a9d0-e616c590cd2a")),
            None
        ]
    );
    api_test!(
        create_poll,
        [
            &BroadcasterId::new("141981764"),
            "Heads or Tails?",
            &[Title::new("Heads"), Title::new("Tails")],
            1800,
            Some(
                PollsRequest::new()
                    .channel_points_voting_enabled(true)
                    .channel_points_per_vote(100)
            )
        ]
    );
    api_test!(
        end_poll,
        [
            &BroadcasterId::new("141981764"),
            &Id::new("ed961efd-8a3f-4cf5-a9d0-e616c590cd2a"),
            PollStatus::TERMINATED
        ]
    );
}
