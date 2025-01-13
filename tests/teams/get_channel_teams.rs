use twitch_highway::types::BroadcasterId;

fn_expected_request!(
    api: twitch_highway::teams::TeamsAPI,
    endpoint: get_channel_teams,
    token_type: Any,
    scopes: None,
    args: [BroadcasterId::new("96909659")],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/teams/channel?broadcaster_id=96909659",
    json: None,
    text: None,
    urlencoded: None
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"broadcaster_id\": \"96909659\",\n      \"broadcaster_name\": \"CSharpFritz\",\n      \"broadcaster_login\": \"csharpfritz\",\n      \"background_image_url\": null,\n      \"banner\": null,\n      \"created_at\": \"2019-02-11T12:09:22Z\",\n      \"updated_at\": \"2020-11-18T15:56:41Z\",\n      \"info\": \"<p>An outgoing and enthusiastic group of friendly channels that write code, teach about technology, and promote the technical community.</p>\",\n      \"thumbnail_url\": \"https://static-cdn.jtvnw.net/jtv_user_pictures/team-livecoders-team_logo_image-bf1d9a87ca81432687de60e24ad9593d-600x600.png\",\n      \"team_name\": \"livecoders\",\n      \"team_display_name\": \"Live Coders\",\n      \"id\": \"6358\"\n    }\n  ]\n}",
    module: twitch_highway::teams::response::ChannelTeamsResponse,
    de: ChannelTeamsResponse
);
