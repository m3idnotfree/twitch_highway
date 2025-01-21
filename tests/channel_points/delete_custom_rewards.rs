fn_expected_request!(
    modules: [
        twitch_highway::channel_points::ChannelPointsAPI,
        twitch_highway::types::BroadcasterId,
        twitch_highway::types::CustomRewardId,
        twitch_oauth_token::types::Scope
    ],
    endpoint: delete_custom_rewards,
    token_type: User,
    scopes: Some(vec![Scope::ChannelManageRedemptions]),
    args: [
        BroadcasterId::new("274637212"),
        CustomRewardId::new("b045196d-9ce7-4a27-a9b9-279ed341ab28")
    ],
    method: DELETE,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/channel_points/custom_rewards?broadcaster_id=274637212&id=b045196d-9ce7-4a27-a9b9-279ed341ab28"
);
