fn_expected_request!(
    name: channel_raid,
    api:twitch_highway::eventsub::EventSubAPI,
    endpoint: channel_raid_as_webhook,
    token_type: App,
    scopes: None,
    args: ["https://example.com/webhooks/callback", Some("s3cRe7"), None, Some("1337")],
    method: POST,
    header: expected_headers!(json),
    url: "https://api.twitch.tv/helix/eventsub/subscriptions",
    json: Some("{\"type\":\"channel.raid\",\"version\":\"1\",\"condition\":{\"to_broadcaster_user_id\":\"1337\"},\"transport\":{\"method\":\"webhook\",\"callback\":\"https://example.com/webhooks/callback\",\"secret\":\"s3cRe7\"}}".to_string())
);
