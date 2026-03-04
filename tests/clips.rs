#![cfg(feature = "clips")]

#[macro_use]
mod common;

use twitch_highway::{
    clips::ClipsAPI,
    types::{BroadcasterId, ClipId, UserId},
};

api_test!(create_clip | api | { api.create_clip(&BroadcasterId::from("44322889")) });

api_test!(
    get_clips | api | {
        api.get_clips_by_ids(&[ClipId::from("AwkwardHelplessSalamanderSwiftRage")])
    }
);

api_test!(
    get_clips_download
    [
        &UserId::from("141981764"),
        &BroadcasterId::from("141981764"),
        &[
            ClipId::from("InexpensiveDistinctFoxChefFrank"),
            ClipId::from("SpinelessCloudyLeopardMcaT")
        ]
    ]
);

api_test!(
    get_clips as get_clips2 | api | {
        api.get_clips_by_broadcaster_id(&BroadcasterId::from("1234"))
            .first(5)
    }
);

api_test!(
    create_clip_from_vod
    [
        &UserId::from("12826"),
        &BroadcasterId::from("141981764"),
        "2277656159",
        8,
        "title",
        None
    ]
);
