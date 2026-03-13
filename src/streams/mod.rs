mod builder;
mod response;
mod types;

pub use builder::{GetFollowedStreams, GetStermaMarkers, GetStreams};
pub use response::{
    CreateStreamMarkerResponse, GetStreamMarkersResponse, StreamKeyResponse, StreamsResponse,
};
pub use types::{Marker, Stream, StreamKey, StreamMarker, StreamVideos};

use builder::StreamMarkerSelect;

use std::future::Future;

use types::CreateStrseamMarkerBody;

use crate::{
    types::{
        constants::{BROADCASTER_ID, KEY, MARKERS, STREAMS},
        BroadcasterId, UserId,
    },
    Client, Error,
};

pub trait StreamsAPI {
    /// See <https://dev.twitch.tv/docs/api/reference/#get-stream-key>
    fn get_stream_key(
        &self,
        broadcaster_id: &BroadcasterId,
    ) -> impl Future<Output = Result<StreamKeyResponse, Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#get-streams>
    fn get_streams<'a>(&'a self) -> GetStreams<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#get-followed-streams>
    fn get_followed_streams<'a>(&'a self, user_id: &'a UserId) -> GetFollowedStreams<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#create-stream-marker>
    fn create_stream_marker(
        &self,
        user_id: &UserId,
        description: Option<&str>,
    ) -> impl Future<Output = Result<CreateStreamMarkerResponse, Error>> + Send;

    /// - `&UserId` or `&VideoId`
    ///
    /// See <https://dev.twitch.tv/docs/api/reference/#get-stream-markers>
    fn get_stream_markers<'a>(
        &'a self,
        select: impl Into<StreamMarkerSelect<'a>> + Send,
    ) -> GetStermaMarkers<'a>;
}

impl StreamsAPI for Client {
    async fn get_stream_key(
        &self,
        broadcaster_id: &BroadcasterId,
    ) -> Result<StreamKeyResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().extend([STREAMS, KEY]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id);

        self.json(self.http_client().get(url)).await
    }

    fn get_streams<'a>(&'a self) -> GetStreams<'a> {
        GetStreams::new(self)
    }

    fn get_followed_streams<'a>(&'a self, user_id: &'a UserId) -> GetFollowedStreams<'a> {
        GetFollowedStreams::new(self, user_id)
    }

    async fn create_stream_marker(
        &self,
        user_id: &UserId,
        description: Option<&str>,
    ) -> Result<CreateStreamMarkerResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().extend([STREAMS, MARKERS]);

        let req = self.http_client().post(url).json(&CreateStrseamMarkerBody {
            user_id,
            description,
        });
        self.json(req).await
    }

    fn get_stream_markers<'a>(
        &'a self,
        select: impl Into<StreamMarkerSelect<'a>> + Send,
    ) -> GetStermaMarkers<'a> {
        GetStermaMarkers::new(self, select)
    }
}
