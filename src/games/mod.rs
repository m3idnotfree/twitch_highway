pub mod request;
pub mod response;

use request::GetGamesRequest;
use response::GamesResponse;

use crate::types::{constants::GAMES, PaginationQuery};

endpoints! {
    GamesAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#get-top-games>
        fn get_top_games(
            &self,
            pagination: Option<PaginationQuery>,
        ) -> GamesResponse {
            endpoint_type: GetTopGames,
            method: GET,
            path: [GAMES, "top"],
            query_params: {
                pagination(pagination)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#get-games>
        fn get_games(
            &self,
            request: GetGamesRequest,
        ) -> GamesResponse {
            endpoint_type: GetGames,
            method: GET,
            path: [GAMES],
            query_params: {
                into_query(request)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        games::{request::GetGamesRequest, GamesAPI},
        types::Id,
    };

    api_test!(get_top_games, [None]);
    api_test!(
        get_games,
        [GetGamesRequest::new().ids(&[Id::from("33214")])]
    );
}
