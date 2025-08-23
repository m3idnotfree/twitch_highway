pub mod response;
pub mod types;

use asknothingx2_util::api::Method;
use response::{CharityCampaignDonationResponse, CharityCampaignResponse};

use crate::{
    request::EndpointType,
    types::{constants::BROADCASTER_ID, BroadcasterId, PaginationQuery},
};

const CHARITY: &str = "charity";

endpoints! {
    CharityAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#get-charity-campaign>
        fn get_charity_campaign(
            &self,
            broadcaster_id: &BroadcasterId,
        ) -> CharityCampaignResponse {
            endpoint_type: EndpointType::GetCharityCampaign,
            method: Method::GET,
            path: [CHARITY, "campaigns"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#get-charity-campaign-donations>
        fn get_charity_campaign_donations(
            &self,
            broadcaster_id: &BroadcasterId,
            pagination: Option<PaginationQuery>,
        ) -> CharityCampaignDonationResponse {
            endpoint_type: EndpointType::GetCharityCampaignDonations,
            method: Method::GET,
            path: [CHARITY, "donations"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                pagination(pagination)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{charity::CharityAPI, types::BroadcasterId};

    api_test!(get_charity_campaign, [&BroadcasterId::from("123456")]);

    api_test!(
        get_charity_campaign_donations,
        [&BroadcasterId::from("123456"), None]
    );
}
