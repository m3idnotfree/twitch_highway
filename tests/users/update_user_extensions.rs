//fn_expected_request!(
//    modules: [
//        twitch_highway::users::UserAPI,
//        twitch_highway::users::types::UserActiveExtensions,
//        twitch_oauth_token::types::Scope,
//        std::collections::HashMap
//    ],
//    endpoint: update_user_extensions,
//    token_type: User,
//    scopes: Some(vec![Scope::UserEditBroadcast]),
//    args: [UserActiveExtensions::new(
//        HashMap::new(),
//        HashMap::new(),
//        HashMap::new())
//],
//    method: PUT,
//    header: expected_headers!(json),
//    url: "https://api.twitch.tv/helix/users/extensions"
//);

//use twitch_highway::{
//    types::Id,
//    users::types::{Component, Overlay, Panel},
//};

//#[test]
//fn request() {
//    use std::collections::HashMap;
//    use twitch_highway::users::types::UserActiveExtensions;
//    use twitch_highway::users::UserAPI;
//    use twitch_oauth_token::types::Scope;
//
//    use asknothingx2_util::api::APIRequest;
//    use twitch_highway::TokenType;
//
//    let mut panel = HashMap::new();
//    panel.insert(
//        "1".to_string(),
//        Panel::new(true)
//            .id(Id::new("rh6jq1q334hqc2rr1qlzqbvwlfl3x0"))
//            .version("1.1.0".to_string()),
//    );
//    panel.insert(
//        "2".to_string(),
//        Panel::new(true)
//            .id(Id::new("wi08ebtatdc7oj83wtl9uxwz807l8b"))
//            .version("1.1.8".to_string()),
//    );
//    panel.insert(
//        "3".to_string(),
//        Panel::new(true)
//            .id(Id::new("naty2zwfp7vecaivuve8ef1hohh6bo"))
//            .version("1.0.9".to_string()),
//    );
//
//    let mut overlay = HashMap::new();
//    overlay.insert(
//        "1".to_string(),
//        Overlay::new(true)
//            .id(Id::new("zfh2irvx2jb4s60f02jq0ajm8vwgka"))
//            .version("1.0.19".to_string()),
//    );
//    eprint!("{:#?}", panel);
//
//    let mut component = HashMap::new();
//    component.insert(
//        "1".to_string(),
//        Component::new(true)
//            .id(Id::new("lqnf3zxk0rv0g7gq92mtmnirjz2cjj"))
//            .version("0.0.1".to_string())
//            .x(0)
//            .y(0),
//    );
//    component.insert("2".to_string(), Component::new(false));
//
//    let api = api!();
//    let endpoint = api.update_user_extensions(UserActiveExtensions::new(panel, overlay, component));
//
//    fn_expected_request!(@required User Some(vec![Scope::UserEditBroadcast]), endpoint);
//    expected_APIRequest!(@method PUT, endpoint);
//    expected_APIRequest!(@header expected_headers!(json), endpoint);
//    expected_APIRequest!(@url "https://api.twitch.tv/helix/users/extensions", endpoint);
//
//    let js_contain = endpoint.json().unwrap();
//    assert!(js_contain.contains(
//        "\"panel\":{\"1\":{\"active\":true,\"id\":\"rh6jq1q334hqc2rr1qlzqbvwlfl3x0\",\"version\":\"1.1.0\"}"
//    //    "\"panel\":{\"1\":{\"active\":true,\"id\":\"rh6jq1q334hqc2rr1qlzqbvwlfl3x0\",\"version\":\"1.1.0\"}"
//    ));
//    assert!(js_contain.contains(
//        "\"2\":{\"active\":true,\"id\":\"wi08ebtatdc7oj83wtl9uxwz807l8b\",\"version\":\"1.1.8\"}"
//    ));
//    assert!(js_contain.contains(
//        "\"3\":{\"active\":true,\"id\":\"naty2zwfp7vecaivuve8ef1hohh6bo\",\"version\":\"1.0.9\"}"
//    ));
//    assert!(js_contain.contains(
//        "\"overlay\":{\"1\":{\"active\":true,\"id\":\"zfh2irvx2jb4s60f02jq0ajm8vwgka\",\"version\":\"1.0.19\"}"
//    ));
//    assert!(js_contain.contains(
//"\"component\":{\"1\":{\"active\":true,\"id\":\"lqnf3zxk0rv0g7gq92mtmnirjz2cjj\",\"version\":\"0.0.1\",\"x\":0,\"y\":0}"
//));
//assert!(js_contain.contains("\"2\":{\"active\":false"));
//}
