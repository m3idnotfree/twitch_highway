mod builder;
mod response;
mod types;

pub use builder::{GetChannelFollowers, GetFollowedChannels, ModifyChannelInfo};
pub use response::{
    ChannelEditorsResponse, ChannelFollowersResponse, ChannelInfoResponse,
    FollowerdChannelsResponse,
};
pub use types::{
    ChannelEditor, ChannelFollower, ChannelInfo, ContentClassificationLabel,
    ContentClassificationLabelsID, FollowedChannel,
};

use std::future::Future;

use crate::{
    Client, Error,
    types::{
        BroadcasterId, UserId,
        constants::{BROADCASTER_ID, CHANNELS, EDITORS},
    },
};

pub trait ChannelsAPI {
    /// See <https://dev.twitch.tv/docs/api/reference/#get-channel-information>
    fn get_channel_info(
        &self,
        broadcaster_ids: &[BroadcasterId],
    ) -> impl Future<Output = Result<ChannelInfoResponse, Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#modify-channel-information>
    fn modify_channel_info<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
    ) -> ModifyChannelInfo<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#get-channel-editors>
    fn get_channel_editor(
        &self,
        broadcaster_id: &BroadcasterId,
    ) -> impl Future<Output = Result<ChannelEditorsResponse, Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#get-followed-channels>
    fn get_followed_channels<'a>(&'a self, user_id: &'a UserId) -> GetFollowedChannels<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#get-channel-followers>
    fn get_channel_followers<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
    ) -> GetChannelFollowers<'a>;
}

impl ChannelsAPI for Client {
    async fn get_channel_info(
        &self,
        broadcaster_ids: &[BroadcasterId],
    ) -> Result<ChannelInfoResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().push(CHANNELS);

        url.query_pairs_mut()
            .extend_pairs(broadcaster_ids.iter().map(|id| (BROADCASTER_ID, id)));

        self.json(self.http_client().get(url)).await
    }

    fn modify_channel_info<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
    ) -> ModifyChannelInfo<'a> {
        ModifyChannelInfo::new(self, broadcaster_id)
    }

    async fn get_channel_editor(
        &self,
        broadcaster_id: &BroadcasterId,
    ) -> Result<ChannelEditorsResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().extend([CHANNELS, EDITORS]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id);

        self.json(self.http_client().get(url)).await
    }

    fn get_followed_channels<'a>(&'a self, user_id: &'a UserId) -> GetFollowedChannels<'a> {
        GetFollowedChannels::new(self, user_id)
    }

    fn get_channel_followers<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
    ) -> GetChannelFollowers<'a> {
        GetChannelFollowers::new(self, broadcaster_id)
    }
}
