use asknothingx2_util::api::Method;
use response::{CharityCampaignDonationResponse, CharityCampaignResponse};

use crate::{
    request::{EmptyBody, EndpointType, TwitchAPIRequest},
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
    ) -> TwitchAPIRequest<EmptyBody, CharityCampaignResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#get-charity-campaign-donations>
    fn get_charity_campaign_donations(
        &self,
        broadcaster_id: BroadcasterId,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, CharityCampaignDonationResponse>;
}

impl CharityAPI for TwitchAPI {
    fn get_charity_campaign(
        &self,
        broadcaster_id: BroadcasterId,
    ) -> TwitchAPIRequest<EmptyBody, CharityCampaignResponse> {
        let mut url = self.build_url();
        url.path([CHARITY, "campaigns"])
            .query(BROADCASTER_ID, broadcaster_id);

        TwitchAPIRequest::new(
            EndpointType::GetCharityCampaign,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn get_charity_campaign_donations(
        &self,
        broadcaster_id: BroadcasterId,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, CharityCampaignDonationResponse> {
        let mut url = self.build_url();
        url.path([CHARITY, "donations"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query_opt_pairs(pagination);

        TwitchAPIRequest::new(
            EndpointType::GetCharityCampaignDonations,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
}
