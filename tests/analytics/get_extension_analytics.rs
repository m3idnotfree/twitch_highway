fn_expected_request!(
    modules: [
        twitch_highway::analytics::AnalyticsAPI,
        twitch_oauth_token::types::Scope
    ],
    endpoint: get_extension_analytics,
    token_type: User,
    scopes: Some(vec![Scope::AnalyticsReadExtensions]),
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/analytics/extensions",
    json: None,
    text: None,
    urlencoded: None
);

fn_expected_resopnse!(
    payload: "{\n   \"data\": [\n      {\n         \"extension_id\": \"efgh\",\n         \"URL\": \"https://twitch-piper-reports.s3-us-west-2.amazonaws.com/dynamic/LoL%20ADC...\",\n         \"type\": \"overview_v2\",\n         \"date_range\": {\n            \"started_at\": \"2018-03-01T00:00:00Z\",\n            \"ended_at\": \"2018-06-01T00:00:00Z\"\n         }\n      }\n   ],\n   \"pagination\": {\"cursor\": \"eyJiIjpudWxsLCJhIjp7Ik9mZnNldCI6NX19\"}\n}",
    module: twitch_highway::analytics::response::ExtensionAnalyticsResponse,
    de: ExtensionAnalyticsResponse
);
