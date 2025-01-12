use serde::{Deserialize, Serialize};

/// https://dev.twitch.tv/docs/api/guide/#pagination
/// - after — Use to get the next page of results
/// - before — Use to get the previous page of results
/// - first — Use to specify the number of items to include per page
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pagination {
    pub cursor: String,
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use super::Pagination;
    use asknothingx2_util::serde::{
        deserialize_empty_object_as_none, serialize_none_as_empty_object,
    };

    #[derive(Clone, Serialize, Deserialize)]
    struct TestPagination {
        #[serde(
            default,
            serialize_with = "serialize_none_as_empty_object",
            deserialize_with = "deserialize_empty_object_as_none"
        )]
        pagination: Option<Pagination>,
    }
    #[test]
    fn pagination() {
        let payload = "{ \"pagination\": {\n      \"cursor\": \"eyJiI...\"\n}}";
        let de: TestPagination = serde_json::from_str(payload).unwrap();
        assert_eq!("eyJiI...", de.pagination.clone().unwrap().cursor);
        let se = serde_json::to_string(&de).unwrap();
        assert!(se.contains("{\"pagination\":{\"cursor\":\"eyJiI...\"}}"));

        let payload = "{     \"pagination\": {}}";
        let de: TestPagination = serde_json::from_str(payload).unwrap();
        assert!(de.pagination.is_none());
        let se = serde_json::to_string(&de).unwrap();
        assert!(se.contains("{\"pagination\":{}}"));

        let payload = "{}";
        let de: TestPagination = serde_json::from_str(payload).unwrap();
        assert!(de.pagination.is_none());
        let se = serde_json::to_string(&de).unwrap();
        assert!(se.contains("{\"pagination\":{}}"));
    }
}
