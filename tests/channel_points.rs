#![cfg(feature = "channel-points")]

#[macro_use]
mod common;

use anyhow::Result;
use common::{mock_api_start, TwitchFixture};
use twitch_highway::{
    channel_points::{ChannelPointsAPI, CustomReward, RedemptionStatus},
    types::{BroadcasterId, RedemptionId, RewardId},
};
use twitch_oauth_token::scope::ChannelPointScopes;

api_test!(build
    create_custom_rewards |api| {
        api.create_custom_rewards(
            &BroadcasterId::from("274637212"),
            "game analysis 1v1",
            50000
        )
    }
);
api_test!(
    delete_custom_reward,
    [
        &BroadcasterId::from("274637212"),
        &RewardId::from("b045196d-9ce7-4a27-a9b9-279ed341ab28"),
    ]
);
api_test!(build
    get_custom_reward |api| {
        api.get_custom_reward(&BroadcasterId::from("274637212"))
    }
);
api_test!(build
    get_custom_reward_redemption |api| {
        api.get_custom_reward_redemption(
            &BroadcasterId::from("274637212"),
            &RewardId::from("92af127c-7326-4483-a52b-b0da0be61c01"),
        )
        .status(RedemptionStatus::CANCELED)
    }
);
api_test!(build
    update_custom_reward |api| {
        api.update_custom_reward(
            &BroadcasterId::from("274637212"),
            &RewardId::from("92af127c-7326-4483-a52b-b0da0be61c01"),
        )
        .is_enabled(false)
    }
);
api_test!(
    update_redemption_status,
    [
        &BroadcasterId::from("274637212"),
        &RewardId::from("92af127c-7326-4483-a52b-b0da0be61c01"),
        &[RedemptionId::from("17fa2df1-ad76-4804-bfa5-a40ef63efe63")],
        RedemptionStatus::CANCELED,
    ]
);

api_test!(build_extra
    get_custom_reward,
    get_custom_reward2 |api| {
        api.get_custom_reward(&BroadcasterId::from("274637212"))
            .only_manageable_rewards(true)
    }
);
api_test!(build_extra
    get_custom_reward,
    get_custom_reward3 |api| {
        api.get_custom_reward(&BroadcasterId::from("274637212"))
            .custom_reward_ids(&[RewardId::from("2af127c-7326-4483-a52b-b0da0be61c01")])
    }
);
api_test!(build_extra
    get_custom_reward_redemption,
    get_custom_reward_redemption2 |api| {
        api.get_custom_reward_redemption(
            &BroadcasterId::from("274637212"),
            &RewardId::from("92af127c-7326-4483-a52b-b0da0be61c01"),
        )
    }
);
api_test!(build_extra
    update_custom_reward,
    update_custom_reward2 |api| {
        api.update_custom_reward(
            &BroadcasterId::from("274637212"),
            &RewardId::from("92af127c-7326-4483-a52b-b0da0be61c01"),
        )
        .title("game analysis 2v2")
    }
);

#[tokio::test]
async fn mock_api() -> Result<()> {
    let _cmd = mock_api_start().await?;

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
