#[macro_use]
mod support;

#[cfg(feature = "chat")]
#[test]
fn get_chat_setting() {
    use twitch_highway::chat::get_chat_setting::GetChatSetting;

    let chat_setting = api_general!(GetChatSetting, "1234");

    let expected_headers = expect_headers!();

    expect_APIRequest!(
        GET,
        expected_headers,
        "https://api.twitch.tv/helix/chat/settings?broadcaster_id=1234",
        json = None,
        text = None,
        urlencoded = None,
        chat_setting
    );
}

#[cfg(feature = "chat")]
#[test]
fn get_chat_settings_response() {
    use twitch_highway::chat::get_chat_setting::ChatSettingResponse;

    expect_response_json!("{\n  \"data\": [\n    {\n      \"broadcaster_id\": \"713936733\",\n      \"slow_mode\": false,\n      \"slow_mode_wait_time\": null,\n      \"follower_mode\": true,\n      \"follower_mode_duration\": 0,\n      \"subscriber_mode\": false,\n      \"emote_mode\": false,\n      \"unique_chat_mode\": false,\n      \"non_moderator_chat_delay\": true,\n      \"non_moderator_chat_delay_duration\": 4\n    }\n  ]\n}",
            ChatSettingResponse);
}

#[cfg(feature = "chat")]
#[test]
fn get_chatters() {
    use twitch_highway::chat::get_chatters::GetChatters;
    let get_chatters = api_general!(GetChatters, "123456", "654321");

    let expected_headers = expect_headers!();

    expect_APIRequest!(
        GET,
        expected_headers,
        "https://api.twitch.tv/helix/chat/chatters?broadcaster_id=123456&moderator_id=654321",
        json = None,
        text = None,
        urlencoded = None,
        get_chatters
    );
}

#[cfg(feature = "chat")]
#[test]
fn get_chatters_set_first() {
    use twitch_highway::chat::get_chatters::GetChatters;
    let mut get_chatters = api_general!(GetChatters, "123456", "654321");

    get_chatters.set_first(40);

    let expected_headers = expect_headers!();

    expect_APIRequest!(
        GET,
        expected_headers,
        "https://api.twitch.tv/helix/chat/chatters?broadcaster_id=123456&moderator_id=654321&first=40",
        json = None,
        text = None,
        urlencoded = None,
        get_chatters
    );
}
#[cfg(feature = "chat")]
#[test]
fn get_chatters_set_after() {
    use twitch_highway::chat::get_chatters::GetChatters;
    let mut get_chatters = api_general!(GetChatters, "123456", "654321");

    get_chatters.set_after("eyJiIjpudWxsLCJhIjp7Ik9mZnNldCI6NX19");

    let expected_headers = expect_headers!();

    expect_APIRequest!(
        GET,
        expected_headers,
        "https://api.twitch.tv/helix/chat/chatters?broadcaster_id=123456&moderator_id=654321&after=eyJiIjpudWxsLCJhIjp7Ik9mZnNldCI6NX19",
        json = None,
        text = None,
        urlencoded = None,
        get_chatters
    );
}

#[cfg(feature = "chat")]
#[test]
fn get_chatters_set_first_after() {
    use twitch_highway::chat::get_chatters::GetChatters;
    let mut get_chatters = api_general!(GetChatters, "123456", "654321");

    get_chatters.set_first(40);
    get_chatters.set_after("eyJiIjpudWxsLCJhIjp7Ik9mZnNldCI6NX19");

    let expected_headers = expect_headers!();

    expect_APIRequest!(
        GET,
        expected_headers,
        "https://api.twitch.tv/helix/chat/chatters?broadcaster_id=123456&moderator_id=654321&first=40&after=eyJiIjpudWxsLCJhIjp7Ik9mZnNldCI6NX19",
        json = None,
        text = None,
        urlencoded = None,
        get_chatters
    );
}

#[cfg(feature = "chat")]
#[test]
fn chatters() {
    use twitch_highway::chat::get_chatters::ChattersResponse;
    expect_response_json!(
        "{\n    \"data\": [\n        {\n            \"user_id\": \"128393656\",\n            \"user_login\": \"smittysmithers\",\n            \"user_name\": \"smittysmithers\"\n        }\n    ],\n    \"pagination\": {\n        \"cursor\": \"eyJiIjpudWxsLCJhIjp7Ik9mZnNldCI6NX19\"\n    },\n    \"total\": 8\n}",
        ChattersResponse
    );
}

#[cfg(feature = "chat")]
#[test]
fn get_shared_chat_session() {
    use twitch_highway::chat::get_shared_chat_session::GetSharedChatSession;
    let get_shared_chat_session = api_general!(GetSharedChatSession, "198704263");

    expect_APIRequest!(
        GET,
        expect_headers!(),
        "https://api.twitch.tv/helix/shared_chat/session?broadcaster_id=198704263",
        json = None,
        text = None,
        urlencoded = None,
        get_shared_chat_session
    );
}

#[cfg(feature = "chat")]
#[test]
fn get_shared_chat_session_response() {
    use twitch_highway::{
        chat::get_shared_chat_session::SharedChatSessionResponse, Datelike, Timelike,
    };

    let data ="{\n          \"data\": [\n            {\n              \"session_id\": \"359bce59-fa4e-41a5-bd6f-9bc0c8360485\",\n              \"host_broadcaster_id\": \"198704263\",\n              \"participants\": [{\n                  \"broadcaster_id\": \"198704263\"\n              }, {\n                  \"broadcaster_id\": \"487263401\"\n              }],\n              \"created_at\": \"2024-09-29T19:45:37Z\",\n              \"updated_at\": \"2024-09-29T19:45:37Z\"\n            }\n          ]\n        }";
    let shared_chat_sesseion_response: SharedChatSessionResponse =
        serde_json::from_str(data).unwrap();

    let first = shared_chat_sesseion_response.data.first().unwrap();
    assert_eq!(first.created_at.year(), 2024);
    assert_eq!(first.created_at.month(), 9);
    assert_eq!(first.created_at.day(), 29);
    assert_eq!(first.created_at.hour(), 19);
    assert_eq!(first.created_at.minute(), 45);
    assert_eq!(first.created_at.second(), 37);
}

#[cfg(all(feature = "chat", feature = "test"))]
#[test]
fn get_shared_chat_session_feature_test() {
    use twitch_highway::chat::get_shared_chat_session::GetSharedChatSession;
    use twitch_highway::TestUrl;

    let mut get_shared_chat_session = api_general!(GetSharedChatSession, "198704263");
    get_shared_chat_session.with_url("https://test.url/shared_chat/session");
    expect_APIRequest!(
        GET,
        expect_headers!(),
        "https://test.url/shared_chat/session?broadcaster_id=198704263",
        json = None,
        text = None,
        urlencoded = None,
        get_shared_chat_session
    );
}

#[cfg(feature = "chat")]
#[test]
fn send_chat_message() {
    use twitch_highway::chat::send_chat_message::SendChatMessage;
    let mut send_chat_message = api_general!(SendChatMessage, "12826", "141981764");
    send_chat_message.set_message("Hello, world! twitchdevHype");

    expect_APIRequest!(
        POST,
        expect_headers!(json),
        "https://api.twitch.tv/helix/chat/messages",
        json = Some("{\"broadcaster_id\":\"12826\",\"sender_id\":\"141981764\",\"message\":\"Hello, world! twitchdevHype\"}".to_string()),
        text = None,
        urlencoded= None,
        send_chat_message
    );
}

#[cfg(feature = "chat")]
#[test]
fn send_chat_message_response() {
    use twitch_highway::chat::send_chat_message::SendChatMessageResponse;
    expect_response_json!("{\n  \"data\": [\n    {\n      \"message_id\": \"abc-123-def\",\n      \"is_sent\": true\n    }\n  ]\n}",
    SendChatMessageResponse);
}
