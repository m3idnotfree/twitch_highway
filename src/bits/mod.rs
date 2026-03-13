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
    types::{
        constants::{BITS, BROADCASTER_ID, CHEERMOTES},
        BroadcasterId, ExtensionId,
    },
    Error,
};

pub trait BitsAPI {
    /// Gets the Bits leaderboard for the authenticated broadcaster
    ///
    /// # Returns
    ///
    /// Returns a [`BitsLeaderboardRequest`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     bits::{BitsAPI, Period},
    ///     types::UserId,
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_bits_leaderboard()
    ///     .count(50)
    ///     .period(Period::Week)
    ///     .started_at(&"2018-01-01T00:00:00Z".parse().unwrap())
    ///     .user_id(&UserId::from("1234"))
    ///     .send()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `bits:read`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-bits-leaderboard>
    fn get_bits_leaderboard<'a>(&'a self) -> GetBitsLeaderboard<'a>;

    /// Gets a list of Cheermotes that users can use to cheer Bits in any Bits-enabled channel’s chat room
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - Optional The ID of the broadcaster whose custom Cheermotes you want to get.
    ///
    /// # Returns
    ///
    /// Returns a [`CheermotesResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     bits::BitsAPI,
    ///     types::BroadcasterId,
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_cheermotes(Some(&BroadcasterId::from("1234")))
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// No scope required
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-cheermotes>
    fn get_cheermotes(
        &self,
        broadcaster_id: Option<&BroadcasterId>,
    ) -> impl Future<Output = Result<CheermotesResponse, Error>> + Send;

    /// Gets an extension’s list of transactions
    ///
    /// # Returns
    ///
    /// Returns a [`GetExtensionTransactionsBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     bits::BitsAPI,
    ///     types::{ExtensionId, TransactionId}
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_extension_transactions(&ExtensionId::from("1234"))
    ///     .ids(&[TransactionId::from("5678"), TransactionId::from("6789")])
    ///     .first(50)
    ///     .after("eyJiI...")
    ///     .send()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// No scope required
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-extension-transactions>
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
