mod builder;
mod response;
mod types;

pub use builder::{GetDropsEntitlements, UpdateDropsEntitlements};
pub use response::{DropsEntitlementsResponse, UpdateDropEntitlementsResponse};
pub use types::{DropEntitlement, DropEntitlementStatus, FulfillmentStatus, UpdateDropEntitlement};

use crate::Client;

pub trait EntitlementsAPI {
    /// See <https://dev.twitch.tv/docs/api/reference/#get-drops-entitlements>
    fn get_drops_entitlements<'a>(&'a self) -> GetDropsEntitlements<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#update-drops-entitlements>
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
