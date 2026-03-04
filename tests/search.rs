#[macro_use]
mod common;

use twitch_highway::search::SearchAPI;

api_test!(search_categories | api | { api.search_categories("fort") });

api_test!(search_channels | api | { api.search_channels("twitchdev") });

api_test!(
    search_channels as search_channels_2 | api | {
        api.search_channels("a_seagull").live_only(true)
    }
);
