fn_expected_request!(
    modules: [
        twitch_highway::channels::ChannelsAPI,
        twitch_highway::channels::request::ModifyChannelRequest,
        twitch_highway::types::BroadcasterId,
        twitch_highway::types::GameId,
        twitch_oauth_token::types::Scope
    ],
    endpoint: modify_channel_info,
    token_type: User,
    scopes: Some(vec![Scope::ChannelManageBroadcast]),
    args: [
        BroadcasterId::new("41245072"),
        Some(ModifyChannelRequest::new()
            .game_id(GameId::new("33214"))
            .title("there are helicopters in the game? REASON TO PLAY FORTNITE found")
            .broadcaster_language("en")
            .tags(&["LevelingUp"])
        )
    ],
    json_contain: [
        "\"game_id\":\"33214\"",
        "\"title\":\"there are helicopters in the game? REASON TO PLAY FORTNITE found\"",
        "\"broadcaster_language\":\"en\"","\"tags\":[\"LevelingUp\"]"
    ],
    method: PATCH,
    header: expected_headers!(json),
    url: "https://api.twitch.tv/helix/channels?broadcaster_id=41245072"
);
