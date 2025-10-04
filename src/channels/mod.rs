mod request;
mod response;
mod types;

pub use request::{
    ContentClassificationLabel, ContentClassificationLabelsID, ModifyChannelRequest,
};
pub use response::{
    ChannelEditorsResponse, ChannelFollowersResponse, ChannelInfoResponse,
    FollowerdChannelsResponse,
};
pub use types::{ChannelEditor, ChannelFollower, ChannelInfo, FollowedChannel};

use crate::{
    request::NoContent,
    types::{
        constants::{BROADCASTER_ID, CHANNELS, USER_ID},
        BroadcasterId, PaginationQuery, UserId,
    },
};

endpoints! {
    ChannelsAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#get-channel-information>
        fn get_channel_info(
            &self,
            broadcaster_ids: &[BroadcasterId],
        ) -> ChannelInfoResponse {
            endpoint_type: GetChanelInformation,
            method: GET,
            path: [CHANNELS],
            query_params: {
                extend(broadcaster_ids.iter().map(|x| (BROADCASTER_ID, x)))
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#modify-channel-information>
        fn modify_channel_info(
            &self,
            broadcaster_id: &BroadcasterId,
            modify: ModifyChannelRequest,
        ) -> NoContent {
            endpoint_type: ModifyChannelInformation,
            method: PATCH,
            path: [CHANNELS],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id)
            },
            headers: [json],
            body: modify.into_json()
        }

        /// <https://dev.twitch.tv/docs/api/reference/#get-channel-editors>
        fn get_channel_editors(
            &self,
            broadcaster_id: &BroadcasterId,
        ) -> ChannelEditorsResponse {
            endpoint_type: GetChannelEditors,
            method: GET,
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
            endpoint_type: GetFollowedChannels,
            method: GET,
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
            endpoint_type: GetChannelFollowers,
            method: GET,
            path: [CHANNELS, "followers"],
            query_params: {
                opt(USER_ID, user_id),
                query(BROADCASTER_ID, broadcaster_id),
                pagination(pagination)
            }
        }
    }
}
