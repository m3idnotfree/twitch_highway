use serde::{Deserialize, Serialize};

use crate::{
    base::{IntoQueryPairs, QueryParams},
    types::{constants::ID, Id},
};

request_struct!(
    #[derive(Serialize, Deserialize)]
    GetGamesRequest {
        ids: Vec<Id>,
        names: Vec<String>,
        igdb_ids: Vec<String>,
    }
);

impl IntoQueryPairs for GetGamesRequest {
    fn into_query_pairs(self) -> Vec<(&'static str, String)> {
        let mut params = QueryParams::new();

        params
            .extend_opt(self.ids.map(|x| x.into_iter().map(|id| (ID, id))))
            .extend_opt(self.names.map(|x| x.into_iter().map(|name| ("name", name))))
            .extend_opt(
                self.igdb_ids
                    .map(|x| x.into_iter().map(|igdb_id| ("igdb_id", igdb_id))),
            );

        params.build()
    }
}
