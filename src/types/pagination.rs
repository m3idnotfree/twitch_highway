use serde::{Deserialize, Serialize};

/// <https://dev.twitch.tv/docs/api/guide/#pagination>
/// - after — Use to get the next page of results
/// - before — Use to get the previous page of results
/// - first — Use to specify the number of items to include per page
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub cursor: String,
}

define_request! {
    #[derive(Debug, Clone, Serialize, Deserialize)]
     PaginationQuery {
        opts: {
            first: u64 => FIRST ; u64,
            before: String | into => BEFORE,
            after: String | into => AFTER,
        };
        into_query
    }
}

#[cfg(test)]
mod tests {
    use asknothingx2_util::serde::{
        deserialize_empty_object_as_none, serialize_none_as_empty_object,
    };
    use serde::{Deserialize, Serialize};

    use crate::types::{Pagination, PaginationQuery};

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
    fn query() {
        let pagination = PaginationQuery::new().first(50).after("eyJiI...");

        let mut url = url::Url::parse("https://api.twitch.tv/helix").unwrap();
        let mut query_pairs = url.query_pairs_mut();
        pagination.into_query(&mut query_pairs);
        drop(query_pairs);

        let query = url.query().unwrap();
        assert!(query.contains("first=50"));
        assert!(query.contains("after=eyJiI..."));
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
