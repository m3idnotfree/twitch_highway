use crate::{
    search::{CategoriesResponse, ChannelsResponse},
    types::constants::{AFTER, CATEGORIES, CHANNELS, FIRST, QUERY, SEARCH},
    Client, Error,
};

const LIVE_ONLY: &str = "live_only";

#[derive(Debug)]
pub struct SearchCategoriesBuilder<'a> {
    client: &'a Client,
    query: &'a str,
    first: Option<u8>,
    after: Option<&'a str>,
}

impl<'a> SearchCategoriesBuilder<'a> {
    pub fn new(client: &'a Client, query: &'a str) -> Self {
        Self {
            client,
            query,
            first: None,
            after: None,
        }
    }

    pub fn first(mut self, value: u8) -> Self {
        self.first = Some(value);
        self
    }

    pub fn after(mut self, value: &'a str) -> Self {
        self.after = Some(value);
        self
    }

    pub async fn send(self) -> Result<CategoriesResponse, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([SEARCH, CATEGORIES]);

        url.query_pairs_mut().append_pair(QUERY, self.query);
        if let Some(val) = self.first {
            url.query_pairs_mut().append_pair(FIRST, &val.to_string());
        }
        if let Some(val) = self.after {
            url.query_pairs_mut().append_pair(AFTER, val);
        }

        let req = self.client.http_client().get(url);
        self.client.json(req).await
    }
}

#[derive(Debug)]
pub struct SearchChannelsBuilder<'a> {
    client: &'a Client,
    query: &'a str,
    live_only: Option<bool>,
    first: Option<u8>,
    after: Option<&'a str>,
}

impl<'a> SearchChannelsBuilder<'a> {
    pub fn new(client: &'a Client, query: &'a str) -> Self {
        Self {
            client,
            query,
            live_only: None,
            first: None,
            after: None,
        }
    }

    pub fn live_only(mut self, value: bool) -> Self {
        self.live_only = Some(value);
        self
    }

    pub fn first(mut self, value: u8) -> Self {
        self.first = Some(value);
        self
    }

    pub fn after(mut self, value: &'a str) -> Self {
        self.after = Some(value);
        self
    }

    pub async fn send(self) -> Result<ChannelsResponse, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut().unwrap().extend([SEARCH, CHANNELS]);

        url.query_pairs_mut().append_pair(QUERY, self.query);
        if let Some(val) = self.live_only {
            url.query_pairs_mut()
                .append_pair(LIVE_ONLY, &val.to_string());
        }
        if let Some(val) = self.first {
            url.query_pairs_mut().append_pair(FIRST, &val.to_string());
        }
        if let Some(val) = self.after {
            url.query_pairs_mut().append_pair(AFTER, val);
        }

        let req = self.client.http_client().get(url);
        self.client.json(req).await
    }
}
