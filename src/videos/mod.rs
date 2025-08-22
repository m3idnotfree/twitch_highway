use asknothingx2_util::api::Method;
use request::{VideoSelector, VideosRequest};
use response::{DeleteVideosResponse, VideosResponse};

use crate::{
    request::{EndpointType, TwitchAPIRequest},
    types::{
        constants::{ID, VIDEOS},
        Id, PaginationQuery,
    },
    TwitchAPI,
};

pub mod request;
pub mod response;
pub mod types;

endpoints! {
    VideosAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#get-videos>
        fn get_videos(
            &self,
            video_selector: VideoSelector,
            opts: Option<VideosRequest>,
            pagination: Option<PaginationQuery>,
        ) -> VideosResponse {
            endpoint_type: EndpointType::GetVideos,
            method: Method::GET,
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
            endpoint_type: EndpointType::DeleteVideos,
            method: Method::DELETE,
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
        [VideoSelector::by_ids(&[Id::new("335921245")]), None, None]
    );
    api_test!(delete_videos, [&[Id::new("1234"), Id::new("9876")],]);
}
