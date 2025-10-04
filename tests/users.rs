#![cfg(feature = "users")]

#[macro_use]
mod common;

use std::collections::HashMap;

use anyhow::Result;
use common::{mock_api_start, TwitchFixture};
use twitch_highway::{
    types::{BroadcasterId, Id, UserId},
    users::{Component, Overlay, Panel, UserAPI, UserActiveExtensions},
};
use twitch_oauth_token::scope::UserScopes;

api_test!(get_users, [Some(&[Id::from("141981764")]), None]);
api_test!(update_user, ["BaldAngel"]);
api_test!(
    get_user_block_list,
    [&BroadcasterId::from("141981764"), None]
);
api_test!(block_user, [&UserId::from("198704263"), None, None]);
api_test!(unblock_user, [&UserId::from("198704263")]);
api_test!(get_user_extensions, []);
api_test!(get_user_active_extensions, [None]);
api_test!(
    update_user_extensions,
    [{
        let mut panels = HashMap::new();
        panels.insert(
            "1".to_string(),
            Panel::new(true)
                .id(Id::from("rh6jq1q334hqc2rr1qlzqbvwlfl3x0"))
                .version("1.1.8".to_string()),
        );

        panels.insert(
            "2".to_string(),
            Panel::new(true)
                .id(Id::from("wi08ebtatdc7oj83wtl9uxwz807l8b"))
                .version("1.1.8"),
        );

        panels.insert(
            "2".to_string(),
            Panel::new(true)
                .id(Id::from("naty2zwfp7vecaivuve8ef1hohh6bo"))
                .version("1.1.8"),
        );

        let mut overlays = HashMap::new();
        overlays.insert(
            "1".to_string(),
            Overlay::new(true)
                .id(Id::from("zfh2irvx2jb4s60f02jq0ajm8vwgka"))
                .version("1.0.19"),
        );

        let mut components = HashMap::new();
        components.insert(
            "1".to_string(),
            Component::new(true)
                .id(Id::from("lqnf3zxk0rv0g7gq92mtmnirjz2cjj"))
                .version("0.0.1")
                .name("Dev Experience Test")
                .x(0)
                .y(0),
        );

        components.insert("2".to_string(), Component::new(false));

        UserActiveExtensions::new(panels, overlays, components)
    }]
);

#[tokio::test]
async fn mock_api() -> Result<()> {
    let _cmd = mock_api_start().await?;

    let api = TwitchFixture::user_access_token(|scope| {
        scope.users_api();
    })
    .await?;

    mock_api_get_users(&api).await?;
    mock_api_update_user(&api).await?;
    mock_api_get_user_block_list(&api).await?;
    mock_api_block_user(&api).await?;
    mock_api_unblock_user(&api).await?;

    // mock_api_get_user_extensions(&api).await?;
    // mock_api_get_user_active_extensions(&api).await?;

    Ok(())
}

async fn mock_api_get_users(api: &TwitchFixture) -> Result<()> {
    api.api.get_users(None, None).json().await?;
    Ok(())
}
async fn mock_api_update_user(api: &TwitchFixture) -> Result<()> {
    api.api.update_user("ffs").json().await?;
    Ok(())
}
async fn mock_api_get_user_block_list(api: &TwitchFixture) -> Result<()> {
    api.api
        .get_user_block_list(&api.selected_broadcaster_id(), None)
        .json()
        .await?;
    Ok(())
}
async fn mock_api_block_user(api: &TwitchFixture) -> Result<()> {
    let random_user = api.get_random_user()?;
    let user_id = UserId::from(random_user.id);
    api.api.block_user(&user_id, None, None).json().await?;
    Ok(())
}
async fn mock_api_unblock_user(api: &TwitchFixture) -> Result<()> {
    api.api.unblock_user(&api.selected_user_id()).json().await?;
    Ok(())
}
// async fn mock_api_get_user_extensions(api: &TwitchFixture) -> Result<()> {
//     api.api.get_user_extensions().json().await?;
//     Ok(())
// }
// async fn mock_api_get_user_active_extensions(api: &TwitchFixture) -> Result<()> {
//     api.api.get_user_active_extensions(None).json().await?;
//     Ok(())
// }
// async fn mock_api_update_user_extensions(api: &TwitchFixture) -> Result<()> {
//     api.api.get_user_active_extensions(None).json().await?;
//     Ok(())
// }
