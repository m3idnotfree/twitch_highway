use std::fmt;

use chrono::{DateTime, Duration, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DateRange {
    pub started_at: DateTime<FixedOffset>,
    pub ended_at: DateTime<FixedOffset>,
}

impl DateRange {
    pub fn duration(&self) -> Duration {
        self.ended_at - self.started_at
    }
}

impl fmt::Display for DateRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let date = if self.started_at.date_naive() == self.ended_at.date_naive() {
            format!(
                "{} ({} - {})",
                self.started_at.format("%Y-%m-%d"),
                self.started_at.format("%H:%M"),
                self.ended_at.format("%H:%M")
            )
        } else {
            format!(
                "{} to {}",
                self.started_at.format("%Y-%m-%d %H:%M"),
                self.ended_at.format("%Y-%m-%d %H:%M")
            )
        };

        write!(f, "{date}")
    }
}

#[cfg(test)]
mod tests {
    use crate::types::DateRange;

    #[test]
    fn date_range_structure() {
        let date_range = DateRange {
            started_at: "2023-12-01T00:00:00Z".parse().unwrap(),
            ended_at: "2023-12-01T23:59:59Z".parse().unwrap(),
        };

        assert_eq!(
            date_range.started_at.to_rfc3339(),
            "2023-12-01T00:00:00+00:00"
        );
        assert_eq!(
            date_range.ended_at.to_rfc3339(),
            "2023-12-01T23:59:59+00:00"
        );

        let serialized = serde_json::to_string(&date_range).unwrap();
        let deserialized: DateRange = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.started_at, date_range.started_at);
        assert_eq!(deserialized.ended_at, date_range.ended_at);
    }
}
