#[macro_use]
mod support;

#[cfg(feature = "eventsub")]
#[test]
fn create_eventsub() {
    use std::collections::HashMap;

    use asknothingx2_util::oauth::{AccessToken, ClientId};
    use twitch_highway::{
        eventsub::create::CreateEventSub,
        types::{SubscriptionTypes, Transport},
    };

    let mut condition = HashMap::new();
    condition.insert("user_id", "1234");

    let create_eventsub = CreateEventSub::new(
        AccessToken::new("cfabdegwdoklmawdzdo98xt2fo512y".to_string()),
        ClientId::new("uo6dggojyb8d6soh92zknwmi5ej1q2".to_string()),
        SubscriptionTypes::UserUpdate,
        HashMap::new(),
        Transport::websocket(""),
    )
    .set_condition(condition);

    let expected_headers = expect_headers!(json);
    let expected_json ="{\"type\":\"user.update\",\"version\":\"1\",\"condition\":{\"user_id\":\"1234\"},\"transport\":{\"method\":\"websocket\",\"session_id\":\"\"}}".to_string();

    expect_APIRequest!(
        POST,
        expected_headers,
        "https://api.twitch.tv/helix/eventsub/subscriptions",
        json = Some(expected_json),
        text = None,
        urlencoded = None,
        create_eventsub
    );
}

#[cfg(feature = "eventsub")]
#[test]
fn delete_eventsub() {
    use twitch_highway::eventsub::delete::DeleteEventSub;

    let delete_eventsub = api_general!(DeleteEventSub, "26b1c993-bfcf-44d9-b876-379dacafe75a");

    let expected_headers = expect_headers!();

    expect_APIRequest!(
            DELETE,
            expected_headers,
            "https://api.twitch.tv/helix/eventsub/subscriptions?id=26b1c993-bfcf-44d9-b876-379dacafe75a",
            json = None,
            text = None,
            urlencoded = None,
            delete_eventsub
        );
}

#[cfg(feature = "eventsub")]
#[test]
fn get_eventsub() {
    use twitch_highway::eventsub::get::GetEventSub;

    let get_eventsub = api_general!(GetEventSub);

    expect_APIRequest!(
        GET,
        expect_headers!(),
        "https://api.twitch.tv/helix/eventsub/subscriptions",
        json = None,
        text = None,
        urlencoded = None,
        get_eventsub
    );
}

#[cfg(feature = "eventsub")]
#[test]
fn get_eventsub_response() {
    use twitch_highway::eventsub::get::GetEventResponse;

    expect_response_json!("{\n  \"total\": 2,\n  \"data\": [\n    {\n      \"id\": \"26b1c993-bfcf-44d9-b876-379dacafe75a\",\n      \"status\": \"enabled\",\n      \"type\": \"stream.online\",\n      \"version\": \"1\",\n      \"condition\": {\n        \"broadcaster_user_id\": \"1234\"\n      },\n      \"created_at\": \"2020-11-10T20:08:33.12345678Z\",\n      \"transport\": {\n        \"method\": \"webhook\",\n        \"callback\": \"https://this-is-a-callback.com\"\n      },\n      \"cost\": 1\n    },\n    {\n      \"id\": \"35016908-41ff-33ce-7879-61b8dfc2ee16\",\n      \"status\": \"webhook_callback_verification_pending\",\n      \"type\": \"user.update\",\n      \"version\": \"1\",\n      \"condition\": {\n        \"user_id\": \"1234\"\n      },\n      \"created_at\": \"2020-11-10T14:32:18.730260295Z\",\n      \"transport\": {\n        \"method\": \"webhook\",\n        \"callback\": \"https://this-is-a-callback.com\"\n      },\n      \"cost\": 0\n    }\n  ],\n  \"total_cost\": 1,\n  \"max_total_cost\": 10000,\n  \"pagination\": {}\n}",
        GetEventResponse);
    let data_pagination_none = serde_json::from_str::<GetEventResponse>("{\n  \"total\": 2,\n  \"data\": [\n    {\n      \"id\": \"26b1c993-bfcf-44d9-b876-379dacafe75a\",\n      \"status\": \"enabled\",\n      \"type\": \"stream.online\",\n      \"version\": \"1\",\n      \"condition\": {\n        \"broadcaster_user_id\": \"1234\"\n      },\n      \"created_at\": \"2020-11-10T20:08:33.12345678Z\",\n      \"transport\": {\n        \"method\": \"webhook\",\n        \"callback\": \"https://this-is-a-callback.com\"\n      },\n      \"cost\": 1\n    },\n    {\n      \"id\": \"35016908-41ff-33ce-7879-61b8dfc2ee16\",\n      \"status\": \"webhook_callback_verification_pending\",\n      \"type\": \"user.update\",\n      \"version\": \"1\",\n      \"condition\": {\n        \"user_id\": \"1234\"\n      },\n      \"created_at\": \"2020-11-10T14:32:18.730260295Z\",\n      \"transport\": {\n        \"method\": \"webhook\",\n        \"callback\": \"https://this-is-a-callback.com\"\n      },\n      \"cost\": 0\n    }\n  ],\n  \"total_cost\": 1,\n  \"max_total_cost\": 10000,\n  \"pagination\": {}\n}").unwrap();
    assert!(data_pagination_none.pagination.is_none());

    let data_pagination_some:GetEventResponse = serde_json::from_str("{\n  \"total\": 2,\n  \"data\": [\n    {\n      \"id\": \"26b1c993-bfcf-44d9-b876-379dacafe75a\",\n      \"status\": \"enabled\",\n      \"type\": \"stream.online\",\n      \"version\": \"1\",\n      \"condition\": {\n        \"broadcaster_user_id\": \"1234\"\n      },\n      \"created_at\": \"2020-11-10T20:08:33.12345678Z\",\n      \"transport\": {\n        \"method\": \"webhook\",\n        \"callback\": \"https://this-is-a-callback.com\"\n      },\n      \"cost\": 1\n    },\n    {\n      \"id\": \"35016908-41ff-33ce-7879-61b8dfc2ee16\",\n      \"status\": \"webhook_callback_verification_pending\",\n      \"type\": \"user.update\",\n      \"version\": \"1\",\n      \"condition\": {\n        \"user_id\": \"1234\"\n      },\n      \"created_at\": \"2020-11-10T14:32:18.730260295Z\",\n      \"transport\": {\n        \"method\": \"webhook\",\n        \"callback\": \"https://this-is-a-callback.com\"\n      },\n      \"cost\": 0\n    }\n  ],\n  \"total_cost\": 1,\n  \"max_total_cost\": 10000,\n  \"pagination\": {\n\"cursor\":\"eyJiIjpudWxsLCJhIjp7Ik9mZnNldCI6NX19\"}\n}").unwrap();
    assert_eq!(
        data_pagination_some.pagination.unwrap().cursor,
        "eyJiIjpudWxsLCJhIjp7Ik9mZnNldCI6NX19".to_string()
    );
}
