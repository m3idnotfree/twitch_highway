use crate::types::{GameId, Id, UserId};

use super::types::{Period, Sort, Type};

define_select!(
    VideoSelector<'a> {
        ids: &'a [Id] => ID as by_ids ; vec,
        user_id: UserId => USER_ID as by_user_id,
        game_id: GameId => GAME_ID as by_game_id
    };
    apply_to_url
);

define_request! {
    VideosRequest<'a> {
        opts: {
            language: &'a str,
            period: Period,
            sort: Sort,
            kind: Type => TYPE
        };
        apply_to_url
    }
}
