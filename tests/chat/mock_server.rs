//fn_mock_server!(
//    token_type: user_token,
//    name: get_chatters,
//    endpoint: get_chatters,
//    scopes: twitch_highway::EndpointType::GetChatters.required_scopes().unwrap_or_default(),
//    args: |user_id| {&user_id, &user_id, None, None},
//    url: TestUrlHold::chat_url(None, Some(&["chatters"])),
//    response: twitch_highway::chat::response::ChattersResponse
//);

use twitch_highway::{
    chat::request::UpdateChatSettingsRequest,
    types::{BroadcasterId, ModeratorId, UserId},
};

fn_mock_server!(
    token_type: user_token,
    name: channel_emotes,
    endpoint: channel_emotes,
    scopes: twitch_highway::EndpointType::GetChannelEmotes.required_scopes().unwrap_or_default(),
    args: |user_id| {BroadcasterId::new(user_id)},
    url: TestUrlHold::chat_url(None, Some(&["emotes"])),
    response: twitch_highway::chat::response::EmotesResponse
);

fn_mock_server!(
    token_type: user_token,
    name: global_emotes,
    endpoint: global_emotes,
    scopes: twitch_highway::EndpointType::GetGlobalEmotes.required_scopes().unwrap_or_default(),
    args: |_user_id| {},
    url: TestUrlHold::chat_url(None, Some(&["emotes","global"])),
    response: twitch_highway::chat::response::EmotesResponse
);

fn_mock_server!(
    token_type: user_token,
    name: emote_sets,
    endpoint: emote_sets,
    scopes: twitch_highway::EndpointType::GetEmoteSets.required_scopes().unwrap_or_default(),
    args: |user_id| {[user_id.as_str()]},
    url: TestUrlHold::chat_url(None, Some(&["emotes","set"])),
    response: twitch_highway::chat::response::EmotesResponse
);

fn_mock_server!(
    token_type: user_token,
    name: channel_badge,
    endpoint: channel_badge,
    scopes: twitch_highway::EndpointType::GetChannelChatBadges.required_scopes().unwrap_or_default(),
    args: |user_id| {BroadcasterId::new(user_id)},
    url: TestUrlHold::chat_url(None, Some(&["badges"])),
    response: twitch_highway::chat::response::BadgesResponse
);

fn_mock_server!(
    token_type: user_token,
    name: global_badge,
    endpoint: global_badge,
    scopes: twitch_highway::EndpointType::GetGlobalChatBadges.required_scopes().unwrap_or_default(),
    args: |_user_id| {},
    url: TestUrlHold::chat_url(None, Some(&["badges","global"])),
    response: twitch_highway::chat::response::BadgesResponse
);

fn_mock_server!(
    token_type: user_token,
    name: get_chat_setting,
    endpoint: get_chat_settings,
    scopes: twitch_highway::EndpointType::GetChatSettings.required_scopes().unwrap_or_default(),
    args: |user_id| {BroadcasterId::new(user_id), None},
    url: TestUrlHold::chat_url(None, Some(&["settings"])),
    response: twitch_highway::chat::response::ChatSettingResponse
);

//fn_mock_server!( // 404
//    token_type: user_token,
//    name: get_shared_chat_session,
//    endpoint: get_shared_chat_session,
//    scopes: twitch_highway::EndpointType::GetShardChatSession.required_scopes().unwrap_or_default(),
//    args: |user_id| {&user_id},
//    url: TestUrlHold::base_url(None, Some(&["shared_chat","session"])),
//    response: twitch_highway::chat::response::SharedChatSessionResponse
//);

//fn_mock_server!( // missing a access token
//    token_type: user_token,
//    name: user_emotes,
//    endpoint: user_emotes,
//    scopes: twitch_highway::EndpointType::GetUserEmotes.required_scopes().unwrap_or_default(),
//    args: |user_id| {user_id.as_str(), None, None},
//    url: TestUrlHold::chat_url(None, Some(&["emotes","user"])),
//    response: twitch_highway::chat::response::EmotesResponse
//);

//fn_mock_server!( // missing a access token
//    token_type: user_token,
//    name: chat_write,
//    endpoint: chat_write,
//    scopes: twitch_highway::EndpointType::SendChatMessage.required_scopes().unwrap_or_default(),
//    args: |user_id| {&user_id,&user_id,"Hello"},
//    url: TestUrlHold::chat_url(None, Some(&["messages",])),
//    response: twitch_highway::chat::response::SendChatMessageResponse
//);

fn_mock_server!(
    token_type: user_token,
    name: update_chat_settings,
    endpoint: update_chat_settings,
    scopes: twitch_highway::EndpointType::UpdateChatSettings.required_scopes().unwrap_or_default(),
    args: |user_id| {
        BroadcasterId::new(user_id.clone()),
        ModeratorId::new(user_id),
        UpdateChatSettingsRequest::new().follower_mode(false)
    },
    url: TestUrlHold::chat_url(None, Some(&["settings",])),
    response: twitch_highway::chat::response::ChatSettingResponse
);

fn_mock_server!(
    token_type: user_token,
    name: user_chat_color,
    endpoint: user_chat_color,
    scopes: twitch_highway::EndpointType::GetUserChatColor.required_scopes().unwrap_or_default(),
    args: |user_id| {[UserId::new(user_id)]},
    url: TestUrlHold::chat_url(None, Some(&["color"])),
    response: twitch_highway::chat::response::UsersColorResponse
);

fn_mock_server!(
    token_type: user_token,
    name: update_user_chat_color,
    endpoint: update_user_chat_color,
    scopes: twitch_highway::EndpointType::UpdateUserChatColor.required_scopes().unwrap_or_default(),
    args: |user_id| {
        UserId::new(user_id),
        twitch_highway::chat::request::ChatColor::Blue
    },
    url: TestUrlHold::chat_url(None, Some(&["color"]))
);
