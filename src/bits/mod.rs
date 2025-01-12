use asknothingx2_util::api::Method;
use request::{BitsLeaderboardRequest, ExtensionTransactionRequest};

use crate::{
    base::TwitchAPIBase,
    types::{BroadcasterId, BROADCASTER_ID, EXTENSIONS},
    EmptyBody, EndpointType, TwitchAPI, TwitchAPIRequest,
};

pub mod request;
pub mod response;
pub mod types;

const BITS: &str = "bits";

pub trait BitsAPI: TwitchAPIBase {
    /// https://dev.twitch.tv/docs/api/reference/#get-bits-leaderboard
    fn get_bits_leaderboard(&self, request: BitsLeaderboardRequest) -> TwitchAPIRequest<EmptyBody>;
    /// https://dev.twitch.tv/docs/api/reference/#get-cheermotes
    fn get_cheermotes(&self, broadcaster_id: Option<BroadcasterId>) -> TwitchAPIRequest<EmptyBody>;
    /// https://dev.twitch.tv/docs/api/reference/#get-extension-transactions
    fn get_extension_transactions(
        &self,
        request: ExtensionTransactionRequest,
    ) -> TwitchAPIRequest<EmptyBody>;
}

impl BitsAPI for TwitchAPI {
    fn get_bits_leaderboard(&self, request: BitsLeaderboardRequest) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path([BITS, "leaderboard"]).query_pairs(request);

        TwitchAPIRequest::new(
            EndpointType::GetBitsLeaderboard,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn get_cheermotes(&self, broadcaster_id: Option<BroadcasterId>) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path([BITS, "cheermotes"])
            .query_opt(BROADCASTER_ID, broadcaster_id);

        TwitchAPIRequest::new(
            EndpointType::GetCheermotes,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn get_extension_transactions(
        &self,
        request: ExtensionTransactionRequest,
    ) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path([EXTENSIONS, "transactions"]).query_pairs(request);

        TwitchAPIRequest::new(
            EndpointType::GetExtensionTransactions,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
}
