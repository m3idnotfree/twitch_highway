use serde::Serialize;

use crate::{
    base::{IntoQueryPairs, QueryParams},
    types::{
        constants::{STARTED_AT, USER_ID},
        UserId,
    },
};

request_struct!(
    #[derive(Serialize)]
    BitsLeaderboardRequest {
        string {
            period: String,
            started_at: String,
        },
        any {
            user_id: UserId,
            count: u64
        }
    }
);

impl IntoQueryPairs for BitsLeaderboardRequest {
    fn into_query_pairs(self) -> Vec<(&'static str, String)> {
        let mut params = QueryParams::new();
        params
            .push_opt("count", self.count.map(|x| x.to_string()))
            .push_opt("period", self.period)
            .push_opt(STARTED_AT, self.started_at)
            .push_opt(USER_ID, self.user_id);
        params.build()
    }
}

//request_struct!(
//    #[derive(Serialize)]
//    ExtensionTransactionRequest {
//        required {
//            extension_id: ExtensionId
//        },
//        optional {
//            id: Id,
//            first: String,
//            after: String,
//        }
//    }
//);
//
//impl IntoQueryPairs for ExtensionTransactionRequest {
//    fn into_query_pairs(self) -> Vec<(&'static str, String)> {
//        let mut params = QueryParams::new();
//
//        params
//            .push(EXTENSION_ID, self.extension_id)
//            .push_opt(ID, self.id)
//            .push_opt(FIRST, self.first)
//            .push_opt(AFTER, self.after);
//
//        params.build()
//    }
//}
