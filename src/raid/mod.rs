use asknothingx2_util::api::Method;
use response::StartRaidResponse;

use crate::{
    base::TwitchAPIBase,
    request::{EmptyBody, EndpointType, TwitchAPIRequest},
    types::{constants::BROADCASTER_ID, BroadcasterId},
    TwitchAPI,
};

pub mod response;
pub mod types;

#[cfg_attr(docsrs, doc(cfg(feature = "raid")))]
pub trait RaidAPI: TwitchAPIBase {
    /// <https://dev.twitch.tv/docs/api/reference/#start-a-raid>
    fn start_raid(
        &self,
        from_broadcaster_id: &str,
        to_broadcaster_id: &str,
    ) -> TwitchAPIRequest<EmptyBody, StartRaidResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#cancel-a-raid>
    fn cancel_raid(&self, broadcaster_id: BroadcasterId) -> TwitchAPIRequest<EmptyBody, EmptyBody>;
}

impl RaidAPI for TwitchAPI {
    fn start_raid(
        &self,
        from_broadcaster_id: &str,
        to_broadcaster_id: &str,
    ) -> TwitchAPIRequest<EmptyBody, StartRaidResponse> {
        let mut url = self.build_url();
        url.path(["raids"]).query_extend([
            ("from_broadcaster_id", from_broadcaster_id),
            ("to_broadcaster_id", to_broadcaster_id),
        ]);

        TwitchAPIRequest::new(
            EndpointType::Startraid,
            url.build(),
            Method::POST,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn cancel_raid(&self, broadcaster_id: BroadcasterId) -> TwitchAPIRequest<EmptyBody, EmptyBody> {
        let mut url = self.build_url();
        url.path(["raids"]).query(BROADCASTER_ID, broadcaster_id);

        TwitchAPIRequest::new(
            EndpointType::Cancelraid,
            url.build(),
            Method::DELETE,
            self.build_headers().build(),
            EmptyBody,
        )
    }
}
