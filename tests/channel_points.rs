#![cfg(feature = "channel-points")]

#[macro_use]
mod common;

use twitch_highway::{
    channel_points::{ChannelPointsAPI, RedemptionStatus},
    types::{BroadcasterId, RedemptionId, RewardId},
};

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
