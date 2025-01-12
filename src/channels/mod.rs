use asknothingx2_util::api::Method;
use request::{ChannelFollowRequest, ModifyChannelRequest};

use crate::{
    base::TwitchAPIBase,
    types::{BroadcasterId, BROADCASTER_ID, CHANNELS},
    EmptyBody, EndpointType, TwitchAPI, TwitchAPIRequest,
};

pub mod request;
pub mod response;
pub mod types;

pub trait ChannelsAPI: TwitchAPIBase {
    fn get_channel_info<I: IntoIterator<Item = BroadcasterId>>(
        &self,
        broadcaster_ids: I,
    ) -> TwitchAPIRequest<EmptyBody>;
    fn modify_channel_info(
        &self,
        broadcaster_id: BroadcasterId,
        request: ModifyChannelRequest,
    ) -> TwitchAPIRequest<ModifyChannelRequest>;
    fn get_channel_editors(&self, broadcaster_id: BroadcasterId) -> TwitchAPIRequest<EmptyBody>;
    fn get_followed_channels(&self, request: ChannelFollowRequest) -> TwitchAPIRequest<EmptyBody>;
    fn get_channel_followers(&self, request: ChannelFollowRequest) -> TwitchAPIRequest<EmptyBody>;
}

impl ChannelsAPI for TwitchAPI {
    fn get_channel_info<I: IntoIterator<Item = BroadcasterId>>(
        &self,
        broadcaster_ids: I,
    ) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path([CHANNELS])
            .query_extend(broadcaster_ids.into_iter().map(|x| (BROADCASTER_ID, x)));

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
        request: ModifyChannelRequest,
    ) -> TwitchAPIRequest<ModifyChannelRequest> {
        let mut url = self.build_url();
        url.path([CHANNELS])
            .query([(BROADCASTER_ID, broadcaster_id)]);

        let mut headers = self.build_headers();
        headers.json();
        TwitchAPIRequest::new(
            EndpointType::ModifyChannelInformation,
            url.build(),
            Method::PATCH,
            headers.build(),
            request,
        )
    }
    fn get_channel_editors(&self, broadcaster_id: BroadcasterId) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path([CHANNELS, "editors"])
            .query([(BROADCASTER_ID, broadcaster_id)]);

        TwitchAPIRequest::new(
            EndpointType::GetChannelEditors,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn get_followed_channels(&self, request: ChannelFollowRequest) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path([CHANNELS, "followed"]).query_pairs(request);

        TwitchAPIRequest::new(
            EndpointType::GetFollowedChannels,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn get_channel_followers(&self, request: ChannelFollowRequest) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path([CHANNELS, "followed"]).query_pairs(request);

        TwitchAPIRequest::new(
            EndpointType::GetChannelFollowers,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
}
