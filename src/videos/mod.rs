mod request;
mod response;
mod types;

pub use request::{VideoSelector, VideosRequest};
pub use response::{DeleteVideosResponse, VideosResponse};
pub use types::{MutedSegment, Period, Sort, Type, Video};

use crate::types::{constants::ID, Id, PaginationQuery};

const VIDEOS: &str = "videos";

endpoints! {
    VideosAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#get-videos>
        fn get_videos(
            &self,
            video_selector: VideoSelector,
            opts: Option<VideosRequest>,
            pagination: Option<PaginationQuery>,
        ) -> VideosResponse {
            endpoint_type: GetVideos,
            method: GET,
            path: [VIDEOS],
            query_params: {
                into_query(video_selector),
                opt_into_query(opts),
                pagination(pagination),
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#delete-videos>
        fn delete_videos(
            &self,
            ids: &[Id],
        ) -> DeleteVideosResponse {
            endpoint_type: DeleteVideos,
            method: DELETE,
            path: [VIDEOS],
            query_params: {
                extend(ids.iter().map(|id| (ID, id)))
            }
        }
    }
}
