use chrono::{DateTime, Utc};

use crate::{
    analytics::{AnalyticsType, ExtensionAnalyticsResponse, GameAnalyticsResponse},
    types::{
        constants::{
            AFTER, ENDED_AT, EXTENSIONS, EXTENSION_ID, FIRST, GAMES, GAME_ID, STARTED_AT, TYPE,
        },
        ExtensionId, GameId,
    },
};

const ANALYTICS: &str = "analytics";

define_request_builder! {
    #[derive(Debug)]
    GetExtensionBuilder<'a> {
        extension_id: &'a ExtensionId [key = EXTENSION_ID],
        kind: AnalyticsType [key = TYPE, convert = as_ref],
        started_at: &'a DateTime<Utc>[key = STARTED_AT, convert = rfc3339_opt],
        ended_at: &'a DateTime<Utc>[key = ENDED_AT, convert = rfc3339_opt],
        first: u8 [key = FIRST, convert = to_string],
        after: &'a str [key = AFTER]
    } -> ExtensionAnalyticsResponse;
    endpoint_type: GetExtensionAnalytics,
    method: GET,
    path: [ANALYTICS, EXTENSIONS],
}

define_request_builder! {
    #[derive(Debug)]
    GetGameAnalyticsBuilder<'a> {
        game_id: &'a GameId [key = GAME_ID],
        kind: AnalyticsType [key = TYPE, convert = as_ref],
        started_at: &'a DateTime<Utc>[key = STARTED_AT, convert = rfc3339_opt],
        ended_at: &'a DateTime<Utc>[key = ENDED_AT, convert = rfc3339_opt],
        first: u8 [key = FIRST, convert = to_string],
        after: &'a str [key = AFTER]
    } -> GameAnalyticsResponse;
    endpoint_type: GetGameAnalytics,
    method: GET,
    path: [ANALYTICS, GAMES],
}
