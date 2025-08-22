use serde::{Deserialize, Serialize};

use crate::types::Id;

#[derive(Debug, Serialize, Deserialize)]
pub struct Ccl {
    pub id: Id,
    pub description: String,
    pub name: String,
}

#[cfg(test)]
mod tests {
    use crate::{ccls::types::Ccl, types::Id};

    #[test]
    fn ccls_type_structure() {
        let ccls = Ccl {
            id: Id::new("TestLabel"),
            description: "Test description for content classification".to_string(),
            name: "Test Label".to_string(),
        };

        assert_eq!(ccls.id.as_str(), "TestLabel");
        assert_eq!(
            ccls.description,
            "Test description for content classification"
        );
        assert_eq!(ccls.name, "Test Label");

        let serialized = serde_json::to_string(&ccls).unwrap();
        let deserialized: Ccl = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.id.as_str(), ccls.id.as_str());
        assert_eq!(deserialized.description, ccls.description);
        assert_eq!(deserialized.name, ccls.name);
    }
}
