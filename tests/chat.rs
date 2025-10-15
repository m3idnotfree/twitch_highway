#![cfg(feature = "chat")]

#[macro_use]
mod common;

use anyhow::Result;
use common::{mock_api_start, TwitchFixture};
use twitch_highway::{
    chat::{AnnouncementColor, ChatAPI, ChatColor},
    types::{BroadcasterId, ModeratorId, UserId},
};
use twitch_oauth_token::scope::ChatScopes;

api_test!(build
    get_chatters |api| {
        api.get_chatters(&BroadcasterId::from("123456"), &ModeratorId::from("654321"))
    }
);
api_test!(get_channel_emotes, [&BroadcasterId::from("141981764")]);
api_test!(get_global_emotes, []);
api_test!(get_emote_sets, [&["301590448"]]);
api_test!(get_channel_chat_badges, [&BroadcasterId::from("135093069")]);
api_test!(get_global_chat_badges, []);
api_test!(build
    get_chat_settings |api| {
        api.get_chat_settings(&BroadcasterId::from("1234"))
    }
);
api_test!(get_shared_chat_session, [&BroadcasterId::from("198704263")]);
api_test!(build
    get_user_emotes |api| {
        api.get_user_emotes(&UserId::from("123456"))
    }
);
api_test!(build
    update_chat_settings |api| {
        api.update_chat_settings(&BroadcasterId::from("1234"), &ModeratorId::from("5678"))
            .follower_mode(false)
    }
);
api_test!(build
    send_chat_announcement |api| {
        api.send_chat_announcement(
            &BroadcasterId::from("11111"),
            &ModeratorId::from("44444"),
            "Hello chat!",
        )
        .color(AnnouncementColor::Purple)
    }
);
api_test!(
    send_a_shoutout,
    [
        &BroadcasterId::from("12345"),
        &BroadcasterId::from("626262"),
        &ModeratorId::from("98765"),
    ]
);
api_test!(
    build
    send_chat_message |api| {
        api.send_chat_message(
            &BroadcasterId::from("12826"),
            &UserId::from("141981764"),
            "Hello, world! twitchdevHype",
        )
        .for_source_only(true)
    }
);
api_test!(
    get_user_chat_color,
    [&[UserId::from("11111"), UserId::from("44444")]]
);
api_test!(
    update_user_chat_color,
    [&UserId::from("123"), ChatColor::Blue,]
);
api_test!(build_extra
    update_chat_settings,
    update_chat_settings2 |api| {
        api.update_chat_settings(&BroadcasterId::from("1234"), &ModeratorId::from("5678"))
            .slow_mode(true)
            .slow_mode_wait_time(10)
    }
);

// api_test!(extra
//     update_user_chat_color,
//     update_user_chat_color2,
//     [&UserId::from("123"), ChatColor::Blue,]
// );

#[tokio::test]
async fn mock_api() -> Result<()> {
    let _cmd = mock_api_start().await?;

    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    let api = TwitchFixture::user_access_token(|scope| {
        scope
            .get_chatters()
            .get_chat_settings()
            .update_chat_settings()
            .update_user_chat_color();
    })
    .await?;

    mock_api_get_chatters(&api).await?;
    mock_api_get_channel_emotes(&api).await?;
    mock_api_get_global_emotes(&api).await?;

    // mock_api_get_emote_sets(&api).await?; // missing required emote_set_id

    mock_api_get_channel_chat_badges(&api).await?;
    mock_api_get_global_chat_badges(&api).await?;
    mock_api_get_chat_settings(&api).await?;

    // mock_api_get_shared_chat_session(&api).await?; // page not found
    // mock_api_get_user_emotes(&api).await?; //page not found

    mock_api_update_chat_settings(&api).await?;

    // mock_api_send_chat_announcement(&api).await?; // Method not allowed
    // mock_api_send_chat_message(&api).await?; // page not found

    mock_api_get_user_chat_color(&api).await?;
    mock_api_update_user_chat_color(&api).await?;

    Ok(())
}

async fn mock_api_get_chatters(api: &TwitchFixture) -> Result<()> {
    api.api
        .get_chatters(&api.selected_broadcaster_id(), &api.selected_moderator_id())
        .json()
        .await?;
    Ok(())
}
async fn mock_api_get_channel_emotes(api: &TwitchFixture) -> Result<()> {
    api.api
        .get_channel_emotes(&api.selected_broadcaster_id())
        .json()
        .await?;
    Ok(())
}
async fn mock_api_get_global_emotes(api: &TwitchFixture) -> Result<()> {
    api.api.get_global_emotes().json().await?;
    Ok(())
}
// async fn mock_api_get_emote_sets(api: &TwitchFixture) -> Result<()> {
//     api.api.get_emote_sets(&[""]).json().await?;
//     Ok(())
// }
async fn mock_api_get_channel_chat_badges(api: &TwitchFixture) -> Result<()> {
    api.api
        .get_channel_chat_badges(&api.selected_broadcaster_id())
        .json()
        .await?;
    Ok(())
}
async fn mock_api_get_global_chat_badges(api: &TwitchFixture) -> Result<()> {
    api.api.get_global_chat_badges().json().await?;
    Ok(())
}
async fn mock_api_get_chat_settings(api: &TwitchFixture) -> Result<()> {
    api.api
        .get_chat_settings(&api.selected_broadcaster_id())
        .json()
        .await?;
    Ok(())
}
// async fn mock_api_get_shared_chat_session(api: &TwitchFixture) -> Result<()> {
//     api.api
//         .get_shared_chat_session(&api.selected_broadcaster_id())
//         .json()
//         .await?;
//     Ok(())
// }
// async fn mock_api_get_user_emotes(api: &TwitchFixture) -> Result<()> {
//     api.api
//         .get_user_emotes(&api.selected_user_id())
//         .json()
//         .await?;
//     Ok(())
// }

async fn mock_api_update_chat_settings(api: &TwitchFixture) -> Result<()> {
    api.api
        .update_chat_settings(&api.selected_broadcaster_id(), &api.selected_moderator_id())
        .follower_mode(false)
        .json()
        .await?;
    Ok(())
}

// async fn mock_api_send_chat_announcement(api: &TwitchFixture) -> Result<()> {
//     api.api
//         .send_chat_announcement(
//             &api.selected_broadcaster_id(),
//             &api.selected_moderator_id(),
//             "message",
//         )
//         .json()
//         .await?;
//     Ok(())
// }
// async fn mock_api_send_a_shoutout(api: &TwitchFixture) -> Result<()> {
//     api.api
//         .send_a_shoutout(
//             &api.selected_broadcaster_id(),
//             &api.selected_broadcaster_id(),
//             &api.selected_moderator_id(),
//         )
//         .json()
//         .await?;
//     Ok(())
// }
// async fn mock_api_send_chat_message(api: &TwitchFixture) -> Result<()> {
//     api.api
//         .send_chat_message(&api.selected_broadcaster_id(), &UserId::from(""), "")
//         .json()
//         .await?;
//     Ok(())
// }

async fn mock_api_get_user_chat_color(api: &TwitchFixture) -> Result<()> {
    api.api
        .get_user_chat_color(&[api.selected_user_id()])
        .json()
        .await?;
    Ok(())
}
async fn mock_api_update_user_chat_color(api: &TwitchFixture) -> Result<()> {
    api.api
        .update_user_chat_color(&api.selected_user_id(), ChatColor::Green)
        .json()
        .await?;
    Ok(())
}
