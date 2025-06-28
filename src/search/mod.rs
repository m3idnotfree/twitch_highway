use asknothingx2_util::api::Method;
use response::{CategoriesResponse, ChannelsResponse};

use crate::{
    request::{EndpointType, TwitchAPIRequest},
    types::{constants::CHANNELS, PaginationQuery},
    TwitchAPI,
};

pub mod response;
pub mod types;

#[cfg_attr(docsrs, doc(cfg(feature = "search")))]
pub trait SearchAPI {
    /// <https://dev.twitch.tv/docs/api/reference/#search-categories>
    fn search_categories(
        &self,
        query: &str,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<CategoriesResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#search-channels>
    fn search_channels(
        &self,
        query: &str,
        live_only: Option<bool>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<ChannelsResponse>;
}

impl SearchAPI for TwitchAPI {
    fn search_categories(
        &self,
        query: &str,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<CategoriesResponse> {
        let mut url = self.build_url();
        url.path(["search", "categories"]).query("query", query);
        if let Some(pagination) = pagination {
            pagination.apply_to_url(&mut url);
        }

        TwitchAPIRequest::new(
            EndpointType::SearchCategories,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            None,
        )
    }
    fn search_channels(
        &self,
        query: &str,
        live_only: Option<bool>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<ChannelsResponse> {
        let mut url = self.build_url();
        url.path(["search", CHANNELS])
            .query("query", query)
            .query_opt("live_only", live_only.map(|x| x.to_string()));
        if let Some(pagination) = pagination {
            pagination.apply_to_url(&mut url);
        }

        TwitchAPIRequest::new(
            EndpointType::SearchCategories,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            None,
        )
    }
}
