use chrono::{DateTime, Utc};

use crate::{
    bits::{BitsLeaderboardResponse, ExtensionTransactionsResponse},
    types::{
        constants::{
            AFTER, BITS, COUNT, EXTENSIONS, EXTENSION_ID, FIRST, ID, LEADERBOARD, PERIOD,
            STARTED_AT, TRANSACTIONS, USER_ID,
        },
        ExtensionId, TransactionId, UserId,
    },
};

use crate::bits::Period;

define_request_builder! {
    #[derive(Debug)]
    BitsLeaderboardRequest<'a> {
        /// min 1, max 100, default 10
        count: u8 [key = COUNT, convert = to_string],
        period: Period [key = PERIOD, convert = as_ref],
        started_at: &'a DateTime<Utc>[key = STARTED_AT, convert = rfc3339_opt],
        user_id: &'a UserId [key = USER_ID]
    } -> BitsLeaderboardResponse;
        endpoint_type: GetBitsLeaderboard,
        method: GET,
        path: [BITS, LEADERBOARD],
}

define_request_builder! {
    GetExtensionTransactionsBuilder<'a>{
        req: {extension_id: &'a ExtensionId [key = EXTENSION_ID, convert = as_ref]},
        opts: {
            ids: &'a [TransactionId] [key = ID, convert = extend],
            first: u8 [key = FIRST, convert = to_string],
            after: &'a str [key = AFTER]
        }
    } -> ExtensionTransactionsResponse;
        endpoint_type: GetExtensionTransactions,
        method: GET,
        path: [EXTENSIONS, TRANSACTIONS],

}
