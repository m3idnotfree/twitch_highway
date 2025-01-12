use twitch_highway::{
    moderation::request::UpdateAutoModSettingsRequest,
    types::{BroadcasterId, ModeratorId},
};

fn_expected_request!(
    api: twitch_highway::moderation::ModerationAPI,
    endpoint: update_auto_mod_settings,
    token_type: User,
    scopes: Some(vec![Scope::ModeratorManageAutomodSettings]),
    args: [
        BroadcasterId::new("1234"),
        ModeratorId::new("5678"),
        UpdateAutoModSettingsRequest::new()
            .overall_level(3)
    ],
    json_contain: ["\"overall_level\":3"],
    method: PUT,
    header: expected_headers!(json),
    url: "https://api.twitch.tv/helix/moderation/automod/settings?broadcaster_id=1234&moderator_id=5678"
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"broadcaster_id\": \"1234\",\n      \"moderator_id\": \"5678\",\n      \"overall_level\": 3,\n      \"disability\": 3,\n      \"aggression\": 3,\n      \"sexuality_sex_or_gender\": 3,\n      \"misogyny\": 3,\n      \"bullying\": 2,\n      \"swearing\": 0,\n      \"race_ethnicity_or_religion\": 3,\n      \"sex_based_terms\": 3\n    }\n  ]\n}",
    module: twitch_highway::moderation::response::AutoModSettingsResponse,
    de: AutoModSettingsResponse
);

fn_expected_resopnse!(
    name: overall_elve,
    payload: "{\n  \"data\": [\n    {\n      \"broadcaster_id\": \"1234\",\n      \"moderator_id\": \"5678\",\n      \"overall_level\": null,\n      \"disability\": 0,\n      \"aggression\": 0,\n      \"sexuality_sex_or_gender\": 0,\n      \"misogyny\": 0,\n      \"bullying\": 0,\n      \"swearing\": 2,\n      \"race_ethnicity_or_religion\": 0,\n      \"sex_based_terms\": 0\n    }\n  ]\n}",
    module: twitch_highway::moderation::response::AutoModSettingsResponse,
    de: AutoModSettingsResponse
);
