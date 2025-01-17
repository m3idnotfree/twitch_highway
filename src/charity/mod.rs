use crate::{
    base::TwitchAPIBase,
    types::{
        constants::{BROADCASTER_ID, CHARITY},
        BroadcasterId, PaginationQuery,
    },
    EmptyBody, EndpointType, TwitchAPI, TwitchAPIRequest,
};
use asknothingx2_util::api::Method;
use response::{CharityCampaignDonationResponse, CharityCampaignResponse};

pub mod response;
pub mod types;

pub trait CharityAPI: TwitchAPIBase {
    fn get_charity_campaign(
        &self,
        broadcaster_id: BroadcasterId,
    ) -> TwitchAPIRequest<EmptyBody, CharityCampaignResponse>;
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
