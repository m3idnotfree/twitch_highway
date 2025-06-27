use chrono::{DateTime, FixedOffset};
use serde::Serialize;

define_request!(
    #[derive(Serialize)]
    AnalyticsRequest {
        opts: {
            #[serde(rename = "type")]
            kind: AnalyticsType => TYPE,
            started_at: DateTime<FixedOffset> => STARTED_AT ; date,
            ended_at: DateTime<FixedOffset> ; date
        };
        apply_to_url
    }
);

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

impl AsRef<str> for AnalyticsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
