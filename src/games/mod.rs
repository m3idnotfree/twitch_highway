use asknothingx2_util::api::Method;

use crate::{
    base::TwitchAPIBase,
    types::{Id, AFTER, FIRST, ID},
    EmptyBody, EndpointType, TwitchAPI, TwitchAPIRequest,
};

pub mod response;
pub mod types;

pub trait GamesAPI: TwitchAPIBase {
    fn get_top_games(
        &self,
        first: Option<u64>,
        after: Option<&str>,
        before: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody>;
    fn get_games(
        &self,
        ids: Option<&[Id]>,
        names: Option<&[&str]>,
        igdb_ids: Option<&[&str]>,
    ) -> TwitchAPIRequest<EmptyBody>;
}

impl GamesAPI for TwitchAPI {
    fn get_top_games(
        &self,
        first: Option<u64>,
        after: Option<&str>,
        before: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path(["games", "top"])
            .query_opt(FIRST, first.map(|x| x.to_string()))
            .query_opt(AFTER, after)
            .query_opt("before", before);

        TwitchAPIRequest::new(
            EndpointType::GetTopGames,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn get_games(
        &self,
        ids: Option<&[Id]>,
        names: Option<&[&str]>,
        igdb_ids: Option<&[&str]>,
    ) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path(["games"])
            .query_opt_extend(ids.map(|x| x.iter().map(|id| (ID, id))))
            .query_opt_extend(names.map(|x| x.iter().map(|name| ("name", name))))
            .query_opt_extend(igdb_ids.map(|x| x.iter().map(|igdb_id| ("igdb_id", igdb_id))));

        TwitchAPIRequest::new(
            EndpointType::GetGames,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
}
