mod builder;
mod response;
mod types;

pub use builder::{GetDropsEntitlements, UpdateDropsEntitlements};
pub use response::{DropsEntitlementsResponse, UpdateDropEntitlementsResponse};
pub use types::{DropEntitlement, DropEntitlementStatus, FulfillmentStatus, UpdateDropEntitlement};

use crate::Client;

pub trait EntitlementsAPI {
    /// Gets an organization’s list of entitlements that have been granted to a game, a user, or both
    ///
    /// # Returns
    ///
    /// Returns a [`DropsEntitlementsResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     entitlements::{EntitlementsAPI, FulfillmentStatus},
    ///     types::{EntitlementId, UserId, GameId}
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_drops_entitlements()
    ///     .ids(&[EntitlementId::from("1234")])
    ///     .user_id(&UserId::from("5678"))
    ///     .game_id(&GameId::from("7890"))
    ///     .fulfillment_status(FulfillmentStatus::CLAIMED)
    ///     .first(5)
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
    /// <https://dev.twitch.tv/docs/api/reference/#get-drops-entitlements>
    fn get_drops_entitlements<'a>(&'a self) -> GetDropsEntitlements<'a>;

    /// Updates the Drop entitlement’s fulfillment status
    ///
    /// # Returns
    ///
    /// Returns a [`UpdateDropEntitlementsResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     entitlements::{EntitlementsAPI, FulfillmentStatus},
    ///     types::EntitlementId
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .update_drops_entitlements()
    ///     .entitlement_ids(&[EntitlementId::from("1234")])
    ///     .fulfillment_status(FulfillmentStatus::CLAIMED)
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
    /// <https://dev.twitch.tv/docs/api/reference/#update-drops-entitlements>
    fn update_drops_entitlements<'a>(&'a self) -> UpdateDropsEntitlements<'a>;
}

impl EntitlementsAPI for Client {
    fn get_drops_entitlements<'a>(&'a self) -> GetDropsEntitlements<'a> {
        GetDropsEntitlements::new(self)
    }

    fn update_drops_entitlements<'a>(&'a self) -> UpdateDropsEntitlements<'a> {
        UpdateDropsEntitlements::new(self)
    }
}
