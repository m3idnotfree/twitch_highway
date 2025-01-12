use twitch_highway::{
    channel_points::{request::UpdateRedemptionStatusRequest, types::RedemptionStatus},
    types::{BroadcasterId, Id},
};

fn_expected_request!(
    api:twitch_highway::channel_points::ChannelPointsAPI,
    endpoint: update_redemption_status,
    token_type: User,
    scopes: Some(vec![Scope::ChannelManageRedemptions]),
    args: [
        BroadcasterId::new("274637212"),
        [Id::new("17fa2df1-ad76-4804-bfa5-a40ef63efe63")],
        "92af127c-7326-4483-a52b-b0da0be61c01",
        UpdateRedemptionStatusRequest::new(RedemptionStatus::CANCELED)
    ],
    json_contain: ["\"status\":\"CANCELED\""],
    method: PATCH,
    header: expected_headers!(json),
    url: "https://api.twitch.tv/helix/channel_points/custom_rewards/redemptions?broadcaster_id=274637212&reward_id=92af127c-7326-4483-a52b-b0da0be61c01&id=17fa2df1-ad76-4804-bfa5-a40ef63efe63"
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"broadcaster_name\": \"torpedo09\",\n      \"broadcaster_login\": \"torpedo09\",\n      \"broadcaster_id\": \"274637212\",\n      \"id\": \"17fa2df1-ad76-4804-bfa5-a40ef63efe63\",\n      \"user_id\": \"274637212\",\n      \"user_name\": \"torpedo09\",\n      \"user_login\": \"torpedo09\",\n      \"user_input\": \"\",\n      \"status\": \"CANCELED\",\n      \"redeemed_at\": \"2020-07-01T18:37:32Z\",\n      \"reward\": {\n        \"id\": \"92af127c-7326-4483-a52b-b0da0be61c01\",\n        \"title\": \"game analysis\",\n        \"prompt\": \"\",\n        \"cost\": 50000\n      }\n    }\n  ]\n}",
    module:twitch_highway::channel_points::response::CustomRewardsRedemptionResponse,
    de: CustomRewardsRedemptionResponse
);
