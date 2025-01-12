use twitch_highway::types::BroadcasterId;

fn_expected_request!(
    api: twitch_highway::moderation::ModerationAPI,
    endpoint: get_banned_users,
    token_type: User,
    scopes: Some(vec![Scope::ModerationRead]),
    args: [BroadcasterId::new("198704263"), None, None, None, None],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/moderation/banned?broadcaster_id=198704263"
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"user_id\": \"423374343\",\n      \"user_login\": \"glowillig\",\n      \"user_name\": \"glowillig\",\n      \"expires_at\": \"2022-03-15T02:00:28Z\",\n      \"created_at\": \"2022-03-15T01:30:28Z\",\n      \"reason\": \"Does not like pineapple on pizza.\",\n      \"moderator_id\": \"141981764\",\n      \"moderator_login\": \"twitchdev\",\n      \"moderator_name\": \"TwitchDev\"\n    },\n    {\n      \"user_id\": \"424596340\",\n      \"user_login\": \"quotrok\",\n      \"user_name\": \"quotrok\",\n      \"expires_at\": \"2022-08-07T02:07:55Z\",\n      \"created_at\": \"2022-08-07T02:02:55Z\",\n      \"reason\": \"Inappropriate words.\",\n      \"moderator_id\": \"141981764\",\n      \"moderator_login\": \"twitchdev\",\n      \"moderator_name\": \"TwitchDev\"\n    }\n  ],\n  \"pagination\": {\n    \"cursor\": \"eyJiIjpudWxsLCJhIjp7IkN1cnNvciI6IjEwMDQ3MzA2NDo4NjQwNjU3MToxSVZCVDFKMnY5M1BTOXh3d1E0dUdXMkJOMFcifX0\"\n  }\n}",
    module: twitch_highway::moderation::response::GetBannedUsersResponse,
    de: GetBannedUsersResponse
);
