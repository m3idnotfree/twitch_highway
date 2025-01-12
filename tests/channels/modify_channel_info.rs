use twitch_highway::{channels::request::ModifyChannelRequest, types::BroadcasterId};

fn_expected_request!(
    api: twitch_highway::channels::ChannelsAPI,
    endpoint: modify_channel_info,
    token_type: User,
    scopes: Some(vec![Scope::ChannelManageBroadcast]),
    args: [
        BroadcasterId::new("41245072"),
        ModifyChannelRequest::new()
            .game_id("33214".to_string())
            .title("there are helicopters in the game? REASON TO PLAY FORTNITE found".to_string())
            .broadcaster_language("en".to_string())
            .tags(vec!["LevelingUp".to_string()])
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
