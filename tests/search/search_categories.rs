fn_expected_request!(
    modules: [twitch_highway::search::SearchAPI],
    endpoint: search_categories,
    token_type: Any,
    scopes: None,
    args: ["fort", None],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/search/categories?query=fort"
);

fn_expected_request!(
    name: request_url_encoded,
    modules: [twitch_highway::search::SearchAPI],
    endpoint: search_categories,
    token_type: Any,
    scopes: None,
    args: ["#archery", None],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/search/categories?query=%23archery"
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"id\": \"33214\",\n      \"name\": \"Fortnite\",\n      \"box_art_url\": \"https://static-cdn.jtvnw.net/ttv-boxart/33214-52x72.jpg\"\n    }\n  ],\n  \"pagination\": {\n    \"cursor\": \"eyJiIjpudWxsLCJhIjp7IkN\"\n  }\n}",
    module: twitch_highway::search::response::CategoriesResponse,
    de: CategoriesResponse
);
