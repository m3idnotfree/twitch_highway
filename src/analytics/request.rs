use chrono::{DateTime, FixedOffset};
use serde::Serialize;

use crate::{
    base::{IntoQueryPairs, QueryParams},
    types::constants::{STARTED_AT, TYPE},
};

request_struct!(
    #[derive(Serialize)]
    AnalyticsRequest {
        #[serde(rename = "type")]
        kind: AnalyticsType,
        started_at: DateTime<FixedOffset>,
        ended_at: DateTime<FixedOffset>
    }
);

impl IntoQueryPairs for AnalyticsRequest {
    fn into_query_pairs(self) -> Vec<(&'static str, String)> {
        let mut params = QueryParams::new();

        params
            .push_opt(TYPE, self.kind)
            .push_opt(STARTED_AT, self.started_at.map(|x| x.to_rfc3339()))
            .push_opt("ended_at", self.ended_at.map(|x| x.to_rfc3339()));

        params.build()
    }
}

#[derive(Debug, Serialize)]
pub enum AnalyticsType {
    #[serde(rename(serialize = "overview_v2"))]
    OverviewV2,
}

impl AnalyticsType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::OverviewV2 => "overview_v2",
        }
    }
}

impl From<AnalyticsType> for String {
    fn from(value: AnalyticsType) -> Self {
        value.as_str().to_string()
    }
}
