use asknothingx2_util::api::Method;
use response::{CategoriesResponse, ChannelsResponse};

use crate::{
    base::TwitchAPIBase,
    types::{constants::CHANNELS, PaginationQuery},
    EmptyBody, EndpointType, TwitchAPI, TwitchAPIRequest,
};

pub mod response;
pub mod types;

pub trait SearchAPI: TwitchAPIBase {
    /// <https://dev.twitch.tv/docs/api/reference/#search-categories>
    fn search_categories(
        &self,
        query: &str,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, CategoriesResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#search-channels>
    fn search_channels(
        &self,
        query: &str,
        live_only: Option<bool>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, ChannelsResponse>;
}

impl SearchAPI for TwitchAPI {
    fn search_categories(
        &self,
        query: &str,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, CategoriesResponse> {
        let mut url = self.build_url();
        url.path(["search", "categories"])
            .query("query", query)
            .query_opt_pairs(pagination);

        TwitchAPIRequest::new(
            EndpointType::SearchCategories,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn search_channels(
        &self,
        query: &str,
        live_only: Option<bool>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, ChannelsResponse> {
        let mut url = self.build_url();
        url.path(["search", CHANNELS])
            .query("query", query)
            .query_opt("live_only", live_only.map(|x| x.to_string()))
            .query_opt_pairs(pagination);

        TwitchAPIRequest::new(
            EndpointType::SearchCategories,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
}
