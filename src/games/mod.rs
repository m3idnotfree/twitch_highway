use asknothingx2_util::api::Method;
use request::GetGamesRequest;
use response::GamesResponse;

use crate::{
    request::{EndpointType, TwitchAPIRequest},
    types::{constants::GAMES, PaginationQuery},
    TwitchAPI,
};

pub mod request;
pub mod response;

#[cfg_attr(docsrs, doc(cfg(feature = "games")))]
pub trait GamesAPI {
    /// <https://dev.twitch.tv/docs/api/reference/#get-top-games>
    fn get_top_games(&self, pagination: Option<PaginationQuery>)
        -> TwitchAPIRequest<GamesResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#get-games>
    fn get_games(&self, request: GetGamesRequest) -> TwitchAPIRequest<GamesResponse>;
}

impl GamesAPI for TwitchAPI {
    fn get_top_games(
        &self,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<GamesResponse> {
        let mut url = self.build_url();
        url.path([GAMES, "top"]);

        if let Some(pagination) = pagination {
            pagination.apply_to_url(&mut url);
        }

        TwitchAPIRequest::new(
            EndpointType::GetTopGames,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            None,
        )
    }
    fn get_games(&self, request: GetGamesRequest) -> TwitchAPIRequest<GamesResponse> {
        let mut url = self.build_url();
        url.path([GAMES]);
        request.apply_to_url(&mut url);

        TwitchAPIRequest::new(
            EndpointType::GetGames,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            None,
        )
    }
}
