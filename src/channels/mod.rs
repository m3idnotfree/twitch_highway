use asknothingx2_util::api::Method;
use request::ModifyChannelRequest;
use response::{
    ChannelEditorsResponse, ChannelFollowersResponse, ChannelInfoResponse,
    FollowerdChannelsResponse,
};

use crate::{
    request::{EndpointType, NoContent, TwitchAPIRequest},
    types::{
        constants::{BROADCASTER_ID, CHANNELS, USER_ID},
        BroadcasterId, PaginationQuery, UserId,
    },
    TwitchAPI,
};

pub mod request;
pub mod response;
pub mod types;

endpoints! {
    ChannelsAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#get-channel-information>
        fn get_channel_info(
            &self,
            broadcaster_ids: &[BroadcasterId],
        ) -> ChannelInfoResponse {
            endpoint_type: EndpointType::GetChanelInformation,
            method: Method::GET,
            path: [CHANNELS],
            query_params: {
                extend(broadcaster_ids.iter().map(|x| (BROADCASTER_ID, x)))
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#modify-channel-information>
        fn modify_channel_info(
            &self,
            broadcaster_id: &BroadcasterId,
            opts: Option<ModifyChannelRequest>,
        ) -> NoContent {
            endpoint_type: EndpointType::ModifyChannelInformation,
            method: Method::PATCH,
            path: [CHANNELS],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id)
            },
            headers: [json],
            body: opts.and_then(|o| o.into_json())
        }

        /// <https://dev.twitch.tv/docs/api/reference/#get-channel-editors>
        fn get_channel_editors(
            &self,
            broadcaster_id: &BroadcasterId,
        ) -> ChannelEditorsResponse {
            endpoint_type: EndpointType::GetChannelEditors,
            method: Method::GET,
            path: [CHANNELS, "editors"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#get-followed-channels>
        fn get_followed_channels(
            &self,
            user_id: &UserId,
            broadcaster_id: Option<&BroadcasterId>,
            pagination: Option<PaginationQuery>,
        ) -> FollowerdChannelsResponse {
            endpoint_type: EndpointType::GetFollowedChannels,
            method: Method::GET,
            path: [CHANNELS, "followed"],
            query_params: {
                query(USER_ID, user_id),
                opt(BROADCASTER_ID, broadcaster_id),
                pagination(pagination)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#get-channel-followers>
        fn get_channel_followers(
            &self,
            broadcaster_id: &BroadcasterId,
            user_id: Option<&UserId>,
            pagination: Option<PaginationQuery>,
        ) -> ChannelFollowersResponse {
            endpoint_type: EndpointType::GetChannelFollowers,
            method: Method::GET,
            path: [CHANNELS, "followers"],
            query_params: {
                opt(USER_ID, user_id),
                query(BROADCASTER_ID, broadcaster_id),
                pagination(pagination)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        channels::{
            request::{
                ContentClassificationLabel, ContentClassificationLabelsID, ModifyChannelRequest,
            },
            ChannelsAPI,
        },
        types::{BroadcasterId, GameId, UserId},
    };

    api_test!(get_channel_info, [&[BroadcasterId::new("141981764")]]);

    api_test!(
        modify_channel_info,
        [
            &BroadcasterId::new("41245072"),
            Some(
                ModifyChannelRequest::new()
                    .title("there are helicopters in the game? REASON TO PLAY FORTNITE found")
                    .game_id(&GameId::new("33214"))
                    .broadcaster_language("en")
                    .tags(&["LevelingUp"])
            )
        ]
    );

    api_test!(get_channel_editors, [&BroadcasterId::new("141981764")]);

    api_test!(get_followed_channels, [&UserId::new("123456"), None, None]);

    api_test!(
        get_channel_followers,
        [&BroadcasterId::new("123456"), None, None,]
    );

    api_test!(extra
        modify_channel_info,
        modify_channel_info2,
        [
            &BroadcasterId::new("41245072"),
            Some(
                ModifyChannelRequest::new()
                    .game_id(&GameId::new("SomeGameID"))
                        .content_classification_labels(&[
                            ContentClassificationLabel::new(ContentClassificationLabelsID::Gambling, true),
                            ContentClassificationLabel::new(ContentClassificationLabelsID::DrugsIntoxication, false)
                    ])
                    .is_branded_content(true)
            )
        ]
    );

    api_test!(extra
        get_followed_channels,
        get_followed_channels2,
        [&UserId::new("123456"), Some(&BroadcasterId::new("654321")), None]
    );

    api_test!(extra
        get_channel_followers,
        get_channel_followers2,
        [
            &BroadcasterId::new("123456"),
            Some(&UserId::new("654321")),
            None,
        ]
    );
}
