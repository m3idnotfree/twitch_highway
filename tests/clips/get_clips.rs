fn_expected_request!(
    modules: [
        twitch_highway::clips::ClipsAPI,
        twitch_highway::clips::request::ClipsSelector,
        twitch_highway::types::Id
    ],
    endpoint: get_clips,
    token_type: Any,
    scopes: None,
    args: [
        ClipsSelector::by_ids(&[Id::new("AwkwardHelplessSalamanderSwiftRage")]),
        None,
        None
    ],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/clips?id=AwkwardHelplessSalamanderSwiftRage",
    json: None
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"id\": \"AwkwardHelplessSalamanderSwiftRage\",\n      \"url\": \"https://clips.twitch.tv/AwkwardHelplessSalamanderSwiftRage\",\n      \"embed_url\": \"https://clips.twitch.tv/embed?clip=AwkwardHelplessSalamanderSwiftRage\",\n      \"broadcaster_id\": \"67955580\",\n      \"broadcaster_name\": \"ChewieMelodies\",\n      \"creator_id\": \"53834192\",\n      \"creator_name\": \"BlackNova03\",\n      \"video_id\": \"205586603\",\n      \"game_id\": \"488191\",\n      \"language\": \"en\",\n      \"title\": \"babymetal\",\n      \"view_count\": 10,\n      \"created_at\": \"2017-11-30T22:34:18Z\",\n      \"thumbnail_url\": \"https://clips-media-assets.twitch.tv/157589949-preview-480x272.jpg\",\n      \"duration\": 60,\n      \"vod_offset\": 480,\n      \"is_featured\": false\n    }\n  ]\n}",
    module: twitch_highway::clips::response::ClipsInfoResponse,
    de: ClipsInfoResponse
);
