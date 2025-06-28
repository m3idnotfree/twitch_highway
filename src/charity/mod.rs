use asknothingx2_util::api::Method;
use response::{CharityCampaignDonationResponse, CharityCampaignResponse};

use crate::{
    request::{EndpointType, TwitchAPIRequest},
    types::{
        constants::{BROADCASTER_ID, CHARITY},
        BroadcasterId, PaginationQuery,
    },
    TwitchAPI,
};

pub mod response;
pub mod types;

#[cfg_attr(docsrs, doc(cfg(feature = "charity")))]
pub trait CharityAPI {
    /// <https://dev.twitch.tv/docs/api/reference/#get-charity-campaign>
    fn get_charity_campaign(
        &self,
        broadcaster_id: BroadcasterId,
    ) -> TwitchAPIRequest<CharityCampaignResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#get-charity-campaign-donations>
    fn get_charity_campaign_donations(
        &self,
        broadcaster_id: BroadcasterId,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<CharityCampaignDonationResponse>;
}

impl CharityAPI for TwitchAPI {
    fn get_charity_campaign(
        &self,
        broadcaster_id: BroadcasterId,
    ) -> TwitchAPIRequest<CharityCampaignResponse> {
        let mut url = self.build_url();
        url.path([CHARITY, "campaigns"])
            .query(BROADCASTER_ID, broadcaster_id);

        TwitchAPIRequest::new(
            EndpointType::GetCharityCampaign,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            None,
        )
    }
    fn get_charity_campaign_donations(
        &self,
        broadcaster_id: BroadcasterId,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<CharityCampaignDonationResponse> {
        let mut url = self.build_url();
        url.path([CHARITY, "donations"])
            .query(BROADCASTER_ID, broadcaster_id);
        if let Some(pagination) = pagination {
            pagination.apply_to_url(&mut url);
        }

        TwitchAPIRequest::new(
            EndpointType::GetCharityCampaignDonations,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            None,
        )
    }
}
