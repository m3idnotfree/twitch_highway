use serde::{Deserialize, Deserializer, Serialize};

/// <https://dev.twitch.tv/docs/api/guide/#pagination>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    #[serde(deserialize_with = "deserialize_non_empty_string")]
    pub cursor: String,
}

fn deserialize_non_empty_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    if s.is_empty() {
        Err(serde::de::Error::custom("cursor cannot be empty"))
    } else {
        Ok(s)
    }
}
