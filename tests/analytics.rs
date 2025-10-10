#![cfg(feature = "analytics")]

#[macro_use]
mod common;

use twitch_highway::{analytics::AnalyticsAPI, types::GameId};

api_test!(
    build
    get_extension_analytics |api| {
        api.get_extension_analytics()
    }
);
api_test!(
    build
    get_game_analytics |api| {
        api.get_game_analytics()
            .game_id(&GameId::from("493057"))
            .started_at(&"2018-01-01T00:00:00Z".parse().unwrap())
            .ended_at(&"2018-03-01T00:00:00Z".parse().unwrap())
    }
);
// api_test!(extra
//     get_game_analytics,
//     get_game_analytics2,
//     [
//         None,
//         None,
//         Some(PaginationQuery::new().first(5))
//     ]
// );
