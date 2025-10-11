use chrono::{DateTime, SecondsFormat, Utc};

use crate::{
    clips::{ClipsInfoResponse, CreateClipsResponse},
    request::TwitchAPIRequest,
    types::{
        constants::{
            AFTER, BROADCASTER_ID, CLIPS, ENDED_AT, FIRST, GAME_ID, HAS_DELAY, ID, IS_FEATURED,
            STARTED_AT,
        },
        BroadcasterId, ClipId, GameId,
    },
    TwitchAPI,
};

define_request_builder! {
    #[derive(Debug)]
    CreateClipBuilder<'a> {
        req: {broadcaster_id: &'a BroadcasterId [key = BROADCASTER_ID]},
        opts: {has_delay: bool [key = HAS_DELAY, convert = to_string]}
    } -> CreateClipsResponse;
    endpoint_type: CreateClip,
    method: POST,
    path: [CLIPS],

}

#[derive(Debug)]
pub enum ClipsQuery<'a> {
    Broadcaster(&'a BroadcasterId),
    Game(&'a GameId),
    Ids(&'a [ClipId]),
}

#[derive(Debug)]
pub struct GetClipsBuilder<'a> {
    api: &'a TwitchAPI,
    query: ClipsQuery<'a>,
    started_at: Option<&'a DateTime<Utc>>,
    ended_at: Option<&'a DateTime<Utc>>,
    is_featured: Option<bool>,
    first: Option<u8>,
    after: Option<&'a str>,
}

impl<'a> GetClipsBuilder<'a> {
    pub fn by_broadcaster_id(api: &'a TwitchAPI, broadcaster_id: &'a BroadcasterId) -> Self {
        Self {
            api,
            query: ClipsQuery::Broadcaster(broadcaster_id),
            started_at: None,
            ended_at: None,
            is_featured: None,
            first: None,
            after: None,
        }
    }

    pub fn by_game_id(api: &'a TwitchAPI, game_id: &'a GameId) -> Self {
        Self {
            api,
            query: ClipsQuery::Game(game_id),
            started_at: None,
            ended_at: None,
            is_featured: None,
            first: None,
            after: None,
        }
    }

    pub fn by_ids(api: &'a TwitchAPI, ids: &'a [ClipId]) -> Self {
        Self {
            api,
            query: ClipsQuery::Ids(ids),
            started_at: None,
            ended_at: None,
            is_featured: None,
            first: None,
            after: None,
        }
    }

    pub fn started_at(mut self, value: &'a DateTime<Utc>) -> Self {
        self.started_at = Some(value);
        self
    }
    pub fn ended_at(mut self, value: &'a DateTime<Utc>) -> Self {
        self.ended_at = Some(value);
        self
    }
    pub fn is_featured(mut self, value: bool) -> Self {
        self.is_featured = Some(value);
        self
    }
    pub fn first(mut self, value: u8) -> Self {
        self.first = Some(value);
        self
    }
    pub fn after(mut self, value: &'a str) -> Self {
        self.after = Some(value);
        self
    }

    pub fn build(self) -> TwitchAPIRequest<ClipsInfoResponse> {
        let mut url = self.api.build_url();

        url.path_segments_mut().unwrap().extend(&[CLIPS]);

        let mut query = url.query_pairs_mut();

        match self.query {
            ClipsQuery::Broadcaster(id) => {
                query.append_pair(BROADCASTER_ID, id);
            }
            ClipsQuery::Game(id) => {
                query.append_pair(GAME_ID, id);
            }
            ClipsQuery::Ids(ids) => {
                query.extend_pairs(ids.iter().map(|id| (ID, id)));
            }
        }

        if let Some(started_at) = self.started_at {
            query.append_pair(
                STARTED_AT,
                &started_at.to_rfc3339_opts(SecondsFormat::Secs, true),
            );
        }

        if let Some(ended_at) = self.ended_at {
            query.append_pair(
                ENDED_AT,
                &ended_at.to_rfc3339_opts(SecondsFormat::Secs, true),
            );
        }

        if let Some(is_featured) = self.is_featured {
            query.append_pair(IS_FEATURED, &is_featured.to_string());
        }

        if let Some(first) = self.first {
            query.append_pair(FIRST, &first.to_string());
        }

        if let Some(after) = self.after {
            query.append_pair(AFTER, after);
        }

        drop(query);

        TwitchAPIRequest::new(
            crate::request::EndpointType::GetClips,
            url,
            reqwest::Method::GET,
            self.api.default_headers(),
            None,
            self.api.client.clone(),
        )
    }

    pub async fn send(self) -> Result<reqwest::Response, crate::Error> {
        self.build().send().await
    }

    pub async fn json(self) -> Result<ClipsInfoResponse, crate::Error> {
        self.build().json().await
    }
}
