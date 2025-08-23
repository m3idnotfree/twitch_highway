pub mod request;
pub mod response;
pub mod types;

use asknothingx2_util::api::Method;
use request::BitsLeaderboardRequest;
use response::{BitsLeaderboardResponse, CheermotesResponse, ExtensionTransactionsResponse};

use crate::{
    request::EndpointType,
    types::{
        constants::{BITS, BROADCASTER_ID, EXTENSIONS, EXTENSION_ID, ID},
        BroadcasterId, ExtensionId, Id, PaginationQuery,
    },
};

endpoints! {
    BitsAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#get-bits-leaderboard>
        fn get_bits_leaderboard(
            &self,
            opts: Option<BitsLeaderboardRequest>,
        ) -> BitsLeaderboardResponse {
            endpoint_type: EndpointType::GetBitsLeaderboard,
            method: Method::GET,
            path: [BITS, "leaderboard"],
            query_params: {
                opt_into_query(opts)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#get-cheermotes>
        fn get_cheermotes(
            &self,
            broadcaster_id: Option<&BroadcasterId>,
        ) -> CheermotesResponse {
            endpoint_type: EndpointType::GetCheermotes,
            method: Method::GET,
            path: [BITS, "cheermotes"],
            query_params: {
                opt(BROADCASTER_ID, broadcaster_id)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#get-extension-transactions>
        fn get_extension_transactions(
            &self,
            extension_id: &ExtensionId,
            id: Option<&Id>,
            pagination: Option<PaginationQuery>,
        ) -> ExtensionTransactionsResponse {
            endpoint_type: EndpointType::GetExtensionTransactions,
            method: Method::GET,
            path: [EXTENSIONS, "transactions"],
            query_params: {
                query(EXTENSION_ID, extension_id),
                opt(ID, id),
                pagination(pagination)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        bits::{request::BitsLeaderboardRequest, BitsAPI},
        types::{BroadcasterId, ExtensionId},
    };

    api_test!(
        get_bits_leaderboard,
        [Some(
            BitsLeaderboardRequest::new()
                .count(2)
                .period("week")
                .started_at(&"2018-02-05T08:00:00Z".parse().unwrap())
        )]
    );
    api_test!(get_cheermotes, [Some(&BroadcasterId::from("41245072"))]);
    api_test!(
        get_extension_transactions,
        [&ExtensionId::from("1234"), None, None]
    );

    api_test!(extra
        get_cheermotes, 
        get_cheermotes2, 
        [Some(&BroadcasterId::from("41245072"))]);
}
