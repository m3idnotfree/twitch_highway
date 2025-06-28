use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct EmptyBody;

impl<'de> Deserialize<'de> for EmptyBody {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct EmptyVisitor;
        impl serde::de::Visitor<'_> for EmptyVisitor {
            type Value = EmptyBody;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("empty body")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if v.is_empty() {
                    Ok(EmptyBody)
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
        }
        deserializer.deserialize_str(EmptyVisitor)
    }
}
