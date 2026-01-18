#![cfg(feature = "hype-train")]

#[macro_use]
mod common;

use twitch_highway::{hype_train::HypeTrainAPI, types::BroadcasterId};

// api_test!(
//     get_hype_train_events | api | {
//         api.get_hype_train_events(&BroadcasterId::from("270954519"))
//             .first(1)
//     }
// );
api_test!(get_hype_train_status[&BroadcasterId::from("123")]);
