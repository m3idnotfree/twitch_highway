use twitch_highway::types::UserId;

fn_expected_request!(
    api: twitch_highway::streams::StreamsAPI,
    endpoint: get_followed_streams,
    token_type: User,
    scopes: Some(vec![Scope::UserReadFollows]),
    args: [UserId::new("141981764"), None, None],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/streams/followed?user_id=141981764",
    json: None,
    text: None,
    urlencoded: None
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"id\": \"42170724654\",\n      \"user_id\": \"132954738\",\n      \"user_login\": \"aws\",\n      \"user_name\": \"AWS\",\n      \"game_id\": \"417752\",\n      \"game_name\": \"Talk Shows & Podcasts\",\n      \"type\": \"live\",\n      \"title\": \"AWS Howdy Partner! Y'all welcome ExtraHop to the show!\",\n      \"viewer_count\": 20,\n      \"started_at\": \"2021-03-31T20:57:26Z\",\n      \"language\": \"en\",\n      \"thumbnail_url\": \"https://static-cdn.jtvnw.net/previews-ttv/live_user_aws-{width}x{height}.jpg\",\n      \"tag_ids\": [],\n      \"tags\": [\"English\"]\n    }\n  ],\n  \"pagination\": {\n    \"cursor\": \"eyJiIjp7IkN1cnNvciI6ImV5SnpJam8zT0RNMk5TNDBORFF4TlRjMU1UY3hOU3dpWkNJNlptRnNjMlVzSW5RaU9uUnlkV1Y5In0sImEiOnsiQ3Vyc29yIjoiZXlKeklqb3hOVGs0TkM0MU56RXhNekExTVRZNU1ESXNJbVFpT21aaGJITmxMQ0owSWpwMGNuVmxmUT09In19\"\n  }\n}",
    module: twitch_highway::streams::response::StreamsResponse,
    de: StreamsResponse
);
