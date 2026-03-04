#[macro_use]
mod common;

use twitch_highway::{analytics::AnalyticsAPI, types::GameId};

api_test!(get_extension_analytics | api | { api.get_extension_analytics() });

api_test!(
    get_game_analytics | api | {
        api.get_game_analytics()
            .game_id(&GameId::from("493057"))
            .started_at(&"2018-01-01T00:00:00Z".parse().unwrap())
            .ended_at(&"2018-03-01T00:00:00Z".parse().unwrap())
    }
);

api_test!(get_game_analytics2 | api | { api.get_game_analytics().first(5) });
