use asknothingx2_util::api::Method;
use response::HypeTrainResponse;

use crate::{
    request::{EndpointType, TwitchAPIRequest},
    types::{constants::BROADCASTER_ID, BroadcasterId, PaginationQuery},
    TwitchAPI,
};

pub mod response;
pub mod types;

#[cfg_attr(docsrs, doc(cfg(feature = "hype-train")))]
pub trait HypeTrainAPI {
    /// <https://dev.twitch.tv/docs/api/reference/#get-hype-train-events>
    fn get_hype_train_events(
        &self,
        broadcaster_id: BroadcasterId,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<HypeTrainResponse>;
}

impl HypeTrainAPI for TwitchAPI {
    fn get_hype_train_events(
        &self,
        broadcaster_id: BroadcasterId,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<HypeTrainResponse> {
        let mut url = self.build_url();
        url.path(["hypetrain", "events"])
            .query(BROADCASTER_ID, broadcaster_id);
        if let Some(pagination) = pagination {
            pagination.apply_to_url(&mut url);
        }

        TwitchAPIRequest::new(
            EndpointType::GetHypeTrainEvents,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            None,
        )
    }
}
