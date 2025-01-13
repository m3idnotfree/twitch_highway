use asknothingx2_util::api::Method;
use request::StartCommercialRequestBody;

use crate::{
    base::TwitchAPIBase,
    types::{BroadcasterId, BROADCASTER_ID, CHANNELS},
    EmptyBody, EndpointType, TwitchAPI, TwitchAPIRequest,
};

pub mod request;
pub mod response;
pub mod types;

pub trait AdsAPI: TwitchAPIBase {
    /// https://dev.twitch.tv/docs/api/reference/#start-commercial
    fn start_commercial(
        &self,
        broadcaster_id: BroadcasterId,
        length: u64,
    ) -> TwitchAPIRequest<StartCommercialRequestBody>;
    /// https://dev.twitch.tv/docs/api/reference/#get-ad-schedule
    fn get_ad_schedule(&self, broadcaster_id: BroadcasterId) -> TwitchAPIRequest<EmptyBody>;
    /// https://dev.twitch.tv/docs/api/reference/#snooze-next-ad
    fn snooze_next_ad(&self, broadcaster_id: BroadcasterId) -> TwitchAPIRequest<EmptyBody>;
}

impl AdsAPI for TwitchAPI {
    fn start_commercial(
        &self,
        broadcaster_id: BroadcasterId,
        length: u64,
    ) -> TwitchAPIRequest<StartCommercialRequestBody> {
        let mut url = self.build_url();
        url.path([CHANNELS, "commercial"]);

        let mut headers = self.build_headers();
        headers.json();

        TwitchAPIRequest::new(
            EndpointType::StartCommercial,
            url.build(),
            Method::POST,
            headers.build(),
            StartCommercialRequestBody::new(broadcaster_id, length),
        )
    }
    fn get_ad_schedule(&self, broadcaster_id: BroadcasterId) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path([CHANNELS, "ads"])
            .query(BROADCASTER_ID, broadcaster_id);

        TwitchAPIRequest::new(
            EndpointType::GetAdSchedule,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn snooze_next_ad(&self, broadcaster_id: BroadcasterId) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path([CHANNELS, "ads", "schedule", "snooze"])
            .query(BROADCASTER_ID, broadcaster_id);

        TwitchAPIRequest::new(
            EndpointType::SnoozeNextAd,
            url.build(),
            Method::POST,
            self.build_headers().build(),
            EmptyBody,
        )
    }
}
