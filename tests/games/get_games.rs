use twitch_highway::types::Id;

fn_expected_request!(
    api: twitch_highway::games::GamesAPI,
    endpoint: get_games,
    token_type: Any,
    scopes: None,
    args: [Some(&[Id::new("33214")]), None, None],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/games?id=33214"
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"id\": \"33214\",\n      \"name\": \"Fortnite\",\n      \"box_art_url\": \"https://static-cdn.jtvnw.net/ttv-boxart/33214-{width}x{height}.jpg\",\n      \"igdb_id\": \"1905\"\n    }\n  ]\n}",
    module: twitch_highway::games::response::GamesResponse,
    de: GamesResponse
);
