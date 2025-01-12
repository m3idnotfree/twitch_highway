use twitch_highway::types::BroadcasterId;

fn_expected_request!(
    api: twitch_highway::subscriptions::SubscriptionsAPI,
    endpoint: get_broadcaster_subscriptions,
    token_type: User,
    scopes: Some(vec![Scope::ChannelReadSubscriptions]),
    args: [
        BroadcasterId::new("141981764"),
        None,
        None,
        None,
        None
    ],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/subscriptions?broadcaster_id=141981764",
    json: None,
    text: None,
    urlencoded: None
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"broadcaster_id\": \"141981764\",\n      \"broadcaster_login\": \"twitchdev\",\n      \"broadcaster_name\": \"TwitchDev\",\n      \"gifter_id\": \"12826\",\n      \"gifter_login\": \"twitch\",\n      \"gifter_name\": \"Twitch\",\n      \"is_gift\": true,\n      \"tier\": \"1000\",\n      \"plan_name\": \"Channel Subscription (twitchdev)\",\n      \"user_id\": \"527115020\",\n      \"user_name\": \"twitchgaming\",\n      \"user_login\": \"twitchgaming\"\n    }\n  ],\n  \"pagination\": {\n    \"cursor\": \"xxxx\"\n  },\n  \"total\": 13,\n  \"points\": 13\n}",
    module: twitch_highway::subscriptions::response::BroadcasterSubscriptionResponse,
    de: BroadcasterSubscriptionResponse
);
