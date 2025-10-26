use crate::{
    charity::CharityCampaignDonationResponse,
    types::{
        constants::{AFTER, BROADCASTER_ID, CHARITY, DONATIONS, FIRST},
        BroadcasterId,
    },
};

define_request_builder! {
    #[derive(Debug)]
    GetCharityCampaignDonationBuilder<'a>{
        req: {broadcaster_id: &'a BroadcasterId [key = BROADCASTER_ID]}
        opts: {
            first: u8 [key = FIRST, convert = to_string],
            after: &'a str [key = AFTER]
        }
    } -> CharityCampaignDonationResponse;
            endpoint: GetCharityCampaignDonations,
            method: GET,
            path: [CHARITY, DONATIONS],
}
