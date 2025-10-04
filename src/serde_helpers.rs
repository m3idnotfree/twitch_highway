use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

pub fn deserialize_optional_datetime<'de, D>(
    deserializer: D,
) -> Result<Option<DateTime<FixedOffset>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        DateTime::parse_from_rfc3339(&s)
            .map(Some)
            .map_err(serde::de::Error::custom)
    }
}
