mod request;
mod response;
mod types;

pub use request::{ClipsSelector, GetClipsRequest};
pub use response::{ClipsInfoResponse, CreateClipsResponse};
pub use types::{Clip, CreateClip};

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
