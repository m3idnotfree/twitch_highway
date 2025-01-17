new_fn_mock_server_f!(
    name: get_top_games,
    oauth: {
        @user,
        module: GamesScopes,
        scopes: with_top_games_read
    },
    api: {
        modules: [
            twitch_highway::games::GamesAPI
        ],
        endpoint: get_top_games,
        args: |_a|{
            None
        }
    }
);

new_fn_mock_server_f!(
    name: get_games,
    oauth: {
        @user,
        module: GamesScopes,
        scopes: with_games_read
    },
    api: {
        modules: [
            twitch_highway::games::GamesAPI,
            twitch_highway::games::request::GetGamesRequest
        ],
        endpoint: get_games,
        args: |broadcaster_id|{
            GetGamesRequest::new().ids(vec![broadcaster_id.to_id()])
        }
    }
);
