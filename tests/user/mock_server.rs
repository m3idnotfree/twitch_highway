use twitch_highway::types::{BroadcasterId, Id};

fn_mock_server!(
    token_type: user_token,
    name: users_info,
    endpoint: users_info,
    scopes: twitch_highway::EndpointType::GetUsers.required_scopes().unwrap_or_default(),
    args: |user_id| {Some(&[Id::new(user_id)]), None},
    url: TestUrlHold::users_url(None, None),
    status: OK,
    response: twitch_highway::users::response::UsersInfoResponse
);

fn_mock_server!(
    token_type: user_token,
    name: update_user,
    endpoint: update_user,
    scopes: twitch_highway::EndpointType::UpdateUser.required_scopes().unwrap_or_default(),
    args: |_user_id| {Some(" ")},
    url: TestUrlHold::users_url(None, None),
    status: OK,
    response: twitch_highway::users::response::UpdateUsersResponse
);

fn_mock_server!(
    token_type: user_token,
    name: block_list,
    endpoint: block_list,
    scopes: twitch_highway::EndpointType::GetUserBlockList.required_scopes().unwrap_or_default(),
    args: |user_id| {BroadcasterId::new(user_id), None, None},
    url: TestUrlHold::users_url(None, Some(&["blocks"])),
    response: twitch_highway::users::response::BlockUserListResponse
);

fn_mock_server!(
    token_type: user_token,
    name: block_user,
    endpoint: block_user,
    scopes: twitch_highway::EndpointType::BlockUser.required_scopes().unwrap_or_default(),
    args: |user_id| {&user_id, None, None},
    url: TestUrlHold::users_url(None, Some(&["blocks"])),
    status: BAD_REQUEST
);

fn_mock_server!(
    token_type: user_token,
    name: unblock_user,
    endpoint: unblock_user,
    scopes: twitch_highway::EndpointType::UnblockUser.required_scopes().unwrap_or_default(),
    args: |user_id| {&user_id},
    url: TestUrlHold::users_url(None, Some(&["blocks"])),
    status: NO_CONTENT
);

//fn_mock_server!(
//    token_type: user_token,
//    name: user_extensions,
//    endpoint: user_extensions,
//    scopes: twitch_highway::EndpointType::GetUserExtensions.required_scopes().unwrap_or_default(),
//    args: |_user_id| {},
//    url: TestUrlHold::users_url(None, Some(&["extensions","list"])),
//    response: twitch_highway::users::response::UserExtensionsResponse
//);

//fn_mock_server!(
//    token_type: user_token,
//    name: user_active_extensions,
//    endpoint: user_active_extensions,
//    scopes: twitch_highway::EndpointType::GetUserExtensions.required_scopes().unwrap_or_default(),
//    args: |user_id| {Some(&user_id)},
//    url: TestUrlHold::users_url(None, Some(&["extensions"])),
//    status: OK,
//    response: twitch_highway::users::response::UserExtensionsResponse
//);
