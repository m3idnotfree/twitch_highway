use asknothingx2_util::api::Method;
use request::GetGamesRequest;
use response::GamesResponse;

use crate::{
    request::{EmptyBody, EndpointType, TwitchAPIRequest},
    types::{constants::GAMES, PaginationQuery},
    TwitchAPI,
};

pub mod request;
pub mod response;

#[cfg_attr(docsrs, doc(cfg(feature = "games")))]
pub trait GamesAPI {
    /// <https://dev.twitch.tv/docs/api/reference/#get-top-games>
    fn get_top_games(
        &self,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, GamesResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#get-games>
    fn get_games(&self, request: GetGamesRequest) -> TwitchAPIRequest<EmptyBody, GamesResponse>;
}

impl GamesAPI for TwitchAPI {
    fn get_top_games(
        &self,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, GamesResponse> {
        let mut url = self.build_url();
        url.path([GAMES, "top"]).query_opt_pairs(pagination);

        TwitchAPIRequest::new(
            EndpointType::GetTopGames,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn get_games(&self, request: GetGamesRequest) -> TwitchAPIRequest<EmptyBody, GamesResponse> {
        let mut url = self.build_url();
        url.path([GAMES]).query_pairs(request);

        TwitchAPIRequest::new(
            EndpointType::GetGames,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
}
