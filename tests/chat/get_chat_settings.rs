fn_expected_request!(
    api:twitch_highway::chat::ChatAPI,
    endpoint: get_chat_settings,
    token_type: Any,
    scopes: None,
    args: ["1234",None],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/chat/settings?broadcaster_id=1234",
    json: None,
    text: None,
    urlencoded: None
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"broadcaster_id\": \"713936733\",\n      \"slow_mode\": false,\n      \"slow_mode_wait_time\": null,\n      \"follower_mode\": true,\n      \"follower_mode_duration\": 0,\n      \"subscriber_mode\": false,\n      \"emote_mode\": false,\n      \"unique_chat_mode\": false,\n      \"non_moderator_chat_delay\": true,\n      \"non_moderator_chat_delay_duration\": 4\n    }\n  ]\n}",
    module: twitch_highway::chat::response::ChatSettingResponse,
    de: ChatSettingResponse
);

//#[test]
//fn get_chat_setting() {
//    let chat_api = api!();
//    let chat_setting = chat_api.get_chat_settings("1234", None);
//
//    expected_APIRequest!(
//        chat_setting,
//        method: GET,
//        header: expected_headers!(),
//        url: "https://api.twitch.tv/helix/chat/settings?broadcaster_id=1234",
//        json: None,
//        text: None,
//        urlencoded: None
//    );
//}

//#[test]
//fn get_chat_settings_response() {
//    use twitch_highway::chat::response::ChatSettingResponse;
//
//    expected_response!(
//        "{\n  \"data\": [\n    {\n      \"broadcaster_id\": \"713936733\",\n      \"slow_mode\": false,\n      \"slow_mode_wait_time\": null,\n      \"follower_mode\": true,\n      \"follower_mode_duration\": 0,\n      \"subscriber_mode\": false,\n      \"emote_mode\": false,\n      \"unique_chat_mode\": false,\n      \"non_moderator_chat_delay\": true,\n      \"non_moderator_chat_delay_duration\": 4\n    }\n  ]\n}",
//        ChatSettingResponse
//    );
//}
