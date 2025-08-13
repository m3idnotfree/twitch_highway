use serde::{de::Error, Deserialize, Serialize};

/// Serialize empty object
/// Deserialize null, empty string, empty object, empty arrays
#[derive(Debug)]
pub struct NoContent;

impl Serialize for NoContent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let state = serializer.serialize_struct("NoContent", 0)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for NoContent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct EmptyVisitor;
        impl<'de> serde::de::Visitor<'de> for EmptyVisitor {
            type Value = NoContent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("empty body")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if v.is_empty() {
                    Ok(NoContent)
                } else {
                    Err(E::custom("expected empty string"))
                }
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_str(&v)
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NoContent)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                if map.next_entry::<String, serde_json::Value>()?.is_none() {
                    Ok(NoContent)
                } else {
                    Err(A::Error::custom("expected empty object"))
                }
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                if seq.next_element::<serde_json::Value>()?.is_none() {
                    Ok(NoContent)
                } else {
                    Err(A::Error::custom("expected empty array"))
                }
            }
        }
        deserializer.deserialize_any(EmptyVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let no_content = NoContent;
        let json = serde_json::to_string(&no_content).unwrap();
        assert_eq!("{}", json);
    }

    #[test]
    fn deserialize() {
        serde_json::from_str::<NoContent>("\"\"").unwrap();
        serde_json::from_str::<NoContent>("null").unwrap();
        serde_json::from_str::<NoContent>("{}").unwrap();
        serde_json::from_str::<NoContent>("[]").unwrap();
    }

    #[test]
    fn invalid() {
        assert!(serde_json::from_str::<NoContent>("\"hello\"").is_err());
        assert!(serde_json::from_str::<NoContent>("{\"key\": \"value\"}").is_err());
        assert!(serde_json::from_str::<NoContent>("[1, 2, 3]").is_err());
    }

    #[test]
    fn round_trip() {
        let no_content = NoContent;
        let json = serde_json::to_string(&no_content).unwrap();
        serde_json::from_str::<NoContent>(&json).unwrap();
    }
}
