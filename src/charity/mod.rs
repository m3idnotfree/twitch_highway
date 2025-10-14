mod builder;
mod response;
mod types;

pub use builder::GetCharityCampaignDonationBuilder;
pub use response::{CharityCampaignDonationResponse, CharityCampaignResponse};
pub use types::{CharityCampaign, CharityCampaignDonation};

use crate::{
    request::TwitchAPIRequest,
    types::{constants::CHARITY, BroadcasterId},
    TwitchAPI,
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
    /// # use twitch_highway::TwitchAPI;
    /// use twitch_highway::{
    ///     charity::CharityAPI,
    ///     types::BroadcasterId
    /// };
    ///
    /// # async fn example(api: TwitchAPI) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .get_charity_campaign(&BroadcasterId::from("1234"))
    ///     .json()
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
    ) -> TwitchAPIRequest<CharityCampaignResponse>;

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
    /// # use twitch_highway::TwitchAPI;
    /// use twitch_highway::{
    ///     charity::CharityAPI,
    ///     types::BroadcasterId
    /// };
    ///
    /// # async fn example(api: TwitchAPI) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .get_charity_campaign_donations(&BroadcasterId::from("1234"))
    ///     .json()
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

impl CharityAPI for TwitchAPI {
    simple_endpoint!(
        fn get_charity_campaign(
            broadcaster_id: &BroadcasterId,
        ) -> CharityCampaignResponse;
            endpoint: GetCharityCampaign,
            method: GET,
            path: [CHARITY, "campaigns"],
    );
    fn get_charity_campaign_donations<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
    ) -> GetCharityCampaignDonationBuilder<'a> {
        GetCharityCampaignDonationBuilder::new(self, broadcaster_id)
    }
}
