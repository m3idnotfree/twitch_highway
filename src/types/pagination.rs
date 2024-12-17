use serde::{Deserialize, Serialize};

/// https://dev.twitch.tv/docs/api/guide/#pagination
/// - after — Use to get the next page of results
/// - before — Use to get the previous page of results
/// - first — Use to specify the number of items to include per page
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub cursor: String,
}
