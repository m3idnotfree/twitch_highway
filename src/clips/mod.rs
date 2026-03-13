mod builder;
mod response;
mod types;

pub use builder::{CreateClip, GetClips};
pub use response::{ClipsDownloadResponse, ClipsInfoResponse, NewClipResponse};
pub use types::{Clip, ClipDownload, NewClip};

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
    /// See <https://dev.twitch.tv/docs/api/reference/#create-clip>
    fn create_clip<'a>(&'a self, broadcaster_id: &'a BroadcasterId) -> CreateClip<'a>;

    /// - `&BroadcasterId`, `&GameId`, or `&[ClipId]` (slice, array, or Vec)
    ///
    /// See <https://dev.twitch.tv/docs/api/reference/#get-clips>
    fn get_clips<'a>(&'a self, select: impl Into<ClipSelect<'a>>) -> GetClips<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference#get-clips-download>
    fn get_clips_download(
        &self,
        editor_id: &UserId,
        broadcaster_id: &BroadcasterId,
        clip_ids: &[ClipId],
    ) -> impl Future<Output = Result<ClipsDownloadResponse, Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference#create-clip-from-vod>
    fn create_clip_from_vod(
        &self,
        editor_id: &UserId,
        broadcaster_id: &BroadcasterId,
        vod_id: &str,
        vod_offset: u64,
        title: &str,
        duration: Option<f64>,
    ) -> impl Future<Output = Result<NewClipResponse, Error>> + Send;
}

impl ClipsAPI for Client {
    fn create_clip<'a>(&'a self, broadcaster_id: &'a BroadcasterId) -> CreateClip<'a> {
        CreateClip::new(self, broadcaster_id)
    }

    fn get_clips<'a>(&'a self, select: impl Into<ClipSelect<'a>>) -> GetClips<'a> {
        GetClips::new(self, select)
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
    ) -> Result<NewClipResponse, Error> {
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
