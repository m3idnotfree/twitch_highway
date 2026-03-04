#[macro_use]
mod common;

use twitch_highway::{
    channels::{ChannelsAPI, ContentClassificationLabel, ContentClassificationLabelsID},
    types::{BroadcasterId, GameId, UserId},
};

api_test!(
    get_channel_info[&[BroadcasterId::from("141981764")]],
    get_channel_editor[&BroadcasterId::from("141981764")]
);

api_test!(
    modify_channel_info | api | {
        api.modify_channel_info(&BroadcasterId::from("41245072"))
            .title("there are helicopters in the game? REASON TO PLAY FORTNITE found")
            .game_id(&GameId::from("33214"))
            .broadcaster_language("en")
            .tags(&["LevelingUp"])
    }
);

api_test!(get_followed_channels | api | { api.get_followed_channels(&UserId::from("123456")) });

api_test!(
    get_channel_followers | api | { api.get_channel_followers(&BroadcasterId::from("123456")) }
);

api_test!(
    modify_channel_info as modify_channel_info2 | api | {
        api.modify_channel_info(&BroadcasterId::from("41245072"))
            .game_id(&GameId::from("SomeGameID"))
            .content_classification_labels(&[
                ContentClassificationLabel::new(ContentClassificationLabelsID::Gambling, true),
                ContentClassificationLabel::new(
                    ContentClassificationLabelsID::DrugsIntoxication,
                    false,
                ),
            ])
            .is_branded_content(true)
    }
);

api_test!(
    get_followed_channels as get_followed_channels2 | api | {
        api.get_followed_channels(&UserId::from("123456"))
            .broadcaster_id(&BroadcasterId::from("654321"))
    }
);

api_test!(
    get_channel_followers as get_channel_followers2 | api | {
        api.get_channel_followers(&BroadcasterId::from("123456"))
            .user_id(&UserId::from("654321"))
    }
);
