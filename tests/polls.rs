#![cfg(feature = "polls")]

#[macro_use]
mod common;

use twitch_highway::{
    polls::{EndPollStatus, PollsAPI},
    types::{BroadcasterId, PollId, Title},
};

api_test!(
    get_polls | api | {
        api.get_polls(&BroadcasterId::from("141981764"))
            .ids(&[PollId::from("ed961efd-8a3f-4cf5-a9d0-e616c590cd2a")])
    }
);

api_test!(
    create_poll | api | {
        api.create_poll(
            &BroadcasterId::from("141981764"),
            "Heads or Tails?",
            &[Title::new("Heads"), Title::new("Tails")],
            1800,
        )
        .channel_points_voting_enabled(true)
        .channel_points_per_vote(100)
    }
);

api_test!(
    end_poll
    [
        &BroadcasterId::from("141981764"),
        &PollId::from("ed961efd-8a3f-4cf5-a9d0-e616c590cd2a"),
        EndPollStatus::TERMINATED
    ]
);
