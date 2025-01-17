fn_expected_request!(
    modules: [
        twitch_highway::chat::ChatAPI,
        twitch_highway::types::BroadcasterId,
        twitch_highway::types::ModeratorId,
        twitch_oauth_token::types::Scope
    ],
    endpoint: get_chatters,
    token_type: User,
    scopes: Some(vec![Scope::ModeratorReadChatters]),
    args: [BroadcasterId::new("123456"), ModeratorId::new("654321"), None],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/chat/chatters?broadcaster_id=123456&moderator_id=654321",
    json: None,
    text: None,
    urlencoded: None
);

fn_expected_request!(
    name: set_first,
    modules: [
        twitch_highway::chat::ChatAPI,
        twitch_highway::types::PaginationQuery,
        twitch_highway::types::BroadcasterId,
        twitch_highway::types::ModeratorId,
        twitch_oauth_token::types::Scope
    ],
    endpoint: get_chatters,
    token_type: User,
    scopes: Some(vec![Scope::ModeratorReadChatters]),
    args: [BroadcasterId::new("123456"), ModeratorId::new("654321"), Some(PaginationQuery::new().first(40))],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/chat/chatters?broadcaster_id=123456&moderator_id=654321&first=40",
    json: None,
    text: None,
    urlencoded: None
);

fn_expected_request!(
    name: set_after,
    modules: [
        twitch_highway::chat::ChatAPI,
        twitch_highway::types::PaginationQuery,
        twitch_highway::types::BroadcasterId,
        twitch_highway::types::ModeratorId,
        twitch_oauth_token::types::Scope
    ],
    endpoint: get_chatters,
    token_type: User,
    scopes: Some(vec![Scope::ModeratorReadChatters]),
    args: [
        BroadcasterId::new("123456"),
        ModeratorId::new("654321"),
        Some(PaginationQuery::new().after("eyJiIjpudWxsLCJhIjp7Ik9mZnNldCI6NX19"))
    ],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/chat/chatters?broadcaster_id=123456&moderator_id=654321&after=eyJiIjpudWxsLCJhIjp7Ik9mZnNldCI6NX19",
    json: None,
    text: None,
    urlencoded: None
);

fn_expected_request!(
    name: set_first_after,
    modules: [
        twitch_highway::chat::ChatAPI,
        twitch_highway::types::PaginationQuery,
        twitch_highway::types::BroadcasterId,
        twitch_highway::types::ModeratorId,
        twitch_oauth_token::types::Scope
    ],
    endpoint: get_chatters,
    token_type: User,
    scopes: Some(vec![Scope::ModeratorReadChatters]),
    args: [
        BroadcasterId::new("123456"),
        ModeratorId::new("654321"),
        Some(PaginationQuery::new().first(40).after("eyJiIjpudWxsLCJhIjp7Ik9mZnNldCI6NX19"))
    ],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/chat/chatters?broadcaster_id=123456&moderator_id=654321&first=40&after=eyJiIjpudWxsLCJhIjp7Ik9mZnNldCI6NX19",
    json: None,
    text: None,
    urlencoded: None
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"user_id\": \"128393656\",\n      \"user_login\": \"smittysmithers\",\n      \"user_name\": \"smittysmithers\"\n    }\n  ],\n  \"pagination\": {\n    \"cursor\": \"eyJiIjpudWxsLCJhIjp7Ik9mZnNldCI6NX19\"\n  },\n  \"total\": 8\n}",
    module: twitch_highway::chat::response::ChattersResponse,
    de: ChattersResponse
);
fn_expected_resopnse!(
    name:pagination_empty,
    payload: "{\n  \"data\": [\n    {\n      \"user_id\": \"128393656\",\n      \"user_login\": \"smittysmithers\",\n      \"user_name\": \"smittysmithers\"\n    }\n  ],\n  \"pagination\": {},\n  \"total\": 8\n}",
    module: twitch_highway::chat::response::ChattersResponse,
    de: ChattersResponse
);

//fn_expected_resopnse!(
//    name: remove_pagination_field,
//    payload: "{\"data\":[{\"user_id\":\"35249427\",\"user_login\":\"banjomarston94\",\"user_name\":\"BanjoMarston94\"r224\",\"user_name\":\"IsaacDeveloper224\"},{\"user_id\":\"38378526\",\"user_login\":\"isaacdeveloper440\",\"user_name\":\"IsaacDeveloper440\"},{\"user_id\":\"21975966\",\"user_login\":\"larachief703\",\"user_name\":\"LaraChief703\"},{\"user_id\":\"13078989\",\"user_login\":\"chiefdeveloper317\",\"user_name\":\"ChiefDeveloper317\"},{\"user_id\":\"94636129\",\"user_login\":\"marstonmarston521\",\"user_name\":\"MarstonMarston521\"},{\"user_id\":\"67933835\",\"user_login\":\"fisherlion712\",\"user_name\":\"FisherLion712\"},{\"user_id\":\"52265672\",\"user_login\":\"concretesteve135\",\"user_name\":\"ConcreteSteve135\"},{\"user_id\":\"63632471\",\"user_login\":\"shepardconcrete229\",\"user_name\":\"ShepardConcrete229\"},{\"user_id\":\"85576418\",\"user_login\":\"jilllion755\",\"user_name\":\"JillLion755\"},{\"user_id\":\"61957881\",\"user_login\":\"drakeentree556\",\"user_name\":\"DrakeEntree556\"},{\"user_id\":\"82566202\",\"user_login\":\"drakeshepard86\",\"user_name\":\"DrakeShepard86\"},{\"user_id\":\"90208493\",\"user_login\":\"drakelara784\",\"user_name\":\"DrakeLara784\"},{\"user_id\":\"35886814\",\"user_login\":\"davelion218\",\"user_name\":\"DaveLion218\"},{\"user_id\":\"4051430\",\"user_login\":\"dukekid683\",\"user_name\":\"DukeKid683\"},{\"user_id\":\"98447595\",\"user_login\":\"liondave405\",\"user_name\":\"LionDave405\"},{\"user_id\":\"25843901\",\"user_login\":\"chiefkomodohype169\",\"user_name\":\"ChiefKomodoHype169\"},{\"user_id\":\"29811993\",\"user_login\":\"dukebanjo196\",\"user_name\":\"DukeBanjo196\"},{\"user_id\":\"45643680\",\"user_login\":\"laraconcrete0\",\"user_name\":\"LaraConcrete0\"},{\"user_id\":\"11265581\",\"user_login\":\"eggdave930\",\"user_name\":\"EggDave930\"}],\"total\":50}",
//    module: twitch_highway::chat::response::ChattersResponse,
//    de: ChattersResponse
//);
