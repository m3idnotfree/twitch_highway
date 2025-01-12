use twitch_highway::types::{BroadcasterId, ModeratorId};

fn_expected_request!(
    api: twitch_highway::moderation::ModerationAPI,
    endpoint: get_auto_mod_settings,
    token_type: User,
    scopes: Some(vec![Scope::ModeratorReadAutomodSettings]),
    args: [BroadcasterId::new("1234"), ModeratorId::new("5678")],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/moderation/automod/settings?broadcaster_id=1234&moderator_id=5678"
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"broadcaster_id\": \"1234\",\n      \"moderator_id\": \"5678\",\n      \"overall_level\": null,\n      \"disability\": 0,\n      \"aggression\": 0,\n      \"sexuality_sex_or_gender\": 0,\n      \"misogyny\": 0,\n      \"bullying\": 0,\n      \"swearing\": 0,\n      \"race_ethnicity_or_religion\": 0,\n      \"sex_based_terms\": 0\n    }\n  ]\n}",
    module: twitch_highway::moderation::response::AutoModSettingsResponse,
    de: AutoModSettingsResponse
);
