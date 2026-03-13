use std::fmt;

use asknothingx2_util::serde::serialize_none_as_empty_string;
use chrono::{DateTime, Duration, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::serde_helpers::deserialize_optional_datetime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_string",
        deserialize_with = "deserialize_optional_datetime"
    )]
    pub started_at: Option<DateTime<FixedOffset>>,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_string",
        deserialize_with = "deserialize_optional_datetime"
    )]
    pub ended_at: Option<DateTime<FixedOffset>>,
}

impl DateRange {
    pub fn duration(&self) -> Option<Duration> {
        match (self.ended_at, self.started_at) {
            (Some(end), Some(start)) => Some(end - start),
            _ => None,
        }
    }
}

impl fmt::Display for DateRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.started_at, self.ended_at) {
            (Some(start), Some(end)) => {
                if start.date_naive() == end.date_naive() {
                    write!(
                        f,
                        "{} ({} - {})",
                        start.format("%Y-%m-%d"),
                        start.format("%H:%M"),
                        end.format("%H:%M")
                    )
                } else {
                    write!(
                        f,
                        "{} to {}",
                        start.format("%Y-%m-%d %H:%M"),
                        end.format("%Y-%m-%d %H:%M")
                    )
                }
            }
            (Some(start), None) => write!(f, "From {}", start.format("%Y-%m-%d %H:%M")),
            (None, Some(end)) => write!(f, "Until {}", end.format("%Y-%m-%d %H:%M")),
            (None, None) => write!(f, "No date range specified"),
        }
    }
}
