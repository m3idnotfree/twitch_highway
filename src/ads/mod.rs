mod response;
mod types;

pub use response::{AdScheduleResponse, SnoozeNextAdResponse, StartCommercialResponse};
pub use types::{AdSchedule, SnoozeNextAd, StartCommercial};

use types::StartCommercialBody;

use std::future::Future;

use crate::{
    Client, Error,
    types::{
        BroadcasterId,
        constants::{ADS, BROADCASTER_ID, CHANNELS, COMMERCIAL, SCHEDULE, SNOOZE},
    },
};

pub trait AdsAPI {
    /// See <https://dev.twitch.tv/docs/api/reference/#start-commercial>
    fn start_commercial(
        &self,
        broadcaster_id: &BroadcasterId,
        length: u8,
    ) -> impl Future<Output = Result<StartCommercialResponse, Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#get-ad-schedule>
    fn get_ad_schedule(
        &self,
        broadcaster_id: &BroadcasterId,
    ) -> impl Future<Output = Result<AdScheduleResponse, Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#snooze-next-ad>
    fn snooze_next_ad(
        &self,
        broadcaster_id: &BroadcasterId,
    ) -> impl Future<Output = Result<SnoozeNextAdResponse, Error>> + Send;
}

impl AdsAPI for Client {
    async fn start_commercial(
        &self,
        broadcaster_id: &BroadcasterId,
        length: u8,
    ) -> Result<StartCommercialResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([CHANNELS, COMMERCIAL]);

        let req = self.http_client().post(url).json(&StartCommercialBody {
            broadcaster_id,
            length,
        });
        self.json(req).await
    }

    async fn get_ad_schedule(
        &self,
        broadcaster_id: &BroadcasterId,
    ) -> Result<AdScheduleResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().extend([CHANNELS, ADS]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id);

        self.json(self.http_client().get(url)).await
    }

    async fn snooze_next_ad(
        &self,
        broadcaster_id: &BroadcasterId,
    ) -> Result<SnoozeNextAdResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([CHANNELS, ADS, SCHEDULE, SNOOZE]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id);

        self.json(self.http_client().post(url)).await
    }
}
