use asknothingx2_util::api::Method;
use request::GetGamesRequest;
use response::GamesResponse;

use crate::{
    base::TwitchAPIBase, types::PaginationQuery, EmptyBody, EndpointType, TwitchAPI,
    TwitchAPIRequest,
};

pub mod request;
pub mod response;
pub mod types;

pub trait GamesAPI: TwitchAPIBase {
    fn get_top_games(
        &self,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, GamesResponse>;
    fn get_games(&self, request: GetGamesRequest) -> TwitchAPIRequest<EmptyBody, GamesResponse>;
}

impl GamesAPI for TwitchAPI {
    fn get_top_games(
        &self,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, GamesResponse> {
        let mut url = self.build_url();
        url.path(["games", "top"]).query_opt_pairs(pagination);

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
        url.path(["games"]).query_pairs(request);

        TwitchAPIRequest::new(
            EndpointType::GetGames,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
}
