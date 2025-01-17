fn_expected_request!(
    modules: [
        twitch_highway::streams::StreamsAPI,
        twitch_highway::streams::request::GetStreamsRequest
    ],
    endpoint: get_streams,
    token_type: Any,
    scopes: None,
    args: [
        Some(GetStreamsRequest::new()
            .user_login(vec!["afro".to_string(), "cohhcarnage".to_string(), "lana_lux".to_string()])),
        None
    ],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/streams?user_login=afro&user_login=cohhcarnage&user_login=lana_lux",
    json: None,
    text: None,
    urlencoded: None
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"id\": \"123456789\",\n      \"user_id\": \"98765\",\n      \"user_login\": \"sandysanderman\",\n      \"user_name\": \"SandySanderman\",\n      \"game_id\": \"494131\",\n      \"game_name\": \"Little Nightmares\",\n      \"type\": \"live\",\n      \"title\": \"hablamos y le damos a Little Nightmares 1\",\n      \"tags\": [\"Español\"],\n      \"viewer_count\": 78365,\n      \"started_at\": \"2021-03-10T15:04:21Z\",\n      \"language\": \"es\",\n      \"thumbnail_url\": \"https://static-cdn.jtvnw.net/previews-ttv/live_user_auronplay-{width}x{height}.jpg\",\n      \"tag_ids\": [],\n      \"is_mature\": false\n    }\n  ],\n  \"pagination\": {\n    \"cursor\": \"eyJiIjp7IkN1cnNvciI6ImV5SnpJam8zT0RNMk5TNDBORFF4TlRjMU1UY3hOU3dpWkNJNlptRnNjMlVzSW5RaU9uUnlkV1Y5In0sImEiOnsiQ3Vyc29yIjoiZXlKeklqb3hOVGs0TkM0MU56RXhNekExTVRZNU1ESXNJbVFpT21aaGJITmxMQ0owSWpwMGNuVmxmUT09In19\"\n  }\n}",
    module: twitch_highway::streams::response::StreamsResponse,
    de: StreamsResponse
);

fn_expected_resopnse!(
    name: response_two,
    payload: "{\n  \"data\": [\n    {\n      \"id\": \"40952121085\",\n      \"user_id\": \"101051819\",\n      \"user_login\": \"afro\",\n      \"user_name\": \"Afro\",\n      \"game_id\": \"32982\",\n      \"game_name\": \"Grand Theft Auto V\",\n      \"type\": \"live\",\n      \"title\": \"Jacob: Digital Den Laptops & Routers | NoPixel | !MAINGEAR !FCF\",\n      \"tags\": [\"English\"],\n      \"viewer_count\": 1490,\n      \"started_at\": \"2021-03-10T03:18:11Z\",\n      \"language\": \"en\",\n      \"thumbnail_url\": \"https://static-cdn.jtvnw.net/previews-ttv/live_user_afro-{width}x{height}.jpg\",\n      \"tag_ids\": [],\n      \"is_mature\": false\n    }\n  ],\n  \"pagination\": {}\n}",
    module: twitch_highway::streams::response::StreamsResponse,
    de: StreamsResponse
);
