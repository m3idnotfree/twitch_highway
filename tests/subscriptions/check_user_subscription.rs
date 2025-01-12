use twitch_highway::types::{BroadcasterId, UserId};

fn_expected_request!(
    api: twitch_highway::subscriptions::SubscriptionsAPI,
    endpoint: check_user_subscpition,
    token_type: User,
    scopes: Some(vec![Scope::UserReadSubscriptions]),
    args: [BroadcasterId::new("149747285"), UserId::new("141981764")],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/subscriptions/user?broadcaster_id=149747285&user_id=141981764",
    json: None,
    text: None,
    urlencoded: None
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"broadcaster_id\": \"149747285\",\n      \"broadcaster_name\": \"TwitchPresents\",\n      \"broadcaster_login\": \"twitchpresents\",\n      \"is_gift\": false,\n      \"tier\": \"1000\"\n    }\n  ]\n}",
    module: twitch_highway::subscriptions::response::UserSubscriptionResponse,
    de: UserSubscriptionResponse
);
