use asknothingx2_util::api::Method;
use request::BitsLeaderboardRequest;
use response::{BitsLeaderboardResponse, CheermotesResponse, ExtensionTransactionsResponse};

use crate::{
    request::{EndpointType, TwitchAPIRequest},
    types::{
        constants::{BITS, BROADCASTER_ID, EXTENSIONS, EXTENSION_ID, ID},
        BroadcasterId, ExtensionId, Id, PaginationQuery,
    },
    TwitchAPI,
};

pub mod request;
pub mod response;
pub mod types;

#[cfg_attr(docsrs, doc(cfg(feature = "bits")))]
pub trait BitsAPI {
    /// <https://dev.twitch.tv/docs/api/reference/#get-bits-leaderboard>
    fn get_bits_leaderboard(
        &self,
        opts: Option<BitsLeaderboardRequest>,
    ) -> TwitchAPIRequest<BitsLeaderboardResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#get-cheermotes>
    fn get_cheermotes(
        &self,
        broadcaster_id: Option<BroadcasterId>,
    ) -> TwitchAPIRequest<CheermotesResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#get-extension-transactions>
    fn get_extension_transactions(
        &self,
        extension_id: ExtensionId,
        id: Option<Id>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<ExtensionTransactionsResponse>;
}

impl BitsAPI for TwitchAPI {
    fn get_bits_leaderboard(
        &self,
        opts: Option<BitsLeaderboardRequest>,
    ) -> TwitchAPIRequest<BitsLeaderboardResponse> {
        let mut url = self.build_url();
        url.path([BITS, "leaderboard"]);

        if let Some(opts) = opts {
            opts.apply_to_url(&mut url);
        }

        TwitchAPIRequest::new(
            EndpointType::GetBitsLeaderboard,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            None,
        )
    }
    fn get_cheermotes(
        &self,
        broadcaster_id: Option<BroadcasterId>,
    ) -> TwitchAPIRequest<CheermotesResponse> {
        let mut url = self.build_url();
        url.path([BITS, "cheermotes"])
            .query_opt(BROADCASTER_ID, broadcaster_id);

        TwitchAPIRequest::new(
            EndpointType::GetCheermotes,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            None,
        )
    }
    fn get_extension_transactions(
        &self,
        extension_id: ExtensionId,
        id: Option<Id>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<ExtensionTransactionsResponse> {
        let mut url = self.build_url();
        url.path([EXTENSIONS, "transactions"])
            .query(EXTENSION_ID, extension_id)
            .query_opt(ID, id);

        if let Some(pagination) = pagination {
            pagination.apply_to_url(&mut url);
        }

        TwitchAPIRequest::new(
            EndpointType::GetExtensionTransactions,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            None,
        )
    }
}
