use crate::types::{BroadcasterId, GameId, Id};

define_select!(
    #[derive(Debug)]
    ClipsSelector<'a> {
            broadcaster_id: BroadcasterId => BROADCASTER_ID as by_broadcaster_id,
            game_id: GameId => GAME_ID as by_game_id,
            ids: &'a[Id] => ID  as by_ids ; vec,
    };
    into_query
);

define_request!(
    GetClipsRequest<'a> {
        opts: {
            started_at: &'a str => STARTED_AT,
            ended_at: &'a str,
            is_featured: bool ; bool
        };
        into_query
    }
);

#[cfg(test)]
mod tests {
    use crate::{
        clips::request::ClipsSelector,
        types::{BroadcasterId, GameId, Id},
    };

    #[test]
    fn clips_selector_by_broadcaster_id() {
        let selector = ClipsSelector::by_broadcaster_id(BroadcasterId::new("123456789"));

        assert_eq!(selector.broadcaster_id.unwrap().as_str(), "123456789");
        assert!(selector.game_id.is_none());
        assert!(selector.ids.is_none());
    }

    #[test]
    fn clips_selector_by_game_id() {
        let selector = ClipsSelector::by_game_id(GameId::new("509658"));

        assert_eq!(selector.game_id.unwrap().as_str(), "509658");
        assert!(selector.broadcaster_id.is_none());
        assert!(selector.ids.is_none());
    }

    #[test]
    fn clips_selector_by_ids() {
        let clip_ids = vec![Id::new("Clip1"), Id::new("Clip2"), Id::new("Clip3")];
        let selector = ClipsSelector::by_ids(&clip_ids);

        assert_eq!(selector.ids.as_ref().unwrap().len(), 3);
        assert_eq!(selector.ids.as_ref().unwrap()[0].as_str(), "Clip1");
        assert_eq!(selector.ids.as_ref().unwrap()[1].as_str(), "Clip2");
        assert_eq!(selector.ids.as_ref().unwrap()[2].as_str(), "Clip3");
        assert!(selector.broadcaster_id.is_none());
        assert!(selector.game_id.is_none());
    }
}
