pub mod request;
pub mod response;
pub mod types;

use request::{ClipsSelector, GetClipsRequest};
use response::{ClipsInfoResponse, CreateClipsResponse};

use crate::types::{constants::BROADCASTER_ID, BroadcasterId, PaginationQuery};

endpoints! {
    ClipsAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#create-clip>
        fn create_clip(
            &self,
            broadcaster_id: &BroadcasterId,
            has_delay: Option<bool>,
        ) -> CreateClipsResponse {
            endpoint_type: CreateClip,
            method: POST,
            path: ["clips"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                opt("has_delay", has_delay.map(|x| x.to_string()))
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#get-clips>
        fn get_clips(
            &self,
            clips_selector: ClipsSelector,
            opts: Option<GetClipsRequest>,
            pagination: Option<PaginationQuery>,
        ) -> ClipsInfoResponse {
            endpoint_type: GetClips,
            method: GET,
            path: ["clips"],
            query_params: {
                into_query(clips_selector),
                opt_into_query(opts),
                pagination(pagination)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        clips::{request::ClipsSelector, ClipsAPI},
        types::{BroadcasterId, Id, PaginationQuery},
    };

    api_test!(create_clip, [&BroadcasterId::from("44322889"), None]);
    api_test!(
        get_clips,
        [
            ClipsSelector::by_ids(&[Id::from("AwkwardHelplessSalamanderSwiftRage")]),
            None,
            None
        ]
    );
    api_test!(extra
        get_clips,
        get_clips2,
        [
            ClipsSelector::by_broadcaster_id(&BroadcasterId::from("1234")),
            None,
            Some(PaginationQuery::new().first(5))
        ]
    );
}
