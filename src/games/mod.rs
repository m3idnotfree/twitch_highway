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

twitch_api_trait! {
    #[cfg_attr(docsrs, doc(cfg(feature = "games")))]
    trait GamesAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#get-top-games>
        fn get_top_games(
            &self,
            pagination: Option<PaginationQuery>,
        ) -> GamesResponse;
        /// <https://dev.twitch.tv/docs/api/reference/#get-games>
        fn get_games(
            &self,
            request: GetGamesRequest,
        ) -> GamesResponse;
    }
    impl {
        get_top_games => {
            endpoint_type: EndpointType::GetTopGames,
            method: Method::GET,
            path: [GAMES, "top"],
            query_params: {
                pagination(pagination)
            }
        }
        get_games => {
            endpoint_type: EndpointType::GetGames,
            method: Method::GET,
            path: [GAMES],
            query_params: {
                into_query(request)
            }
        }
    }
}

#[cfg(test)]
mod games_api_tests {
    use crate::{
        games::{request::GetGamesRequest, GamesAPI},
        test_utils::TwitchApiTest,
        types::{Id, PaginationQuery},
    };

    #[tokio::test]
    async fn get_top_games_endpoint() {
        let suite = TwitchApiTest::new().await;

        suite.mock_games_success().await;

        let pagination = PaginationQuery::new().first(20);

        let response = suite
            .execute("/games/top", |api| api.get_top_games(Some(pagination)))
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 3);
        assert!(response.pagination.is_some());

        let top_game = &response.data[0];
        assert_eq!(top_game.id.as_str(), "21779");
        assert_eq!(top_game.name, "League of Legends");
        assert!(top_game.box_art_url.is_some());
        assert_eq!(top_game.igdb_id.as_ref().unwrap(), "115");

        let second_game = &response.data[1];
        assert_eq!(second_game.id.as_str(), "509658");
        assert_eq!(second_game.name, "Just Chatting");
        assert!(second_game.igdb_id.is_none());

        let third_game = &response.data[2];
        assert_eq!(third_game.id.as_str(), "516575");
        assert_eq!(third_game.name, "VALORANT");
        assert_eq!(third_game.igdb_id.as_ref().unwrap(), "25646");
    }

    #[tokio::test]
    async fn get_top_games_no_pagination_endpoint() {
        let suite = TwitchApiTest::new().await;

        suite.mock_games_success().await;

        let response = suite
            .execute("/games/top", |api| api.get_top_games(None))
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 1);
        assert!(response.pagination.is_none());

        let game = &response.data[0];
        assert_eq!(game.id.as_str(), "32982");
        assert_eq!(game.name, "Grand Theft Auto V");
        assert_eq!(game.igdb_id.as_ref().unwrap(), "3276");
    }

    #[tokio::test]
    async fn get_games_by_ids_endpoint() {
        let suite = TwitchApiTest::new().await;

        suite.mock_games_success().await;

        let request = GetGamesRequest::new().ids(vec![Id::new("511224"), Id::new("138585")]);

        let response = suite
            .execute("/games", |api| api.get_games(request))
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 2);
        assert!(response.pagination.is_none());

        let apex = &response.data[0];
        assert_eq!(apex.id.as_str(), "511224");
        assert_eq!(apex.name, "Apex Legends");
        assert_eq!(apex.igdb_id.as_ref().unwrap(), "25657");

        let hearthstone = &response.data[1];
        assert_eq!(hearthstone.id.as_str(), "138585");
        assert_eq!(hearthstone.name, "Hearthstone");
        assert_eq!(hearthstone.igdb_id.as_ref().unwrap(), "1465");
    }

    #[tokio::test]
    async fn get_games_by_names_endpoint() {
        let suite = TwitchApiTest::new().await;

        suite.mock_games_success().await;

        let request =
            GetGamesRequest::new().names(vec!["Fortnite".to_string(), "Minecraft".to_string()]);

        let response = suite
            .execute("/games", |api| api.get_games(request))
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 2);

        let fortnite = &response.data[0];
        assert_eq!(fortnite.id.as_str(), "33214");
        assert_eq!(fortnite.name, "Fortnite");
        assert_eq!(fortnite.igdb_id.as_ref().unwrap(), "1905");

        let minecraft = &response.data[1];
        assert_eq!(minecraft.id.as_str(), "27471");
        assert_eq!(minecraft.name, "Minecraft");
        assert_eq!(minecraft.igdb_id.as_ref().unwrap(), "19");
    }

    #[tokio::test]
    async fn get_games_by_igdb_ids_endpoint() {
        let suite = TwitchApiTest::new().await;

        suite.mock_games_success().await;

        let request =
            GetGamesRequest::new().igdb_ids(vec!["743".to_string(), "512710".to_string()]);

        let response = suite
            .execute("/games", |api| api.get_games(request))
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 2);

        let chess = &response.data[0];
        assert_eq!(chess.id.as_str(), "743");
        assert_eq!(chess.name, "Chess");
        assert!(chess.igdb_id.is_none());

        let warzone = &response.data[1];
        assert_eq!(warzone.id.as_str(), "512710");
        assert_eq!(warzone.name, "Call of Duty: Warzone");
    }

    #[tokio::test]
    async fn get_games_mixed_parameters_endpoint() {
        let suite = TwitchApiTest::new().await;

        suite.mock_games_success().await;

        let request = GetGamesRequest::new()
            .ids(vec![Id::new("21779")])
            .names(vec!["VALORANT".to_string()])
            .igdb_ids(vec!["1905".to_string()]);

        let response = suite
            .execute("/games", |api| api.get_games(request))
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 3);

        let games: Vec<&str> = response.data.iter().map(|g| g.name.as_str()).collect();
        assert!(games.contains(&"League of Legends"));
        assert!(games.contains(&"VALORANT"));
        assert!(games.contains(&"Fortnite"));
    }

    #[tokio::test]
    async fn games_api_error_response() {
        let suite = TwitchApiTest::new().await;

        suite.mock_games_failure().await;

        let request = GetGamesRequest::new();

        let response = suite
            .execute("/games", |api| api.get_games(request))
            .send()
            .await;

        match response {
            Ok(response) => {
                panic!("Expected Error, got: {response:?}")
            }
            Err(e) => {
                assert!(e.is_api());
            }
        }
    }
}
