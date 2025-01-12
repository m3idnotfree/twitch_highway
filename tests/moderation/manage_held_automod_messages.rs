use twitch_highway::{
    moderation::request::{AutoModAction, ManageHeldAutoModMeussageRequest},
    types::UserId,
};

fn_expected_request!(
    api: twitch_highway::moderation::ModerationAPI,
    endpoint: manage_held_automod_messages,
    token_type: User,
    scopes: Some(vec![Scope::ModeratorManageAutomod]),
    args: [
        ManageHeldAutoModMeussageRequest::new(
            UserId::new("9327994"),
            "836013710".to_string(),
            AutoModAction::ALLOW
        )
    ],
    json_contain: ["\"user_id\":\"9327994\"","\"msg_id\":\"836013710\"","\"action\":\"ALLOW\""],
    method: POST,
    header: expected_headers!(json),
    url: "https://api.twitch.tv/helix/moderation/automod/message"
);
