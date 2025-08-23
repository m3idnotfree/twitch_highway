pub mod request;
pub mod response;
pub mod types;

use request::{VideoSelector, VideosRequest};
use response::{DeleteVideosResponse, VideosResponse};

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

#[cfg(test)]
mod tests {
    use crate::{
        types::Id,
        videos::{request::VideoSelector, VideosAPI},
    };

    api_test!(
        get_videos,
        [VideoSelector::by_ids(&[Id::from("335921245")]), None, None]
    );
    api_test!(delete_videos, [&[Id::from("1234"), Id::from("9876")],]);
}
