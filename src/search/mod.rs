use asknothingx2_util::api::Method;

use crate::{
    base::TwitchAPIBase,
    types::{AFTER, FIRST},
    EmptyBody, EndpointType, TwitchAPI, TwitchAPIRequest,
};

pub mod response;
pub mod types;

pub trait SearchAPI: TwitchAPIBase {
    fn search_categories(
        &self,
        query: &str,
        first: Option<&str>,
        after: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody>;
    fn search_channels(
        &self,
        query: &str,
        live_only: Option<bool>,
        first: Option<u64>,
        after: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody>;
}

impl SearchAPI for TwitchAPI {
    fn search_categories(
        &self,
        query: &str,
        first: Option<&str>,
        after: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path(["search", "categories"])
            .query("query", query)
            .query_opt(FIRST, first)
            .query_opt(AFTER, after);

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
        first: Option<u64>,
        after: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path(["search", "channels"])
            .query("query", query)
            .query_opt("live_only", live_only.map(|x| x.to_string()))
            .query_opt(FIRST, first.map(|x| x.to_string()))
            .query_opt(AFTER, after);

        TwitchAPIRequest::new(
            EndpointType::SearchCategories,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
}
