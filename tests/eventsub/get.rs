use twitch_highway::eventsub::request::EventSubRequest;

fn_expected_request!(
    api:twitch_highway::eventsub::EventSubAPI,
    endpoint: get,
    token_type: App,
    scopes: None,
    args: [EventSubRequest::new()],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/eventsub/subscriptions",
    json: None,
    text: None,
    urlencoded: None
);

fn_expected_resopnse!(
    payload: "{\n  \"total\": 2,\n  \"data\": [\n    {\n      \"id\": \"26b1c993-bfcf-44d9-b876-379dacafe75a\",\n      \"status\": \"enabled\",\n      \"type\": \"stream.online\",\n      \"version\": \"1\",\n      \"condition\": {\n        \"broadcaster_user_id\": \"1234\"\n      },\n      \"created_at\": \"2020-11-10T20:08:33.12345678Z\",\n      \"transport\": {\n        \"method\": \"webhook\",\n        \"callback\": \"https://this-is-a-callback.com\"\n      },\n      \"cost\": 1\n    },\n    {\n      \"id\": \"35016908-41ff-33ce-7879-61b8dfc2ee16\",\n      \"status\": \"webhook_callback_verification_pending\",\n      \"type\": \"user.update\",\n      \"version\": \"1\",\n      \"condition\": {\n        \"user_id\": \"1234\"\n      },\n      \"created_at\": \"2020-11-10T14:32:18.730260295Z\",\n      \"transport\": {\n        \"method\": \"webhook\",\n        \"callback\": \"https://this-is-a-callback.com\"\n      },\n      \"cost\": 0\n    }\n  ],\n  \"total_cost\": 1,\n  \"max_total_cost\": 10000,\n  \"pagination\": {}\n}",
    module: twitch_highway::eventsub::response::GetEventResponse,
    de: GetEventResponse
);
