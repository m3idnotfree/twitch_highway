use asknothingx2_util::api::Method;
use request::ModifyChannelRequest;
use response::{
    ChannelEditorsResponse, ChannelFollowersResponse, ChannelInfoResponse,
    FollowerdChannelsResponse,
};

use crate::{
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
pub trait ChannelsAPI {
    /// <https://dev.twitch.tv/docs/api/reference/#get-channel-information>
    fn get_channel_info(
        &self,
        broadcaster_ids: &[BroadcasterId],
    ) -> TwitchAPIRequest<ChannelInfoResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#modify-channel-information>
    fn modify_channel_info(
        &self,
        broadcaster_id: BroadcasterId,
        opts: Option<ModifyChannelRequest>,
    ) -> TwitchAPIRequest<EmptyBody>;
    /// <https://dev.twitch.tv/docs/api/reference/#get-channel-editors>
    fn get_channel_editors(
        &self,
        broadcaster_id: BroadcasterId,
    ) -> TwitchAPIRequest<ChannelEditorsResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#get-followed-channels>
    fn get_followed_channels(
        &self,
        user_id: UserId,
        broadcaster_id: Option<BroadcasterId>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<FollowerdChannelsResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#get-channel-followers>
    fn get_channel_followers(
        &self,
        user_id: Option<UserId>,
        broadcaster_id: BroadcasterId,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<ChannelFollowersResponse>;
}

impl ChannelsAPI for TwitchAPI {
    fn get_channel_info(
        &self,
        broadcaster_ids: &[BroadcasterId],
    ) -> TwitchAPIRequest<ChannelInfoResponse> {
        let mut url = self.build_url();
        url.path([CHANNELS])
            .query_extend(broadcaster_ids.iter().map(|x| (BROADCASTER_ID, x)));

        TwitchAPIRequest::new(
            EndpointType::GetChanelInformation,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            None,
        )
    }
    fn modify_channel_info(
        &self,
        broadcaster_id: BroadcasterId,
        opts: Option<ModifyChannelRequest>,
    ) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path([CHANNELS]).query(BROADCASTER_ID, broadcaster_id);

        let mut headers = self.build_headers();
        headers.json();

        let opts = if let Some(opts) = opts {
            opts.to_json()
        } else {
            None
        };
        TwitchAPIRequest::new(
            EndpointType::ModifyChannelInformation,
            url.build(),
            Method::PATCH,
            headers.build(),
            opts,
        )
    }
    fn get_channel_editors(
        &self,
        broadcaster_id: BroadcasterId,
    ) -> TwitchAPIRequest<ChannelEditorsResponse> {
        let mut url = self.build_url();
        url.path([CHANNELS, "editors"])
            .query(BROADCASTER_ID, broadcaster_id);

        TwitchAPIRequest::new(
            EndpointType::GetChannelEditors,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            None,
        )
    }
    fn get_followed_channels(
        &self,
        user_id: UserId,
        broadcaster_id: Option<BroadcasterId>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<FollowerdChannelsResponse> {
        let mut url = self.build_url();
        url.path([CHANNELS, "followed"])
            .query(USER_ID, user_id)
            .query_opt(BROADCASTER_ID, broadcaster_id);
        if let Some(pagination) = pagination {
            pagination.apply_to_url(&mut url);
        }

        TwitchAPIRequest::new(
            EndpointType::GetFollowedChannels,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            None,
        )
    }
    fn get_channel_followers(
        &self,
        user_id: Option<UserId>,
        broadcaster_id: BroadcasterId,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<ChannelFollowersResponse> {
        let mut url = self.build_url();
        url.path([CHANNELS, "followers"])
            .query_opt(USER_ID, user_id)
            .query(BROADCASTER_ID, broadcaster_id);
        if let Some(pagination) = pagination {
            pagination.apply_to_url(&mut url);
        }

        TwitchAPIRequest::new(
            EndpointType::GetChannelFollowers,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            None,
        )
    }
}
