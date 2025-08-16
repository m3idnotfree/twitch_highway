use serde::{Deserialize, Serialize};

use crate::types::Id;

define_request!(
    #[derive(Serialize, Deserialize)]
    GetGamesRequest {
        opts: {
            ids: Vec<Id> => ID ; vec,
            names: Vec<String> => "name" ; vec,
            igdb_ids: Vec<String> => "igdb_id" ; vec,
        };
        into_query
    }
);
