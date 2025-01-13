use asknothingx2_util::api::Method;

use crate::{
    base::TwitchAPIBase,
    types::{BroadcasterId, BROADCASTER_ID},
    EmptyBody, EndpointType, TwitchAPI, TwitchAPIRequest,
};

pub mod response;
pub mod types;

pub trait RaidAPI: TwitchAPIBase {
    fn start_raid(
        &self,
        from_broadcaster_id: &str,
        to_broadcaster_id: &str,
    ) -> TwitchAPIRequest<EmptyBody>;
    fn cancel_raid(&self, broadcaster_id: BroadcasterId) -> TwitchAPIRequest<EmptyBody>;
}

impl RaidAPI for TwitchAPI {
    fn start_raid(
        &self,
        from_broadcaster_id: &str,
        to_broadcaster_id: &str,
    ) -> TwitchAPIRequest<EmptyBody> {
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
    fn cancel_raid(&self, broadcaster_id: BroadcasterId) -> TwitchAPIRequest<EmptyBody> {
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
