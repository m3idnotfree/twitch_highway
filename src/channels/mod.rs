use asknothingx2_util::api::Method;
use request::ModifyChannelRequest;
use response::{
    ChannelEditorsResponse, ChannelFollowersResponse, ChannelInfoResponse,
    FollowerdChannelsResponse,
};

use crate::{
    base::TwitchAPIBase,
    request::{EmptyBody, EndpointType, TwitchAPIRequest},
    types::{
        constants::{BROADCASTER_ID, CHANNELS, USER_ID},
        BroadcasterId, PaginationQuery, UserId,
    },
    TwitchAPI,
};

pub mod request;
pub mod response;
pub mod types;

#[cfg_attr(docsrs, doc(cfg(feature = "channels")))]
pub trait ChannelsAPI: TwitchAPIBase {
    /// <https://dev.twitch.tv/docs/api/reference/#get-channel-information>
    fn get_channel_info(
        &self,
        broadcaster_ids: &[BroadcasterId],
    ) -> TwitchAPIRequest<EmptyBody, ChannelInfoResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#modify-channel-information>
    fn modify_channel_info(
        &self,
        broadcaster_id: BroadcasterId,
        opts: Option<ModifyChannelRequest>,
    ) -> TwitchAPIRequest<ModifyChannelRequest, EmptyBody>;
    /// <https://dev.twitch.tv/docs/api/reference/#get-channel-editors>
    fn get_channel_editors(
        &self,
        broadcaster_id: BroadcasterId,
    ) -> TwitchAPIRequest<EmptyBody, ChannelEditorsResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#get-followed-channels>
    fn get_followed_channels(
        &self,
        user_id: UserId,
        broadcaster_id: Option<BroadcasterId>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, FollowerdChannelsResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#get-channel-followers>
    fn get_channel_followers(
        &self,
        user_id: Option<UserId>,
        broadcaster_id: BroadcasterId,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, ChannelFollowersResponse>;
}

impl ChannelsAPI for TwitchAPI {
    fn get_channel_info(
        &self,
        broadcaster_ids: &[BroadcasterId],
    ) -> TwitchAPIRequest<EmptyBody, ChannelInfoResponse> {
        let mut url = self.build_url();
        url.path([CHANNELS])
            .query_extend(broadcaster_ids.iter().map(|x| (BROADCASTER_ID, x)));

        TwitchAPIRequest::new(
            EndpointType::GetChanelInformation,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn modify_channel_info(
        &self,
        broadcaster_id: BroadcasterId,
        opts: Option<ModifyChannelRequest>,
    ) -> TwitchAPIRequest<ModifyChannelRequest, EmptyBody> {
        let mut url = self.build_url();
        url.path([CHANNELS]).query(BROADCASTER_ID, broadcaster_id);

        let mut headers = self.build_headers();
        headers.json();
        TwitchAPIRequest::new(
            EndpointType::ModifyChannelInformation,
            url.build(),
            Method::PATCH,
            headers.build(),
            opts.unwrap_or_default(),
        )
    }
    fn get_channel_editors(
        &self,
        broadcaster_id: BroadcasterId,
    ) -> TwitchAPIRequest<EmptyBody, ChannelEditorsResponse> {
        let mut url = self.build_url();
        url.path([CHANNELS, "editors"])
            .query(BROADCASTER_ID, broadcaster_id);

        TwitchAPIRequest::new(
            EndpointType::GetChannelEditors,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn get_followed_channels(
        &self,
        user_id: UserId,
        broadcaster_id: Option<BroadcasterId>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, FollowerdChannelsResponse> {
        let mut url = self.build_url();
        url.path([CHANNELS, "followed"])
            .query(USER_ID, user_id)
            .query_opt(BROADCASTER_ID, broadcaster_id)
            .query_opt_pairs(pagination);

        TwitchAPIRequest::new(
            EndpointType::GetFollowedChannels,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn get_channel_followers(
        &self,
        user_id: Option<UserId>,
        broadcaster_id: BroadcasterId,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, ChannelFollowersResponse> {
        let mut url = self.build_url();
        url.path([CHANNELS, "followers"])
            .query_opt(USER_ID, user_id)
            .query(BROADCASTER_ID, broadcaster_id)
            .query_opt_pairs(pagination);

        TwitchAPIRequest::new(
            EndpointType::GetChannelFollowers,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
}
