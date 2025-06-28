fn_expected_request!(
    modules: [
        twitch_highway::moderation::ModerationAPI,
        twitch_highway::moderation::request::BanUserRequest,
        twitch_highway::types::BroadcasterId,
        twitch_highway::types::ModeratorId,
        twitch_highway::types::UserId,
        twitch_oauth_token::types::Scope
    ],
    endpoint: ban_users,
    token_type: User,
    scopes: Some(vec![Scope::ModeratorManageBannedUsers]),
    args: [
        BroadcasterId::new("1234"),
        ModeratorId::new("5678"),
        BanUserRequest::new(UserId::new("9876"))
            .reason("no reason")
    ],
    json_contain: ["\"user_id\":\"9876\"","\"reason\":\"no reason\""],
    method: POST,
    header: expected_headers!(json),
    url: "https://api.twitch.tv/helix/moderation/bans?broadcaster_id=1234&moderator_id=5678"
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"broadcaster_id\": \"1234\",\n      \"moderator_id\": \"5678\",\n      \"user_id\": \"9876\",\n      \"created_at\": \"2021-09-28T18:22:31Z\",\n      \"end_time\": null\n    }\n  ]\n}",
    module: twitch_highway::moderation::response::BanUsersResponse,
    de: BanUsersResponse
);

fn_expected_request!(
    name: second_request,
    modules: [
        twitch_highway::moderation::ModerationAPI,
        twitch_highway::moderation::request::BanUserRequest,
        twitch_highway::types::BroadcasterId,
        twitch_highway::types::ModeratorId,
        twitch_highway::types::UserId,
        twitch_oauth_token::types::Scope
    ],
    endpoint: ban_users,
    token_type: User,
    scopes: Some(vec![Scope::ModeratorManageBannedUsers]),
    args: [
        BroadcasterId::new("1234"),
        ModeratorId::new("5678"),
        BanUserRequest::new(UserId::new("9876"))
            .duration(300)
            .reason("no reason")
    ],
    json_contain: [
        //"{\"data\":{\"user_id\":\"9876\",\"reason\":\"no reason\"}}",
        "{\"data\":{",
        "\"user_id\":\"9876\"",
        "\"reason\":\"no reason\"",
        "\"duration\":300"
    ],
    method: POST,
    header: expected_headers!(json),
    url: "https://api.twitch.tv/helix/moderation/bans?broadcaster_id=1234&moderator_id=5678"
);

fn_expected_resopnse!(
    name: second_response,
    payload: "{\n  \"data\": [\n    {\n      \"broadcaster_id\": \"1234\",\n      \"moderator_id\": \"5678\",\n      \"user_id\": \"9876\",\n      \"created_at\": \"2021-09-28T19:27:31Z\",\n      \"end_time\": \"2021-09-28T19:22:31Z\"\n    }\n  ]\n}",
    module: twitch_highway::moderation::response::BanUsersResponse,
    de: BanUsersResponse
);
