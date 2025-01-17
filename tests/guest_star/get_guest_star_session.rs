fn_expected_request!(
    modules: [
        twitch_highway::guest_star::GuestStarAPI,
        twitch_highway::types::BroadcasterId ,
        twitch_highway::types::ModeratorId,
        twitch_oauth_token::types::Scope
    ],
    endpoint: get_guest_star_session,
    token_type: User,
    scopes: Some(vec![
        Scope::ChannelReadGuestStar,
        Scope::ChannelManageGuestStar,
        Scope::ModeratorReadGuestStar
    ]),
    args: [BroadcasterId::new("9321049"), ModeratorId::new("9321049")],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/guest_star/session?broadcaster_id=9321049&moderator_id=9321049"
);

fn_expected_resopnse!(
    payload: "{\n    \"data\": [\n        {\n            \"id\": \"2KFRQbFtpmfyD3IevNRnCzOPRJI\",\n            \"guests\": [\n                {\n                    \"slot_id\": \"0\",\n                    \"user_id\": \"9321049\",\n                    \"user_display_name\": \"Cool_User\",\n                    \"user_login\": \"cool_user\",\n                    \"is_live\": true,\n                    \"volume\": 100,\n                    \"assigned_at\": \"2023-01-02T04:16:53.325Z\",\n                    \"audio_settings\": {\n                        \"is_available\": true,\n                        \"is_host_enabled\": true,\n                        \"is_guest_enabled\": true\n                    },\n                    \"video_settings\": {\n                        \"is_available\": true,\n                        \"is_host_enabled\": true,\n                        \"is_guest_enabled\": true\n                    }\n                },\n                {\n                    \"slot_id\": \"1\",\n                    \"user_id\": \"144601104\",\n                    \"user_display_name\": \"Cool_Guest\",\n                    \"user_login\": \"cool_guest\",\n                    \"is_live\": true,\n                    \"volume\": 100,\n                    \"assigned_at\": \"2023-01-02T04:20:59.325Z\",\n                    \"audio_settings\": {\n                        \"is_available\": true,\n                        \"is_host_enabled\": true,\n                        \"is_guest_enabled\": true\n                    },\n                    \"video_settings\": {\n                        \"is_available\": true,\n                        \"is_host_enabled\": true,\n                        \"is_guest_enabled\": true\n                    }\n                }\n            ]\n        }\n    ]\n}",
    module: twitch_highway::guest_star::response::GustStarSessionResponse,
    de: GustStarSessionResponse
);
