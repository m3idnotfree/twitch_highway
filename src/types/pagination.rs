use serde::{Deserialize, Serialize};

use crate::base::{IntoQueryPairs, QueryParams};

use super::constants::{AFTER, BEFORE, FIRST};

/// <https://dev.twitch.tv/docs/api/guide/#pagination>
/// - after — Use to get the next page of results
/// - before — Use to get the previous page of results
/// - first — Use to specify the number of items to include per page
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pagination {
    pub cursor: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PaginationQuery {
    first: Option<u64>,
    before: Option<String>,
    after: Option<String>,
}

impl PaginationQuery {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn first(mut self, first: u64) -> Self {
        self.first = Some(first);
        self
    }
    pub fn after<T: Into<String>>(mut self, after: T) -> Self {
        self.after = Some(after.into());
        self
    }
    pub fn before<T: Into<String>>(mut self, before: T) -> Self {
        self.before = Some(before.into());
        self
    }
}

impl IntoQueryPairs for PaginationQuery {
    fn into_query_pairs(self) -> Vec<(&'static str, String)> {
        let mut params = QueryParams::new();
        params
            .push_opt(FIRST, self.first.map(|x| x.to_string()))
            .push_opt(BEFORE, self.before)
            .push_opt(AFTER, self.after);

        params.build()
    }
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
