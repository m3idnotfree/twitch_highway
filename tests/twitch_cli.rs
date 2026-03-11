#[macro_use]
mod common;

use anyhow::Result;
use common::{mock_api_start, TwitchFixture};
use std::time::Duration;
use tokio::time::sleep;
use twitch_highway::types::{BroadcasterId, PredictionId, RewardId, TeamId, Title, UserId};
use twitch_highway::{
    bits::BitsAPI,
    ccls::CclsAPI,
    channel_points::{ChannelPointsAPI, CustomReward},
    channels::ChannelsAPI,
    charity::CharityAPI,
    chat::{ChatAPI, ChatColor},
    clips::ClipsAPI,
    goals::GoalsAPI,
    polls::PollsAPI,
    predictions::{PredictionStatus, PredictionsAPI},
    raid::RaidAPI,
    search::SearchAPI,
    subscriptions::SubscriptionsAPI,
    teams::TeamsAPI,
    users::UserAPI,
    videos::VideosAPI,
    whisper::WhisperAPI,
};

#[tokio::test]
#[ignore]
async fn mock_api() -> Result<()> {
    let _cmd = mock_api_start().await?;

    sleep(Duration::from_secs(2)).await;

    let api = TwitchFixture::user_access_token(|scope| {
        scope.bits_api();
    })
    .await?;
    mock_api_get_bits_leaderboard(&api).await?;
    mock_api_get_cheermotes(&api).await?;

    let api = TwitchFixture::user_access_token(|scope| {
        scope.ccl_api();
    })
    .await?;

    let resp = api
        .api
        .get_content_classification_labels(None)
        .json()
        .await?;

    assert!(!resp.data.is_empty());

    let api = TwitchFixture::user_access_token_with_partner(|scope| {
        scope.channel_points_api();
    })
    .await?;

    let _reward = mock_api_get_custom_reward(&api).await?;

    let create = mock_api_create_custom_rewards(&api, "twitch_highway").await?;

    mock_api_delete_custom_reward(&api, &create.id).await?;
    // mock_api_update_custom_reward(
    //     &api,
    //     &reward.id,
    //     UpdateCustomRewardRequest::new().title("hello"),
    // )
    // .await?;

    let api = TwitchFixture::user_access_token(|scope| {
        scope.channel_api();
    })
    .await?;

    mock_api_get_channel_info(&api).await?;
    mock_api_modify_channel_info(&api).await?;
    mock_api_get_channel_editors(&api).await?;
    mock_api_get_followed_channels(&api).await?;
    mock_api_get_channel_followers(&api).await?;

    let api = TwitchFixture::user_access_token(|scope| {
        scope.charity_api();
    })
    .await?;

    mock_api_get_charity_campaign(&api).await?;
    mock_api_get_charity_campaign_donations(&api).await?;

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

    let api = TwitchFixture::user_access_token_with_live(|scope| {
        scope.clips_api();
    })
    .await?;

    mock_api_create_clip(&api).await?;
    mock_api_get_clips(&api).await?;

    let api = TwitchFixture::user_access_token(|scope| {
        scope.goals_api();
    })
    .await?;

    mock_api_get_creator_goals(&api).await?;

    //     let api = TwitchFixture::user_access_token(|scope| {
    //         scope.hype_train_api();
    //     })
    //     .await?;
    //
    //     // mock_api_get_hype_train_events(&api).await?;
    //     // mock_api_get_hype_train_status(&api).await?; // page not found

    let api = TwitchFixture::user_access_token(|scope| {
        scope.polls_api();
    })
    .await?;

    mock_api_get_polls(&api).await?;
    mock_api_create_poll(&api).await?;

    let api = TwitchFixture::user_access_token(|scope| {
        scope.predictions_api();
    })
    .await?;

    mock_api_get_predictions(&api).await?;
    mock_api_create_prediction(&api).await?;
    mock_api_end_prediction(&api).await?;

    let api = TwitchFixture::user_access_token(|scope| {
        scope.raids_api();
    })
    .await?;

    mock_api_start_raid(&api).await?;
    mock_api_cancel_raid(&api).await?;

    let api = TwitchFixture::user_access_token(|scope| {
        scope.search_api();
    })
    .await?;

    mock_api_search_categories(&api).await?;
    mock_api_search_channels(&api).await?;

    let api = TwitchFixture::user_access_token(|scope| {
        scope.subscriptions_api();
    })
    .await?;

    mock_api_get_broadcaster_subscriptions(&api).await?;
    mock_api_check_user_subscription(&api).await?;

    let api = TwitchFixture::user_access_token(|scope| {
        scope.teams_api();
    })
    .await?;

    mock_api_get_channel_teams(&api).await?;
    mock_api_get_teams(&api).await?;

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

    let api = TwitchFixture::user_access_token(|scope| {
        scope.videos_api();
    })
    .await?;

    mock_api_get_videos(&api).await?;
    // mock_api_delete_videos(&api).await?;

    let api = TwitchFixture::user_access_token(|scope| {
        scope.whisper_api();
    })
    .await?;

    mock_api_send_whisper(&api).await?;

    Ok(())
}

async fn mock_api_get_bits_leaderboard(api: &TwitchFixture) -> Result<()> {
    let resp = api.api.get_bits_leaderboard().json().await?;
    assert!(!resp.data.is_empty());
    Ok(())
}

async fn mock_api_get_cheermotes(api: &TwitchFixture) -> Result<()> {
    let resp = api.api.get_cheermotes(None).json().await?;
    assert!(!resp.data.is_empty());
    Ok(())
}

async fn mock_api_create_custom_rewards(api: &TwitchFixture, title: &str) -> Result<CustomReward> {
    let resp = api
        .api
        .create_custom_rewards(&api.selected_broadcaster_id(), title, 6)
        .json()
        .await?;

    assert!(resp.data.is_some());
    let data = resp.data.unwrap();
    assert!(!data.is_empty());
    let first = data.first().cloned().unwrap();
    Ok(first)
}

async fn mock_api_delete_custom_reward(
    api: &TwitchFixture,
    custom_reward_id: &RewardId,
) -> Result<()> {
    api.api
        .delete_custom_reward(&api.selected_broadcaster_id(), custom_reward_id)
        .send()
        .await?;

    Ok(())
}

async fn mock_api_get_custom_reward(api: &TwitchFixture) -> Result<CustomReward> {
    let resp = api
        .api
        .get_custom_reward(&api.selected_broadcaster_id())
        .json()
        .await?;

    assert!(resp.data.is_some());
    let data = resp.data.unwrap();
    assert!(!data.is_empty());
    let first = data.first().cloned().unwrap();

    Ok(first)
}
//
// async fn mock_api_get_custom_reward_redemption(
//     api: &TwitchFixture,
//     custom_reward_id: &RewardId,
// ) -> Result<Vec<CustomRewardsRedemption>> {
//     let resp = api
//         .api
//         .get_custom_reward_redemption(&api.selected_broadcaster_id(), custom_reward_id)
//         .status(RedemptionStatus::FULFILLED)
//         .json()
//         .await?;
//
//     assert!(!resp.data.is_empty());
//
//     Ok(resp.data)
// }
//
// async fn mock_api_update_custom_reward(
//     api: &TwitchFixture,
//     custom_reward_id: &RewardId,
// ) -> Result<()> {
//     let resp = api
//         .api
//         .update_custom_reward(&api.selected_broadcaster_id(), custom_reward_id)
//         .json()
//         .await?;
//
//     assert!(resp.data.is_some());
//     let data = resp.data.unwrap();
//     assert!(!data.is_empty());
//
//     Ok(())
// }
//
// async fn mock_api_update_redemption_status(
//     api: &TwitchFixture,
//     custom_reward_id: &RewardId,
//     redemption_id: &RedemptionId,
//     status: RedemptionStatus,
// ) -> Result<()> {
//     let resp = api
//         .api
//         .update_redemption_status(
//             &api.selected_broadcaster_id(),
//             custom_reward_id,
//             std::slice::from_ref(redemption_id),
//             status,
//         )
//         .json()
//         .await?;
//
//     assert!(!resp.data.is_empty());
//
//     Ok(())
// }
//
async fn mock_api_get_channel_info(api: &TwitchFixture) -> Result<()> {
    api.api
        .get_channel_info(&[api.selected_broadcaster_id()])
        .json()
        .await?;
    Ok(())
}

async fn mock_api_modify_channel_info(api: &TwitchFixture) -> Result<()> {
    api.api
        .modify_channel_info(&api.selected_broadcaster_id())
        .broadcaster_language("en")
        .json()
        .await?;
    Ok(())
}

async fn mock_api_get_channel_editors(api: &TwitchFixture) -> Result<()> {
    api.api
        .get_channel_editor(&api.selected_broadcaster_id())
        .json()
        .await?;
    Ok(())
}

async fn mock_api_get_followed_channels(api: &TwitchFixture) -> Result<()> {
    api.api
        .get_followed_channels(&api.selected_user_id())
        .json()
        .await?;
    Ok(())
}

async fn mock_api_get_channel_followers(api: &TwitchFixture) -> Result<()> {
    api.api
        .get_channel_followers(&api.selected_broadcaster_id())
        .json()
        .await?;
    Ok(())
}

async fn mock_api_get_charity_campaign(api: &TwitchFixture) -> Result<()> {
    api.api
        .get_charity_campaign(&api.selected_broadcaster_id())
        .json()
        .await?;
    Ok(())
}

async fn mock_api_get_charity_campaign_donations(api: &TwitchFixture) -> Result<()> {
    api.api
        .get_charity_campaign_donations(&api.selected_broadcaster_id())
        .json()
        .await?;

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
//
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
//
// async fn mock_api_get_shared_chat_session(api: &TwitchFixture) -> Result<()> {
//     api.api
//         .get_shared_chat_session(&api.selected_broadcaster_id())
//         .json()
//         .await?;
//     Ok(())
// }
//
// async fn mock_api_get_user_emotes(api: &TwitchFixture) -> Result<()> {
//     api.api
//         .get_user_emotes(&api.selected_user_id())
//         .json()
//         .await?;
//     Ok(())
// }
//
async fn mock_api_update_chat_settings(api: &TwitchFixture) -> Result<()> {
    api.api
        .update_chat_settings(&api.selected_broadcaster_id(), &api.selected_moderator_id())
        .follower_mode(false)
        .json()
        .await?;
    Ok(())
}
//
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
//
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
//
// async fn mock_api_send_chat_message(api: &TwitchFixture) -> Result<()> {
//     api.api
//         .send_chat_message(&api.selected_broadcaster_id(), &UserId::from(""), "")
//         .json()
//         .await?;
//     Ok(())
// }
//
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

async fn mock_api_create_clip(api: &TwitchFixture) -> Result<()> {
    let _resp = api
        .api
        .create_clip(&api.selected_broadcaster_id())
        .send()
        .await?;

    Ok(())
}

async fn mock_api_get_clips(api: &TwitchFixture) -> Result<()> {
    let _resp = api
        .api
        .get_clips_by_broadcaster_id(&api.selected_broadcaster_id())
        .json()
        .await?;

    Ok(())
}

async fn mock_api_get_creator_goals(api: &TwitchFixture) -> Result<()> {
    api.api
        .get_creator_goals(&api.selected_broadcaster_id())
        .await?;
    Ok(())
}

// async fn mock_api_get_hype_train_events(api: &TwitchFixture) -> Result<()> {
//     api.api
//         .get_hype_train_events(&api.selected_broadcaster_id())
//         .json()
//         .await?;
//
//     Ok(())
// }

// async fn mock_api_get_hype_train_status(api: &TwitchFixture) -> Result<()> {
//     api.api
//         .get_hype_train_status(&api.selected_broadcaster_id())
//         .json()
//         .await?;
//
//     Ok(())
// }

async fn mock_api_get_polls(api: &TwitchFixture) -> Result<()> {
    api.api
        .get_polls(&api.selected_broadcaster_id())
        .send()
        .await?;
    Ok(())
}

async fn mock_api_create_poll(api: &TwitchFixture) -> Result<()> {
    api.api
        .create_poll(
            &api.selected_broadcaster_id(),
            "test",
            &[Title::new("Heads"), Title::new("Tails")],
            30,
        )
        .send()
        .await?;
    Ok(())
}

async fn mock_api_get_predictions(api: &TwitchFixture) -> Result<()> {
    api.api
        .get_predictions(&api.selected_broadcaster_id())
        .send()
        .await?;
    Ok(())
}

async fn mock_api_create_prediction(api: &TwitchFixture) -> Result<()> {
    api.api
        .create_prediction(
            &api.selected_broadcaster_id(),
            "Any leeks in the stream?",
            &[
                Title::new("Yes, give it time."),
                Title::new("Definitely not."),
            ],
            120,
        )
        .await?;
    Ok(())
}

async fn mock_api_end_prediction(api: &TwitchFixture) -> Result<()> {
    let prediction_id = PredictionId::from(api.selected_broadcaster_id().to_string().clone());
    api.api
        .end_prediction(
            &api.selected_broadcaster_id(),
            &prediction_id,
            PredictionStatus::CANCELED,
        )
        .send()
        .await?;
    Ok(())
}

async fn mock_api_start_raid(api: &TwitchFixture) -> Result<()> {
    let random_user = api.get_random_user()?;
    let to_user = BroadcasterId::from(random_user.id);
    api.api
        .start_raid(&api.selected_broadcaster_id(), &to_user)
        .await?;
    Ok(())
}

async fn mock_api_cancel_raid(api: &TwitchFixture) -> Result<()> {
    api.api.cancel_raid(&api.selected_broadcaster_id()).await?;
    Ok(())
}

async fn mock_api_search_categories(api: &TwitchFixture) -> Result<()> {
    api.api.search_categories("ff").send().await?;
    Ok(())
}

async fn mock_api_search_channels(api: &TwitchFixture) -> Result<()> {
    api.api.search_channels("ff").send().await?;
    Ok(())
}

async fn mock_api_get_broadcaster_subscriptions(api: &TwitchFixture) -> Result<()> {
    api.api
        .get_broadcaster_subscriptions(&api.selected_broadcaster_id())
        .send()
        .await?;

    Ok(())
}

async fn mock_api_check_user_subscription(api: &TwitchFixture) -> Result<()> {
    api.api
        .check_user_subscription(&api.selected_broadcaster_id(), &api.selected_user_id())
        .await?;

    Ok(())
}

async fn mock_api_get_channel_teams(api: &TwitchFixture) -> Result<()> {
    api.api
        .get_channel_teams(&api.selected_broadcaster_id())
        .await?;
    Ok(())
}

async fn mock_api_get_teams(api: &TwitchFixture) -> Result<()> {
    api.api
        .get_teams(&TeamId::from(api.selected_broadcaster_id().to_string()))
        .await?;
    Ok(())
}

async fn mock_api_get_users(api: &TwitchFixture) -> Result<()> {
    api.api.get_users().send().await?;
    Ok(())
}

async fn mock_api_update_user(api: &TwitchFixture) -> Result<()> {
    api.api.update_user("ffs").await?;
    Ok(())
}

async fn mock_api_get_user_block_list(api: &TwitchFixture) -> Result<()> {
    api.api
        .get_user_block_list(&api.selected_broadcaster_id())
        .send()
        .await?;
    Ok(())
}

async fn mock_api_block_user(api: &TwitchFixture) -> Result<()> {
    let random_user = api.get_random_user()?;
    let user_id = UserId::from(random_user.id);
    api.api.block_user(&user_id).send().await?;
    Ok(())
}

async fn mock_api_unblock_user(api: &TwitchFixture) -> Result<()> {
    api.api.unblock_user(&api.selected_user_id()).await?;
    Ok(())
}
// async fn mock_api_get_user_extensions(api: &TwitchFixture) -> Result<()> {
//     api.api.get_user_extensions().await?;
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

pub async fn mock_api_get_videos(api: &TwitchFixture) -> Result<()> {
    api.api.get_videos(&api.selected_user_id()).send().await?;
    Ok(())
}

// pub async fn mock_api_delete_videos(api: &TwitchFixture) -> Result<()> {
//     api.api.delete_videos(&[api.selected_id()]).json().await?;
//     Ok(())
// }

async fn mock_api_send_whisper(api: &TwitchFixture) -> Result<()> {
    let random_user = api.get_random_user()?;
    let moderation = UserId::from(api.selected_user.id.clone());

    let user = UserId::from(random_user.id);
    api.api.send_whisper(&moderation, &user, "hello").await?;

    Ok(())
}
