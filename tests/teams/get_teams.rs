fn_expected_request!(
    modules: [
        twitch_highway::teams::TeamsAPI,
        twitch_highway::teams::request::TeamFilter,
        twitch_highway::types::Id
    ],
    endpoint: get_teams,
    token_type: Any,
    scopes: None,
    args: [TeamFilter::by_id(Id::new("6358"))],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/teams?id=6358",
    json: None,
    text: None,
    urlencoded: None
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"users\": [\n        {\n          \"user_id\": \"278217731\",\n          \"user_name\": \"mastermndio\",\n          \"user_login\": \"mastermndio\"\n        },\n        {\n          \"user_id\": \"41284990\",\n          \"user_name\": \"jenninexus\",\n          \"user_login\": \"jenninexus\"\n        }\n      ],\n      \"background_image_url\": null,\n      \"banner\": null,\n      \"created_at\": \"2019-02-11T12:09:22Z\",\n      \"updated_at\": \"2020-11-18T15:56:41Z\",\n      \"info\": \"<p>An outgoing and enthusiastic group of friendly channels that write code, teach about technology, and promote the technical community.</p>\",\n      \"thumbnail_url\": \"https://static-cdn.jtvnw.net/jtv_user_pictures/team-livecoders-team_logo_image-bf1d9a87ca81432687de60e24ad9593d-600x600.png\",\n      \"team_name\": \"livecoders\",\n      \"team_display_name\": \"Live Coders\",\n      \"id\": \"6358\"\n    }\n  ]\n}",
    module: twitch_highway::teams::response::TeamsResponse,
    de: TeamsResponse
);
