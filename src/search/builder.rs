use crate::{
    search::{CategoriesResponse, ChannelsResponse},
    types::constants::{AFTER, CATEGORIES, CHANNELS, FIRST, QUERY, SEARCH},
};

const LIVE_ONLY: &str = "live_only";

define_request_builder! {
   #[derive(Debug)]
   SearchCategoriesBuilder<'a> {
       req: {query: &'a str [key =QUERY]},
       opts: {
           first: u8 [key = FIRST, convert = to_string],
           after: &'a str [key = AFTER],
        }
    } -> CategoriesResponse;
    endpoint: SearchCategories,
    method: GET,
    path: [SEARCH, CATEGORIES],
}

define_request_builder! {
   #[derive(Debug)]
   SearchChannelsBuilder<'a> {
       req: {query: &'a str [key = QUERY]},
       opts: {
           live_only: bool [key = LIVE_ONLY, convert = to_string],
           first: u8 [key = FIRST, convert = to_string],
           after: &'a str [key = AFTER],
        }
    } -> ChannelsResponse;
    endpoint: SearchChannels,
    method: GET,
    path: [SEARCH, CHANNELS],
}
