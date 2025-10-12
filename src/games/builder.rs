use crate::{
    games::GamesResponse,
    types::{
        constants::{AFTER, BEFORE, FIRST, GAMES, ID, IGDB_ID, NAME, TOP},
        Id,
    },
};

define_request_builder! {
    #[derive(Debug)]
    GetTopGamesBuilder<'a> {
        first: u8 [key = FIRST, convert = to_string],
        after: &'a str [key = AFTER],
        before: &'a str [key = BEFORE]
    } -> GamesResponse;
    endpoint_type: GetTopGames,
    method: GET,
    path: [GAMES, TOP],
}

define_request_builder! {
    #[derive(Debug)]
    GetGamesBuilder<'a> {
        ids: &'a [Id] [key = ID, convert = extend],
        names: &'a [&'a str] [key = NAME, convert = extend],
        igdb_ids: &'a [&'a str] [key = IGDB_ID, convert = extend],
    } -> GamesResponse;
    endpoint_type: GetGames,
    method: GET,
    path: [GAMES],
}
