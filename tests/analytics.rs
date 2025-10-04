#![cfg(feature = "analytics")]

#[macro_use]
mod common;

use twitch_highway::{
    analytics::{AnalyticsAPI, AnalyticsRequest},
    types::{GameId, PaginationQuery},
};

api_test!(get_extension_analytics, [None, None, None]);
api_test!(
    get_game_analytics,
    [
        Some(&GameId::from("493057")),
        Some(
            AnalyticsRequest::new()
                .started_at(&"2018-01-01T00:00:00Z".parse().unwrap())
                .ended_at(&"2018-03-01T00:00:00Z".parse().unwrap())
        ),
        None
    ]
);

api_test!(extra
    get_game_analytics,
    get_game_analytics2,
    [
        None,
        None,
        Some(PaginationQuery::new().first(5))
    ]
);
