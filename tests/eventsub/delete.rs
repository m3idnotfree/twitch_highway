fn_expected_request!(
    api:twitch_highway::eventsub::EventSubAPI,
    endpoint: delete,
    token_type: App,
    scopes: None,
    args: ["26b1c993-bfcf-44d9-b876-379dacafe75a"],
    method: DELETE,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/eventsub/subscriptions?id=26b1c993-bfcf-44d9-b876-379dacafe75a",
    json: None,
    text: None,
    urlencoded: None
);
