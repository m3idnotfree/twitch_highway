#[macro_use]
mod common;

use twitch_highway::{
    bits::{BitsAPI, Period},
    types::{BroadcasterId, ExtensionId},
};

api_test!(
    get_bits_leaderboard | api | {
        api.get_bits_leaderboard()
            .count(2)
            .period(Period::Week)
            .started_at(&"2018-02-05T08:00:00Z".parse().unwrap())
    }
);

api_test!(
    get_cheermotes[Some(&BroadcasterId::from("41245072"))],
    get_cheermotes as get_cheermotes2[Some(&BroadcasterId::from("41245072"))]

);

api_test!(
    get_extension_transactions | api | {
        api.get_extension_transactions(&ExtensionId::from("1234"))
    }
);
