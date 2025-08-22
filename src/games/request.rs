use serde::Serialize;

use crate::types::Id;

define_request!(
    #[derive(Serialize)]
    GetGamesRequest<'a> {
        opts: {
            ids: &'a [Id] => ID ; vec,
            names: &'a [&'a str] => "name" ; vec,
            igdb_ids: &'a [&'a str] => "igdb_id" ; vec,
        };
        into_query
    }
);
