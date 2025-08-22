use chrono::{DateTime, FixedOffset};
use serde::Serialize;

use crate::types::UserId;

define_request!(
    #[derive(Serialize)]
    BitsLeaderboardRequest<'a> {
        opts: {
            count: u64 ; u64,
            period: &'a str,
            started_at: &'a DateTime<FixedOffset> => STARTED_AT ; date,
            user_id: &'a UserId => USER_ID,
        };
        into_query
    }
);
#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use chrono::{DateTime, FixedOffset};

    use crate::{bits::request::BitsLeaderboardRequest, types::UserId};

    #[test]
    fn bits_leaderboard_request_serialization() {
        let started_at = DateTime::<FixedOffset>::from_str("2023-12-01T00:00:00Z").unwrap();
        let user_id = UserId::new("123456789");

        let request = BitsLeaderboardRequest::new()
            .count(10)
            .period("week")
            .started_at(&started_at)
            .user_id(&user_id);

        let mut url = url::Url::parse("https://api.twitch.tv/helix").unwrap();
        let mut query_pairs = url.query_pairs_mut();
        request.into_query(&mut query_pairs);
        drop(query_pairs);

        let query = url.query().unwrap();
        assert!(query.contains("count=10"));
        assert!(query.contains("period=week"));
        // assert!(query.contains("started_at=2023-12-01T00:00:00Z"));
        assert!(query.contains("user_id=123456789"));
    }

    #[test]
    fn bits_leaderboard_request_partial() {
        let request = BitsLeaderboardRequest::new().count(25).period("month");

        let mut url = url::Url::parse("https://api.twitch.tv/helix").unwrap();
        let mut query_pairs = url.query_pairs_mut();
        request.into_query(&mut query_pairs);
        drop(query_pairs);

        let query = url.query().unwrap();
        assert!(query.contains("count=25"));
        assert!(query.contains("period=month"));
        assert!(!query.contains("started_at"));
        assert!(!query.contains("user_id"));
    }
}
