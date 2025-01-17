fn_expected_request!(
    modules: [
        twitch_highway::entitlements::EntitlementsAPI,
        twitch_highway::entitlements::request::DropEntitlementRequest,
        twitch_highway::types::UserId,
        twitch_highway::types::GameId
    ],
    endpoint: get_drops_entitlements,
    token_type: Any,
    scopes: None,
    args: [
        Some(
            DropEntitlementRequest::new()
                .user_id(UserId::new("25009227"))
                .game_id(GameId::new("33214"))
        ),
        None
    ],

    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/entitlements/drops?user_id=25009227&game_id=33214"
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"id\": \"fb78259e-fb81-4d1b-8333-34a06ffc24c0\",\n      \"benefit_id\": \"74c52265-e214-48a6-91b9-23b6014e8041\",\n      \"timestamp\": \"2019-01-28T04:17:53.325Z\",\n      \"user_id\": \"25009227\",\n      \"game_id\": \"33214\",\n      \"fulfillment_status\": \"CLAIMED\",\n      \"last_updated\": \"2019-01-28T04:17:53.325Z\"\n    },\n    {\n      \"id\": \"862750a5-265e-4ab6-9f0a-c64df3d54dd0\",\n      \"benefit_id\": \"74c52265-e214-48a6-91b9-23b6014e8041\",\n      \"timestamp\": \"2019-01-28T04:16:53.325Z\",\n      \"user_id\": \"25009227\",\n      \"game_id\": \"33214\",\n      \"fulfillment_status\": \"CLAIMED\",\n      \"last_updated\": \"2021-06-15T04:16:53.325Z\"\n    },\n    {\n      \"id\": \"d8879baa-3966-4d10-8856-15fdd62cce02\",\n      \"benefit_id\": \"cdfdc5c3-65a2-43bc-8767-fde06eb4ab2c\",\n      \"timestamp\": \"2019-01-28T04:15:53.325Z\",\n      \"user_id\": \"25009227\",\n      \"game_id\": \"33214\",\n      \"fulfillment_status\": \"FULFILLED\",\n      \"last_updated\": \"2019-01-28T04:17:53.325Z\"\n    }\n  ],\n  \"pagination\": {\n    \"cursor\": \"eyJiIjpudW...\"\n  }\n}",
    module: twitch_highway::entitlements::response::DropsEntitlementsResponse,
    de :DropsEntitlementsResponse
);
