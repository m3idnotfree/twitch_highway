use twitch_highway::types::BroadcasterId;

fn_expected_request!(
    api: twitch_highway::goals::GoalsAPI,
    endpoint: get_creator_goals,
    token_type: User,
    scopes: Some(vec![Scope::ChannelReadGoals]),
    args: [BroadcasterId::new("141981764")],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/goals?broadcaster_id=141981764"
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"id\": \"1woowvbkiNv8BRxEWSqmQz6Zk92\",\n      \"broadcaster_id\": \"141981764\",\n      \"broadcaster_name\": \"TwitchDev\",\n      \"broadcaster_login\": \"twitchdev\",\n      \"type\": \"follower\",\n      \"description\": \"Follow goal for Helix testing\",\n      \"current_amount\": 27062,\n      \"target_amount\": 30000,\n      \"created_at\": \"2021-08-16T17:22:23Z\"\n    }\n  ]\n}",
    module: twitch_highway::goals::response::GoalsResponse,
    de: GoalsResponse
);

#[cfg(feature = "test")]
new_fn_mock_server_f!(
    name: mock_server_get_creator_goals,
    oauth: {
        @user,
        module: GoalsScopes,
        scopes: with_creator_goals_read
    },
    api: {
        modules: [
            twitch_highway::goals::GoalsAPI
        ],
        endpoint: get_creator_goals,
        args: |broadcaster_id|{
            broadcaster_id
        }
    }
);
