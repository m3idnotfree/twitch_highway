mod builder;
mod response;
mod types;

pub use builder::{CreateClipBuilder, GetClipsBuilder};
pub use response::{ClipsDownloadResponse, ClipsInfoResponse, CreateClipsResponse};
pub use types::{Clip, ClipDownload, CreateClip};

use builder::ClipSelect;

use std::future::Future;

use crate::{
    types::{
        constants::{BROADCASTER_ID, CLIPS, CLIP_ID, DOWNLOADS, EDITOR_ID, VIDEOS},
        BroadcasterId, ClipId, UserId,
    },
    Client, Error,
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
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .create_clip(&BroadcasterId::from("1234"))
    ///     .title("title")
    ///     .duration(5.3)
    ///     .send()
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

    /// Gets one or more video clips that were captured from streams
    ///
    /// # Arguments
    ///
    /// * `select` - The filter to use. Pass [`BroadcasterId`], [GameId](crate::types::GameId), or `&[`[`ClipId`]`]`.
    ///
    /// # Returns
    ///
    /// Returns a [`GetClipsBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     clips::ClipsAPI,
    ///     types::{BroadcasterId, ClipId, GameId},
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// // By broadcaster ID
    /// let response = api
    ///     .get_clips(&BroadcasterId::from("1234"))
    ///     .first(50)
    ///     .send()
    ///     .await?;
    ///
    /// // By game ID
    /// let response = api
    ///     .get_clips(&GameId::from("1234"))
    ///     .first(50)
    ///     .send()
    ///     .await?;
    ///
    /// // By clip IDs
    /// let ids = vec![ClipId::from("1234"), ClipId::from("5678")];
    /// let response = api
    ///     .get_clips(&ids)
    ///     .send()
    ///     .await?;
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
    fn get_clips<'a>(&'a self, select: impl Into<ClipSelect<'a>>) -> GetClipsBuilder<'a>;

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
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_clips_download(
    ///         &UserId::from("1234"),
    ///         &BroadcasterId::from("5678"),
    ///         &[ClipId::from("1234"), ClipId::from("5678")]
    ///     )
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
    ) -> impl Future<Output = Result<ClipsDownloadResponse, Error>> + Send;

    fn create_clip_from_vod(
        &self,
        editor_id: &UserId,
        broadcaster_id: &BroadcasterId,
        vod_id: &str,
        vod_offset: u64,
        title: &str,
        duration: Option<f64>,
    ) -> impl Future<Output = Result<CreateClipsResponse, Error>> + Send;
}

impl ClipsAPI for Client {
    fn create_clip<'a>(&'a self, broadcaster_id: &'a BroadcasterId) -> CreateClipBuilder<'a> {
        CreateClipBuilder::new(self, broadcaster_id)
    }

    fn get_clips<'a>(&'a self, select: impl Into<ClipSelect<'a>>) -> GetClipsBuilder<'a> {
        GetClipsBuilder::new(self, select)
    }

    async fn get_clips_download(
        &self,
        editor_id: &UserId,
        broadcaster_id: &BroadcasterId,
        clip_ids: &[ClipId],
    ) -> Result<ClipsDownloadResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().extend([CLIPS, DOWNLOADS]);

        url.query_pairs_mut()
            .append_pair(EDITOR_ID, editor_id)
            .append_pair(BROADCASTER_ID, broadcaster_id)
            .extend_pairs(clip_ids.iter().map(|id| (CLIP_ID, id)));

        self.json(self.http_client().get(url)).await
    }

    async fn create_clip_from_vod(
        &self,
        editor_id: &UserId,
        broadcaster_id: &BroadcasterId,
        vod_id: &str,
        vod_offset: u64,
        title: &str,
        duration: Option<f64>,
    ) -> Result<CreateClipsResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().extend([VIDEOS, CLIPS]);

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

        self.json(self.http_client().post(url)).await
    }
}
