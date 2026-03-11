mod builder;
mod response;
mod types;

pub use builder::GetCharityCampaignDonationBuilder;
pub use response::{CharityCampaignDonationResponse, CharityCampaignResponse};
pub use types::{CharityCampaign, CharityCampaignDonation};

use std::future::Future;

use crate::{
    types::{
        constants::{BROADCASTER_ID, CHARITY},
        BroadcasterId,
    },
    Client, Error,
};

pub trait CharityAPI {
    /// Gets information about the charity campaign that a broadcaster is running
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` -
    ///
    /// # Returns
    ///
    /// Returns a [`CharityCampaignResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     charity::CharityAPI,
    ///     types::BroadcasterId
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_charity_campaign(&BroadcasterId::from("1234"))
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `channel:read:charity`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-charity-campaign>
    fn get_charity_campaign(
        &self,
        broadcaster_id: &BroadcasterId,
    ) -> impl Future<Output = Result<CharityCampaignResponse, Error>> + Send;

    /// Gets the list of donations that users have made to the broadcaster’s active charity campaign
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` -
    ///
    /// # Returns
    ///
    /// Returns a [`GetCharityCampaignDonationBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     charity::CharityAPI,
    ///     types::BroadcasterId
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_charity_campaign_donations(&BroadcasterId::from("1234"))
    ///     .send()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `channel:read:charity`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-charity-campaign-donations>
    fn get_charity_campaign_donations<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
    ) -> GetCharityCampaignDonationBuilder<'a>;
}

impl CharityAPI for Client {
    async fn get_charity_campaign(
        &self,
        broadcaster_id: &BroadcasterId,
    ) -> Result<CharityCampaignResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([CHARITY, "campaigns"]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id);

        self.json(self.http_client().get(url)).await
    }

    fn get_charity_campaign_donations<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
    ) -> GetCharityCampaignDonationBuilder<'a> {
        GetCharityCampaignDonationBuilder::new(self, broadcaster_id)
    }
}
