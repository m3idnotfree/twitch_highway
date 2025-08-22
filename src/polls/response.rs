use asknothingx2_util::serde::{deserialize_empty_object_as_none, serialize_none_as_empty_object};
use serde::{Deserialize, Serialize};

use crate::types::Pagination;

use super::types::Poll;

#[derive(Debug, Serialize, Deserialize)]
pub struct PollsResponse {
    pub data: Vec<Poll>,

    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::polls::response::PollsResponse;

    #[test]
    fn polls_response_deserialization() {
        let json_data = json!({
            "data": [
                {
                    "id": "poll123",
                    "broadcaster_id": "broadcaster456",
                    "broadcaster_name": "TestStreamer",
                    "broadcaster_login": "teststreamer",
                    "title": "What game should we play next?",
                    "choices": [
                        {
                            "id": "choice1",
                            "title": "Minecraft",
                            "votes": 250,
                            "channel_points_votes": 100,
                            "bits_votes": 50
                        },
                        {
                            "id": "choice2",
                            "title": "Fortnite",
                            "votes": 180,
                            "channel_points_votes": 80,
                            "bits_votes": 30
                        }
                    ],
                    "bits_voting_enabled": true,
                    "bits_per_vote": 10,
                    "channel_points_voting_enabled": true,
                    "channel_points_per_vote": 100,
                    "status": "ACTIVE",
                    "duration": 300,
                    "started_at": "2024-01-15T10:30:00Z",
                    "ended_at": null
                }
            ],
            "pagination": {
                "cursor": "eyJiI..."
            }
        });

        let response: PollsResponse = serde_json::from_value(json_data).unwrap();

        assert_eq!(response.data.len(), 1);
        assert!(response.pagination.is_some());

        let poll = &response.data[0];
        assert_eq!(poll.id.as_str(), "poll123");
        assert_eq!(poll.broadcaster_id.as_str(), "broadcaster456");
        assert_eq!(poll.title, "What game should we play next?");
        assert_eq!(poll.choices.len(), 2);
        assert!(poll.bits_voting_enabled);
        assert!(poll.channel_points_voting_enabled);
        assert_eq!(poll.duration, 300);
        assert!(poll.ended_at.is_none());

        let first_choice = &poll.choices[0];
        assert_eq!(first_choice.id.as_str(), "choice1");
        assert_eq!(first_choice.title, "Minecraft");
        assert_eq!(first_choice.votes, 250);

        let second_choice = &poll.choices[1];
        assert_eq!(second_choice.id.as_str(), "choice2");
        assert_eq!(second_choice.title, "Fortnite");
        assert_eq!(second_choice.votes, 180);
    }
}
