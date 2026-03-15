mod builder;
mod response;
mod types;

pub use builder::{GetBitsLeaderboard, GetExtensionTransactions};
pub use response::{BitsLeaderboardResponse, CheermotesResponse, ExtensionTransactionsResponse};
pub use types::{
    BitsLeaderboard, Cheermotes, Dark, ExtensionTransaction, Images, Imagess, Light, Period,
    ProductType, Tier, TierLevel, TransactionProductData, Type,
};

use std::future::Future;

use crate::{
    Error,
    types::{
        BroadcasterId, ExtensionId,
        constants::{BITS, BROADCASTER_ID, CHEERMOTES},
    },
};

pub trait BitsAPI {
    /// See <https://dev.twitch.tv/docs/api/reference/#get-bits-leaderboard>
    fn get_bits_leaderboard<'a>(&'a self) -> GetBitsLeaderboard<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#get-cheermotes>
    fn get_cheermotes(
        &self,
        broadcaster_id: Option<&BroadcasterId>,
    ) -> impl Future<Output = Result<CheermotesResponse, Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#get-extension-transactions>
    fn get_extension_transactions<'a>(
        &'a self,
        extension_id: &'a ExtensionId,
    ) -> GetExtensionTransactions<'a>;
}

impl BitsAPI for crate::Client {
    fn get_bits_leaderboard<'a>(&'a self) -> GetBitsLeaderboard<'a> {
        GetBitsLeaderboard::new(self)
    }

    async fn get_cheermotes(
        &self,
        broadcaster_id: Option<&BroadcasterId>,
    ) -> Result<CheermotesResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().extend([BITS, CHEERMOTES]);

        if let Some(broadcaster_id) = broadcaster_id {
            url.query_pairs_mut()
                .append_pair(BROADCASTER_ID, broadcaster_id);
        }

        self.json(self.http_client().get(url)).await
    }

    fn get_extension_transactions<'a>(
        &'a self,
        extension_id: &'a ExtensionId,
    ) -> GetExtensionTransactions<'a> {
        GetExtensionTransactions::new(self, extension_id)
    }
}
