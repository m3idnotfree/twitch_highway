use asknothingx2_util::api::Method;
use request::GameAnalyticsRequest;

use crate::{base::TwitchAPIBase, EmptyBody, EndpointType, TwitchAPI, TwitchAPIRequest};

pub mod request;
pub mod response;
pub mod types;

const ANALYTICS: &str = "analytics";

pub trait AnalyticsAPI: TwitchAPIBase {
    /// https://dev.twitch.tv/docs/api/reference/#get-extension-analytics
    fn get_extension_analytics(&self) -> TwitchAPIRequest<EmptyBody>;
    /// https://dev.twitch.tv/docs/api/reference/#get-game-analytics
    fn get_game_analytics(&self, request: GameAnalyticsRequest) -> TwitchAPIRequest<EmptyBody>;
}

impl AnalyticsAPI for TwitchAPI {
    fn get_extension_analytics(&self) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path([ANALYTICS, "extensions"]);

        TwitchAPIRequest::new(
            EndpointType::GetExtensionAnalytics,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn get_game_analytics(&self, request: GameAnalyticsRequest) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path([ANALYTICS, "games"]).query_pairs(request);

        TwitchAPIRequest::new(
            EndpointType::GetGameAnalytics,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
}
