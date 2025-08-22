use crate::types::{GameId, Id, UserId};

use super::types::{Period, Sort, Type};

define_select!(
    VideoSelector<'a> {
        ids: &'a [Id] => ID as by_ids ; vec,
        user_id: &'a UserId => USER_ID as by_user_id,
        game_id: &'a GameId => GAME_ID as by_game_id
    };
    into_query
);

define_request! {
    VideosRequest {
        opts: {
            language: String | into,
            period: Period,
            sort: Sort,
            kind: Type => TYPE
        };
        into_query
    }
}

#[cfg(test)]
mod tests {
    use url::Url;

    use crate::{
        types::{GameId, Id, UserId},
        videos::{
            request::{VideoSelector, VideosRequest},
            types::{Period, Sort, Type},
        },
    };

    #[test]
    fn video_select_into_query_empty() {
        let selector = VideoSelector::new();
        let mut url = Url::parse("https://example.com").unwrap();
        let mut query_pairs = url.query_pairs_mut();

        selector.into_query(&mut query_pairs);
        drop(query_pairs);

        assert_eq!(url.query(), Some(""));
    }

    #[test]
    fn video_select_into_query_with_ids() {
        let ids = vec![Id::from("1234"), Id::from("5678")];
        let selector = VideoSelector::by_ids(&ids);
        let mut url = Url::parse("https://example.com").unwrap();
        let mut query_pairs = url.query_pairs_mut();

        selector.into_query(&mut query_pairs);
        drop(query_pairs);

        let query = url.query().unwrap();
        assert!(query.contains("id=1234"));
        assert!(query.contains("id=5678"));

        assert!(!query.contains("user_id="));
        assert!(!query.contains("game_id="));
    }

    #[test]
    fn video_select_into_query_with_user_id() {
        let user_id = UserId::from("1234");
        let selector = VideoSelector::by_user_id(&user_id);
        let mut url = Url::parse("https://example.com").unwrap();
        let mut query_pairs = url.query_pairs_mut();

        selector.into_query(&mut query_pairs);
        drop(query_pairs);

        let query = url.query().unwrap();
        assert!(query.contains("user_id=1234"));

        assert!(!query.contains("ids="));
        assert!(!query.contains("game_id="));
    }

    #[test]
    fn video_select_into_query_with_game_id() {
        let game_id = GameId::from("1234");
        let selector = VideoSelector::by_game_id(&game_id);
        let mut url = Url::parse("https://example.com").unwrap();
        let mut query_pairs = url.query_pairs_mut();

        selector.into_query(&mut query_pairs);
        drop(query_pairs);

        let query = url.query().unwrap();
        assert!(query.contains("game_id=1234"));

        assert!(!query.contains("ids="));
        assert!(!query.contains("user_id="));
    }

    #[test]
    fn video_request_into_query_empty() {
        let request = VideosRequest::new();
        let mut url = Url::parse("https://example.com").unwrap();
        let mut query_pairs = url.query_pairs_mut();

        request.into_query(&mut query_pairs);
        drop(query_pairs);

        assert_eq!(url.query(), Some(""));
    }

    #[test]
    fn video_request_into_query_with_params() {
        let request = VideosRequest::new()
            .language("en")
            .period(Period::Day)
            .sort(Sort::Time)
            .kind(Type::Upload);

        let mut url = Url::parse("https://example.com").unwrap();
        let mut query_pairs = url.query_pairs_mut();

        request.into_query(&mut query_pairs);
        drop(query_pairs);

        let query = url.query().unwrap();
        assert!(query.contains("language=en"));
        assert!(query.contains("period=day"));
        assert!(query.contains("sort=time"));
        assert!(query.contains("type=upload"));
    }

    #[test]
    fn video_request_partial_params() {
        let request = VideosRequest::new().language("en").sort(Sort::Views);

        let mut url = Url::parse("https://example.com").unwrap();
        let mut query_pairs = url.query_pairs_mut();

        request.into_query(&mut query_pairs);
        drop(query_pairs);

        let query = url.query().unwrap();
        assert!(query.contains("language=en"));
        assert!(query.contains("sort=views"));

        assert!(!query.contains("period="));
        assert!(!query.contains("type="));
    }

    #[test]
    fn selector_and_request() {
        let user_id = UserId::from("1234");
        let selector = VideoSelector::by_user_id(&user_id);
        let request = VideosRequest::new()
            .language("en")
            .period(Period::Week)
            .sort(Sort::Views);

        let mut url = Url::parse("https://api.twitch.tv/helix/videos").unwrap();
        let mut query_pairs = url.query_pairs_mut();

        selector.into_query(&mut query_pairs);
        request.into_query(&mut query_pairs);
        drop(query_pairs);

        let query = url.query().unwrap();
        assert!(query.contains("user_id=1234"));
        assert!(query.contains("language=en"));
        assert!(query.contains("period=week"));
        assert!(query.contains("sort=views"));

        assert!(!query.contains("game_id="));
    }

    #[test]
    fn video_select_empty_ids_vec() {
        let ids: Vec<Id> = vec![];
        let selector = VideoSelector::by_ids(&ids);
        let mut url = Url::parse("https://example.com").unwrap();
        let mut query_pairs = url.query_pairs_mut();

        selector.into_query(&mut query_pairs);
        drop(query_pairs);

        assert_eq!(url.query(), Some(""));
    }
}
