use crate::{
    base::TwitchAPIBase,
    types::{BroadcasterId, AFTER, BROADCASTER_ID, FIRST},
    EmptyBody, EndpointType, TwitchAPI, TwitchAPIRequest,
};
use asknothingx2_util::api::Method;

pub mod response;
pub mod types;

const CHARITY: &str = "charity";

pub trait CharityAPI: TwitchAPIBase {
    fn get_charity_campaign(&self, broadcaster_id: BroadcasterId) -> TwitchAPIRequest<EmptyBody>;
    fn get_charity_campaign_donations(
        &self,
        broadcaster_id: BroadcasterId,
        first: Option<u64>,
        after: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody>;
}

impl CharityAPI for TwitchAPI {
    fn get_charity_campaign(&self, broadcaster_id: BroadcasterId) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path([CHARITY, "campaigns"])
            .query([(BROADCASTER_ID, broadcaster_id)]);

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
        first: Option<u64>,
        after: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path([CHARITY, "donations"])
            .query([(BROADCASTER_ID, broadcaster_id)])
            .query_opt(FIRST, first.map(|x| x.to_string()))
            .query_opt(AFTER, after);

        TwitchAPIRequest::new(
            EndpointType::GetCharityCampaignDonations,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
}
