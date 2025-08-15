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
        into_query
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use chrono::{DateTime, FixedOffset};

    use crate::analytics::request::{AnalyticsRequest, AnalyticsType};

    #[test]
    fn analytics_type_serialization() {
        let overview_type = AnalyticsType::OverviewV2;

        assert_eq!(overview_type.as_str(), "overview_v2");
        assert_eq!(overview_type.as_ref(), "overview_v2");

        let as_string: String = overview_type.into();
        assert_eq!(as_string, "overview_v2");

        let serialized = serde_json::to_string(&AnalyticsType::OverviewV2).unwrap();
        assert_eq!(serialized, "\"overview_v2\"");
    }

    #[test]
    fn request_serialization() {
        let started_at = DateTime::<FixedOffset>::from_str("2023-12-01T00:00:00Z").unwrap();
        let ended_at = DateTime::<FixedOffset>::from_str("2023-12-01T23:59:59Z").unwrap();

        let request = AnalyticsRequest::new()
            .kind(AnalyticsType::OverviewV2)
            .started_at(started_at)
            .ended_at(ended_at);

        let mut url = url::Url::parse("https://api.twitch.tv/helix").unwrap();
        let mut query_pairs = url.query_pairs_mut();
        request.into_query(&mut query_pairs);
        drop(query_pairs);

        let query = url.query().unwrap();
        assert!(query.contains("type=overview_v2"));
        // assert!(query.contains("started_at=2023-12-01T00:00:00Z"));
        // assert!(query.contains("ended_at=2023-12-01T23:59:59Z"));
    }

    #[test]
    fn request_partial() {
        let request = AnalyticsRequest::new().kind(AnalyticsType::OverviewV2);

        let mut url = url::Url::parse("https://api.twitch.tv/helix").unwrap();
        let mut query_pairs = url.query_pairs_mut();
        request.into_query(&mut query_pairs);
        drop(query_pairs);

        let query = url.query().unwrap();
        assert!(query.contains("type=overview_v2"));
        assert!(!query.contains("started_at"));
        assert!(!query.contains("ended_at"));
    }
}
