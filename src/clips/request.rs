use crate::types::{BroadcasterId, GameId, Id};

define_select!(
    #[derive(Debug)]
    ClipsSelector<'a> {
            broadcaster_id: &'a BroadcasterId => BROADCASTER_ID as by_broadcaster_id,
            game_id: &'a GameId => GAME_ID as by_game_id,
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
