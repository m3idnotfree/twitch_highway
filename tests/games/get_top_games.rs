fn_expected_request!(
    api: twitch_highway::games::GamesAPI,
    endpoint: get_top_games,
    token_type: Any,
    scopes: None,
    args: [None, None, None],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/games/top"
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"id\": \"493057\",\n      \"name\": \"PUBG: BATTLEGROUNDS\",\n      \"box_art_url\": \"https://static-cdn.jtvnw.net/ttv-boxart/493057-{width}x{height}.jpg\",\n      \"igdb_id\": \"27789\"\n    }\n  ],\n  \"pagination\":{\"cursor\":\"eyJiIjpudWxsLCJhIjp7Ik9mZnNldCI6MjB9fQ==\"}\n}",
    module: twitch_highway::games::response::GamesResponse,
    de: GamesResponse
);
