use twitch_highway::types::ExtensionId;

fn_expected_request!(
    api: twitch_highway::extensions::ExtensionsAPI,
    endpoint: get_extension_livemchannels,
    token_type: Any,
    scopes: None,
    args: [ExtensionId::new("uo6dggojyb8d6soh92zknwmi5ej1q2"), None, None
    ],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/extensions/live?extension_id=uo6dggojyb8d6soh92zknwmi5ej1q2"
);

//fn_expected_resopnse!(
//    payload: "{\n  \"data\": [\n    {\n      \"broadcaster_id\": \"252766116\",\n      \"broadcaster_name\": \"swoosh_xii\",\n      \"game_name\": \"Tom Clancy's Rainbow Six Siege\",\n      \"game_id\": \"460630\",\n      \"title\": \"[PS4] ITA/ENG UNRANKED CHILLIN' (SUB 1/15) - !instagram !donation !sens !team !youtube\"\n    },\n    {\n      \"broadcaster_id\": \"264525686\",\n      \"broadcaster_name\": \"gaddem_\",\n      \"game_name\": \"For Honor\",\n      \"game_id\": \"490382\",\n      \"title\": \"any Kätzchen ? - 680 Rep + > Kompetitive Kitten\"\n    },\n    {\n      \"broadcaster_id\": \"264787895\",\n      \"broadcaster_name\": \"LenhadorGameplay\",\n      \"game_name\": \"For Honor\",\n      \"game_id\": \"490382\",\n      \"title\": \"Vazou o novo personagem! *Triste*\"\n    }\n  ],\n  \"pagination\": \"YVc1emRHRnNiQ015TmpJek5qazVOVHBoYWpKbGRIZDFaR0Z5YjNCMGN6UTJNMkZ1TUdwM2FHWnBZbm8yYW5rNjoy\"\n}",
//    module: twitch_highway::extensions::response::ExtensionLiveChannelsRespnose,
//    de: ExtensionLiveChannelsRespnose
//);
