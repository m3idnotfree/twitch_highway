fn_expected_request!(
    api:twitch_highway::chat::ChatAPI,
    endpoint: get_shared_chat_session,
    token_type: Any,
    scopes: None,
    args: ["198704263"],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/shared_chat/session?broadcaster_id=198704263",
    json: None,
    text: None,
    urlencoded: None
);

fn_expected_resopnse!(
    payload: "{\n          \"data\": [\n            {\n              \"session_id\": \"359bce59-fa4e-41a5-bd6f-9bc0c8360485\",\n              \"host_broadcaster_id\": \"198704263\",\n              \"participants\": [{\n                  \"broadcaster_id\": \"198704263\"\n              }, {\n                  \"broadcaster_id\": \"487263401\"\n              }],\n              \"created_at\": \"2024-09-29T19:45:37Z\",\n              \"updated_at\": \"2024-09-29T19:45:37Z\"\n            }\n          ]\n        }",
    module: twitch_highway::chat::response::SharedChatSessionResponse,
    de: SharedChatSessionResponse
);

//#[test]
//fn get_shared_chat_session() {
//    let chat_api = api!();
//    let get_shared_chat_session = chat_api.get_shared_chat_session("198704263");
//
//    expected_APIRequest!(
//        get_shared_chat_session,
//        method: GET,
//        header: expected_headers!(),
//        url: "https://api.twitch.tv/helix/shared_chat/session?broadcaster_id=198704263",
//        json: None,
//        text: None,
//        urlencoded: None
//    );
//}

//#[test]
//fn get_shared_chat_session_response() {
//    use twitch_highway::{chat::response::SharedChatSessionResponse, Datelike, Timelike};
//
//    let data ="{\n          \"data\": [\n            {\n              \"session_id\": \"359bce59-fa4e-41a5-bd6f-9bc0c8360485\",\n              \"host_broadcaster_id\": \"198704263\",\n              \"participants\": [{\n                  \"broadcaster_id\": \"198704263\"\n              }, {\n                  \"broadcaster_id\": \"487263401\"\n              }],\n              \"created_at\": \"2024-09-29T19:45:37Z\",\n              \"updated_at\": \"2024-09-29T19:45:37Z\"\n            }\n          ]\n        }";
//    let shared_chat_sesseion_response: SharedChatSessionResponse =
//        serde_json::from_str(data).unwrap();
//
//    let first = shared_chat_sesseion_response.data.first().unwrap();
//    assert_eq!(first.created_at.year(), 2024);
//    assert_eq!(first.created_at.month(), 9);
//    assert_eq!(first.created_at.day(), 29);
//    assert_eq!(first.created_at.hour(), 19);
//    assert_eq!(first.created_at.minute(), 45);
//    assert_eq!(first.created_at.second(), 37);
//}

//#[cfg(all(feature = "chat", feature = "test"))]
//#[test]
//fn get_shared_chat_session_feature_test() {
//    use twitch_highway::TestUrl;
//
//    let chat_api = api!();
//    let mut get_shared_chat_session = chat_api.get_shared_chat_session("198704263");
//    get_shared_chat_session.with_url("https://test.url/shared_chat/session");
//
//    expected_APIRequest!(
//        get_shared_chat_session,
//        method: GET,
//        header: expected_headers!(),
//        url: "https://test.url/shared_chat/session?broadcaster_id=198704263",
//        json: None,
//        text: None,
//        urlencoded: None
//    );
//}
