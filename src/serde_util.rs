#![allow(dead_code)]
use serde::{de::IntoDeserializer, Deserialize, Deserializer, Serialize, Serializer};

/// https://github.com/serde-rs/serde/issues/2362
pub fn deserialize_empty_object<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    #[derive(Deserialize)]
    #[serde(
        untagged,
        deny_unknown_fields,
        expecting = "object, empty object or null"
    )]
    enum Helper<T> {
        Data(T),
        Empty {},
        Null,
    }
    match Helper::deserialize(deserializer) {
        Ok(Helper::Data(data)) => Ok(Some(data)),
        Ok(_) => Ok(None),
        Err(e) => Err(e),
    }
}

/// Deserialize a string that might be empty into an Option<T>
/// Returns None for empty strings, Some(T) for non-empty strings that can be parsed into T
pub fn deserialize_string_or_empty<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrEmpty<T> {
        Value(T),
        Empty(String),
    }
    match StringOrEmpty::deserialize(deserializer)? {
        StringOrEmpty::Value(value) => Ok(Some(value)),
        StringOrEmpty::Empty(s) if s.is_empty() => Ok(None),
        StringOrEmpty::Empty(s) => T::deserialize(s.into_deserializer()).map(Some),
    }
}

/// Deserialize a string that might be empty into a String
/// Returns empty string for empty strings, String for non-empty strings
pub fn deserialize_string_empty_as_empty<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    Ok(s)
}

/// Serialize Option<T> as string, where None becomes empty string
/// and Some(value) becomes value.to_string()
pub fn serialize_none_as_empty_string<T, S>(
    value: &Option<T>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    // T: std::fmt::Display,
    T: Serialize,
{
    match value {
        None => serializer.serialize_str(""),
        Some(v) => v.serialize(serializer),
    }
}
