use crate::types::{BroadcasterId, GameId, Id};

define_select!(
    #[derive(Debug)]
    ClipsSelector {
            broadcaster_id: BroadcasterId => BROADCASTER_ID as by_broadcaster_id,
            game_id: GameId => GAME_ID as by_game_id,
            ids: Vec<Id> => ID  as by_ids ; vec,
    };
    apply_to_url
);

define_request!(
    GetClipsRequest<'a> {
        opts: {
            started_at: &'a str => STARTED_AT,
            ended_at: &'a str,
            is_featured: bool ; bool
        };
        apply_to_url
    }
);
