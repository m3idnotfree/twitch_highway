use twitch_highway::{types::Id, videos::request::VideosRequest};

fn_expected_request!(
    api: twitch_highway::videos::VideosAPI,
    endpoint: get_videos,
    token_type: Any,
    scopes: None,
    args: [VideosRequest::new().id(vec![Id::new("335921245")])],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/videos?id=335921245"
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"id\": \"335921245\",\n      \"stream_id\": null,\n      \"user_id\": \"141981764\",\n      \"user_login\": \"twitchdev\",\n      \"user_name\": \"TwitchDev\",\n      \"title\": \"Twitch Developers 101\",\n      \"description\": \"Welcome to Twitch development! Here is a quick overview of our products and information to help you get started.\",\n      \"created_at\": \"2018-11-14T21:30:18Z\",\n      \"published_at\": \"2018-11-14T22:04:30Z\",\n      \"url\": \"https://www.twitch.tv/videos/335921245\",\n      \"thumbnail_url\": \"https://static-cdn.jtvnw.net/cf_vods/d2nvs31859zcd8/twitchdev/335921245/ce0f3a7f-57a3-4152-bc06-0c6610189fb3/thumb/index-0000000000-%{width}x%{height}.jpg\",\n      \"viewable\": \"public\",\n      \"view_count\": 1863062,\n      \"language\": \"en\",\n      \"type\": \"upload\",\n      \"duration\": \"3m21s\",\n      \"muted_segments\": [\n        {\n          \"duration\": 30,\n          \"offset\": 120\n        }\n      ]\n    }\n  ],\n  \"pagination\": {}\n}",
    module: twitch_highway::videos::response::VideosResponse,
    de: VideosResponse
);
