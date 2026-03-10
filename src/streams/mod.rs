mod builder;
mod response;
mod types;

pub use builder::{
    GetFollowedStreamsBuilder, GetStermaMarkersBuilder, GetStreamsBuilder, StreamMarkerSelect,
};
pub use response::{
    CreateStreamMarkerResponse, GetStreamMarkersResponse, StreamKeyResponse, StreamsResponse,
};
pub use types::{Marker, Stream, StreamKey, StreamMarker, StreamVideos};

use types::CreateStrseamMarkerBody;

use crate::{
    request::TwitchAPIRequest,
    types::{
        constants::{BROADCASTER_ID, KEY, MARKERS, STREAMS},
        BroadcasterId, UserId, VideoId,
    },
    Client,
};

pub trait StreamsAPI {
    /// Gets the channel’s stream key
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster that owns the channel.
    ///
    /// # Returns
    ///
    /// Returns a [`StreamKeyResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     streams::StreamsAPI,
    ///     types::BroadcasterId,
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .get_stream_key(&BroadcasterId::from("1234"))
    ///     .json()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `channel:read:stream_key`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-stream-key>
    fn get_stream_key(&self, broadcaster_id: &BroadcasterId)
        -> TwitchAPIRequest<StreamKeyResponse>;

    /// Gets a list of all streams
    ///
    /// # Returns
    ///
    /// Returns a [`GetStreamsBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     streams::StreamsAPI,
    ///     // types::{}
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .get_streams()
    ///     .json()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// No scope required
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-streams>
    fn get_streams<'a>(&'a self) -> GetStreamsBuilder<'a>;

    /// Gets the list of broadcasters that the user follows and that are streaming live
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose list of followed streams you want to get.
    ///
    /// # Returns
    ///
    /// Returns a [`GetFollowedStreamsBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     streams::StreamsAPI,
    ///     types::UserId,
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let user_id = UserId::from("1234");
    /// let response = api
    ///     .get_followed_streams(&user_id)
    ///     .json()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `user:read:follows`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-followed-streams>
    fn get_followed_streams<'a>(&'a self, user_id: &'a UserId) -> GetFollowedStreamsBuilder<'a>;

    /// Adds a marker to a live stream
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the broadcaster that’s streaming content.
    /// * `description` - Optional A short description of the marker to help the user remember
    ///
    /// # Returns
    ///
    /// Returns a [`CreateStreamMarkerResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     streams::StreamsAPI,
    ///     types::UserId,
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .create_stream_marker(&UserId::from("1234"), None)
    ///     .json()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// No scope required
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#create-stream-marker>
    fn create_stream_marker(
        &self,
        user_id: &UserId,
        description: Option<&str>,
    ) -> TwitchAPIRequest<CreateStreamMarkerResponse>;

    /// Gets a list of markers from the user’s most recent stream or from the specified VOD/video
    ///
    /// # Arguments
    ///
    /// * `user_id` -
    ///
    /// # Returns
    ///
    /// Returns a [`GetStermaMarkersBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     streams::StreamsAPI,
    ///     types::UserId,
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .get_stream_markers_by_user_id(&UserId::from("1234"))
    ///     .json()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `user:read:broadcast` or `channel:manage:broadcast`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-stream-markers>
    fn get_stream_markers_by_user_id<'a>(
        &'a self,
        user_id: &'a UserId,
    ) -> GetStermaMarkersBuilder<'a>;

    /// Gets a list of markers from the user’s most recent stream or from the specified VOD/video
    ///
    /// # Arguments
    ///
    /// * `video_id` - A video on demand (VOD)/video ID.
    ///
    /// # Returns
    ///
    /// Returns a [`GetStermaMarkersBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     streams::StreamsAPI,
    ///     types::VideoId,
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .get_stream_markers_by_video_id(&VideoId::from("1234"))
    ///     .json()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `user:read:broadcast` or `channel:manage:broadcast`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-stream-markers>
    fn get_stream_markers_by_video_id<'a>(
        &'a self,
        video_id: &'a VideoId,
    ) -> GetStermaMarkersBuilder<'a>;
}

impl StreamsAPI for Client {
    fn get_stream_key(
        &self,
        broadcaster_id: &BroadcasterId,
    ) -> TwitchAPIRequest<StreamKeyResponse> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().extend(&[STREAMS, KEY]);

        let mut query = url.query_pairs_mut();

        query.append_pair(BROADCASTER_ID, broadcaster_id);

        drop(query);

        TwitchAPIRequest::new(
            crate::request::EndpointType::GetStreamKey,
            url,
            reqwest::Method::GET,
            self.default_headers(),
            None,
            self.http_client().clone(),
        )
    }
    fn get_streams<'a>(&'a self) -> GetStreamsBuilder<'a> {
        GetStreamsBuilder::new(self)
    }
    fn get_followed_streams<'a>(&'a self, user_id: &'a UserId) -> GetFollowedStreamsBuilder<'a> {
        GetFollowedStreamsBuilder::new(self, user_id)
    }
    fn create_stream_marker(
        &self,
        user_id: &UserId,
        description: Option<&str>,
    ) -> TwitchAPIRequest<CreateStreamMarkerResponse> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().extend(&[STREAMS, MARKERS]);

        let body = serde_json::to_string(&CreateStrseamMarkerBody {
            user_id,
            description,
        })
        .ok();

        TwitchAPIRequest::new(
            crate::request::EndpointType::CreateStreamMarker,
            url,
            reqwest::Method::POST,
            self.header_json(),
            body,
            self.http_client().clone(),
        )
    }
    fn get_stream_markers_by_user_id<'a>(
        &'a self,
        user_id: &'a UserId,
    ) -> GetStermaMarkersBuilder<'a> {
        GetStermaMarkersBuilder::user_id(self, user_id)
    }
    fn get_stream_markers_by_video_id<'a>(
        &'a self,
        video_id: &'a VideoId,
    ) -> GetStermaMarkersBuilder<'a> {
        GetStermaMarkersBuilder::video_id(self, video_id)
    }
}
