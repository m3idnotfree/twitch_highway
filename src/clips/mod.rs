mod builder;
mod response;
mod types;

pub use builder::{CreateClipBuilder, GetClipsBuilder};
pub use response::{ClipsDownloadResponse, ClipsInfoResponse, CreateClipsResponse};
pub use types::{Clip, ClipDownload, CreateClip};

use crate::{
    request::TwitchAPIRequest,
    types::{
        constants::{BROADCASTER_ID, CLIPS, CLIP_ID, DOWNLOADS, EDITOR_ID, VIDEOS},
        BroadcasterId, ClipId, GameId, UserId,
    },
    Client,
};

pub trait ClipsAPI {
    /// Creates a clip from the broadcaster’s stream
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster whose stream you want to create a clip from.
    ///
    /// # Returns
    ///
    /// Returns a [`CreateClipBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     clips::ClipsAPI,
    ///     types::BroadcasterId
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .create_clip(&BroadcasterId::from("1234"))
    ///     .title("title")
    ///     .duration(5.3)
    ///     .json()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `clips:edit`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#create-clip>
    fn create_clip<'a>(&'a self, broadcaster_id: &'a BroadcasterId) -> CreateClipBuilder<'a>;

    /// Gets one or more video clips that were captured from streams by broadcaster ID
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - An ID that identifies the broadcaster whose video clips you want to get.
    ///
    /// # Returns
    ///
    /// Returns a [`GetClipsBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use std::str::FromStr;
    /// use twitch_highway::{
    ///     clips::ClipsAPI,
    ///     types::BroadcasterId,
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .get_clips_by_broadcaster_id(&BroadcasterId::from("1234"))
    ///     .started_at(&"2018-01-01T00:00:00Z".parse().unwrap())
    ///     .ended_at(&"2018-03-01T00:00:00Z".parse().unwrap())
    ///     .is_featured(true)
    ///     .first(50)
    ///     .after("eyJiI...")
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
    /// <https://dev.twitch.tv/docs/api/reference/#get-clips>
    fn get_clips_by_broadcaster_id<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
    ) -> GetClipsBuilder<'a>;

    /// Gets one or more video clips that were captured from streams by game ID
    ///
    /// # Arguments
    ///
    /// * `game_id` - An ID that identifies the game whose clips you want to get.
    ///
    /// # Returns
    ///
    /// Returns a [`GetClipsBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use std::str::FromStr;
    /// use twitch_highway::{
    ///     clips::ClipsAPI,
    ///     types::GameId,
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .get_clips_by_game_id(&GameId::from("1234"))
    ///     .started_at(&"2018-01-01T00:00:00Z".parse().unwrap())
    ///     .ended_at(&"2018-03-01T00:00:00Z".parse().unwrap())
    ///     .is_featured(true)
    ///     .first(50)
    ///     .after("eyJiI...")
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
    /// <https://dev.twitch.tv/docs/api/reference/#get-clips>
    fn get_clips_by_game_id<'a>(&'a self, game_id: &'a GameId) -> GetClipsBuilder<'a>;

    /// Gets one or more video clips that were captured from streams by clip IDs
    ///
    /// # Arguments
    ///
    /// * `ids` - An ID that identifies the clip to get. (max 100)
    ///
    /// # Returns
    ///
    /// Returns a [`GetClipsBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use std::str::FromStr;
    /// use twitch_highway::{
    ///     clips::ClipsAPI,
    ///     types::ClipId
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .get_clips_by_ids(&[ClipId::from("1234"), ClipId::from("5678")])
    ///     .started_at(&"2018-01-01T00:00:00Z".parse().unwrap())
    ///     .ended_at(&"2018-03-01T00:00:00Z".parse().unwrap())
    ///     .is_featured(true)
    ///     .first(50)
    ///     .after("eyJiI...")
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
    /// <https://dev.twitch.tv/docs/api/reference/#get-clips>
    fn get_clips_by_ids<'a>(&'a self, ids: &'a [ClipId]) -> GetClipsBuilder<'a>;

    /// Provides URLs to download the video file(s) for the specified clips
    ///
    /// # Arguments
    ///
    /// * `editor_id` - The User ID of the editor for the channel you want to download a clip for.
    /// * `broadcaster_id` - The ID of the broadcaster you want to download clips for.
    /// * `clip_ids` - The ID that identifies the clip you want to download. Include this parameter for each clip you want to download. (max 10)
    ///
    /// # Returns
    ///
    /// Returns a [`ClipsDownloadResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     clips::ClipsAPI,
    ///     types::{BroadcasterId, ClipId, UserId},
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .get_clips_download(
    ///         &UserId::from("1234"),
    ///         &BroadcasterId::from("5678"),
    ///         &[ClipId::from("1234"), ClipId::from("5678")]
    ///     )
    ///     .json()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `editor:manage:clips or channel:manage:clips`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference#get-clips-download>
    fn get_clips_download(
        &self,
        editor_id: &UserId,
        broadcaster_id: &BroadcasterId,
        clip_ids: &[ClipId],
    ) -> TwitchAPIRequest<ClipsDownloadResponse>;

    fn create_clip_from_vod(
        &self,
        editor_id: &UserId,
        broadcaster_id: &BroadcasterId,
        vod_id: &str,
        vod_offset: u64,
        title: &str,
        duration: Option<f64>,
    ) -> TwitchAPIRequest<CreateClipsResponse>;
}

impl ClipsAPI for Client {
    fn create_clip<'a>(&'a self, broadcaster_id: &'a BroadcasterId) -> CreateClipBuilder<'a> {
        CreateClipBuilder::new(self, broadcaster_id)
    }
    fn get_clips_by_broadcaster_id<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
    ) -> GetClipsBuilder<'a> {
        GetClipsBuilder::by_broadcaster_id(self, broadcaster_id)
    }
    fn get_clips_by_game_id<'a>(&'a self, game_id: &'a GameId) -> GetClipsBuilder<'a> {
        GetClipsBuilder::by_game_id(self, game_id)
    }
    fn get_clips_by_ids<'a>(&'a self, ids: &'a [ClipId]) -> GetClipsBuilder<'a> {
        GetClipsBuilder::by_ids(self, ids)
    }
    simple_endpoint!(
    fn get_clips_download(
        editor_id: &UserId [key = EDITOR_ID],
        broadcaster_id: &BroadcasterId [key = BROADCASTER_ID],
        clip_ids: &[ClipId] [key = CLIP_ID, convert = extend],
    ) -> ClipsDownloadResponse;
        endpoint: GetClipsDownload,
        method: GET,
        path: [CLIPS,DOWNLOADS]
    );
    fn create_clip_from_vod(
        &self,
        editor_id: &UserId,
        broadcaster_id: &BroadcasterId,
        vod_id: &str,
        vod_offset: u64,
        title: &str,
        duration: Option<f64>,
    ) -> TwitchAPIRequest<CreateClipsResponse> {
        let mut url = self.base_url();
        url.path_segments_mut().unwrap().extend(&[VIDEOS, CLIPS]);

        url.query_pairs_mut()
            .append_pair(EDITOR_ID, editor_id)
            .append_pair(BROADCASTER_ID, broadcaster_id)
            .append_pair("vod_id", vod_id)
            .append_pair("vod_offset", &vod_offset.to_string())
            .append_pair("title", title);

        if let Some(duration) = duration {
            url.query_pairs_mut()
                .append_pair("duration", &format!("{:.1}", duration));
        }

        TwitchAPIRequest::new(
            crate::request::EndpointType::CreateClipFromVod,
            url,
            reqwest::Method::POST,
            self.default_headers(),
            None,
            self.http_client().clone(),
        )
    }
}
