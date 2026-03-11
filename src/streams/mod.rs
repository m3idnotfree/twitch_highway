mod builder;
mod response;
mod types;

pub use builder::{GetFollowedStreamsBuilder, GetStermaMarkersBuilder, GetStreamsBuilder};
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
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_stream_key(&BroadcasterId::from("1234"))
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
    fn get_stream_key(
        &self,
        broadcaster_id: &BroadcasterId,
    ) -> impl Future<Output = Result<StreamKeyResponse, Error>> + Send;

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
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_streams()
    ///     .send()
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
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let user_id = UserId::from("1234");
    /// let response = api
    ///     .get_followed_streams(&user_id)
    ///     .send()
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
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .create_stream_marker(&UserId::from("1234"), None)
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
    ) -> impl Future<Output = Result<CreateStreamMarkerResponse, Error>> + Send;

    /// Gets a list of markers from the user’s most recent stream or from the specified VOD/video
    ///
    /// # Arguments
    ///
    /// * `select` - The filter to use. Pass [`UserId`] or [VideoId](crate::types::VideoId).
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
    ///     types::{UserId, VideoId},
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// // By user ID
    /// let response = api
    ///     .get_stream_markers(&UserId::from("1234"))
    ///     .send()
    ///     .await?;
    ///
    /// // By video ID
    /// let response = api
    ///     .get_stream_markers(&VideoId::from("1234"))
    ///     .send()
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
    fn get_stream_markers<'a>(
        &'a self,
        select: impl Into<StreamMarkerSelect<'a>> + Send,
    ) -> GetStermaMarkersBuilder<'a>;
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

    fn get_streams<'a>(&'a self) -> GetStreamsBuilder<'a> {
        GetStreamsBuilder::new(self)
    }

    fn get_followed_streams<'a>(&'a self, user_id: &'a UserId) -> GetFollowedStreamsBuilder<'a> {
        GetFollowedStreamsBuilder::new(self, user_id)
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
    ) -> GetStermaMarkersBuilder<'a> {
        GetStermaMarkersBuilder::new(self, select)
    }
}
