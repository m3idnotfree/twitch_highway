use chrono::{DateTime, SecondsFormat, Utc};

use crate::{
    Client, Error,
    clips::{ClipsInfoResponse, NewClipResponse},
    types::{
        BroadcasterId, ClipId, GameId,
        constants::{
            AFTER, BROADCASTER_ID, CLIPS, ENDED_AT, FIRST, GAME_ID, ID, IS_FEATURED, STARTED_AT,
        },
    },
};

#[derive(Debug)]
pub struct CreateClip<'a> {
    client: &'a Client,
    broadcaster_id: &'a BroadcasterId,
    title: Option<&'a str>,
    duration: Option<f64>,
}

impl<'a> CreateClip<'a> {
    pub fn new(client: &'a Client, broadcaster_id: &'a BroadcasterId) -> Self {
        Self {
            client,
            broadcaster_id,
            title: None,
            duration: None,
        }
    }

    pub fn title(mut self, value: &'a str) -> Self {
        self.title = Some(value);
        self
    }

    pub fn duration(mut self, value: f64) -> Self {
        self.duration = Some(value);
        self
    }

    pub async fn send(self) -> Result<NewClipResponse, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut().unwrap().push(CLIPS);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, self.broadcaster_id);
        if let Some(val) = self.title {
            url.query_pairs_mut().append_pair("title", val);
        }
        if let Some(val) = self.duration {
            url.query_pairs_mut()
                .append_pair("duration", &format!("{:.1}", val));
        }

        let req = self.client.http_client().post(url);
        self.client.json(req).await
    }
}

#[derive(Debug)]
pub enum ClipSelect<'a> {
    Broadcaster(&'a BroadcasterId),
    Game(&'a GameId),
    Ids(&'a [ClipId]),
}

impl<'a> From<&'a BroadcasterId> for ClipSelect<'a> {
    fn from(value: &'a BroadcasterId) -> Self {
        Self::Broadcaster(value)
    }
}

impl<'a> From<&'a GameId> for ClipSelect<'a> {
    fn from(value: &'a GameId) -> Self {
        Self::Game(value)
    }
}

impl<'a> From<&'a Vec<ClipId>> for ClipSelect<'a> {
    fn from(value: &'a Vec<ClipId>) -> Self {
        Self::Ids(value)
    }
}

impl<'a> From<&'a [ClipId]> for ClipSelect<'a> {
    fn from(value: &'a [ClipId]) -> Self {
        Self::Ids(value)
    }
}

impl<'a, const N: usize> From<&'a [ClipId; N]> for ClipSelect<'a> {
    fn from(value: &'a [ClipId; N]) -> Self {
        Self::Ids(value)
    }
}

impl<'a> ClipSelect<'a> {
    pub(crate) fn append_to_query(&self, url: &mut url::Url) {
        match self {
            Self::Broadcaster(id) => {
                url.query_pairs_mut().append_pair(BROADCASTER_ID, id);
            }
            Self::Game(id) => {
                url.query_pairs_mut().append_pair(GAME_ID, id);
            }
            Self::Ids(ids) => {
                url.query_pairs_mut()
                    .extend_pairs(ids.iter().map(|id| (ID, id)));
            }
        }
    }
}

#[derive(Debug)]
pub struct GetClips<'a> {
    client: &'a Client,
    select: ClipSelect<'a>,
    started_at: Option<&'a DateTime<Utc>>,
    ended_at: Option<&'a DateTime<Utc>>,
    is_featured: Option<bool>,
    first: Option<u8>,
    after: Option<&'a str>,
}

impl<'a> GetClips<'a> {
    pub(crate) fn new(client: &'a Client, select: impl Into<ClipSelect<'a>>) -> Self {
        Self {
            client,
            select: select.into(),
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

    pub async fn send(self) -> Result<ClipsInfoResponse, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut().unwrap().push(CLIPS);

        self.select.append_to_query(&mut url);
        if let Some(started_at) = self.started_at {
            url.query_pairs_mut().append_pair(
                STARTED_AT,
                &started_at.to_rfc3339_opts(SecondsFormat::Secs, true),
            );
        }
        if let Some(ended_at) = self.ended_at {
            url.query_pairs_mut().append_pair(
                ENDED_AT,
                &ended_at.to_rfc3339_opts(SecondsFormat::Secs, true),
            );
        }
        if let Some(is_featured) = self.is_featured {
            url.query_pairs_mut()
                .append_pair(IS_FEATURED, &is_featured.to_string());
        }
        if let Some(first) = self.first {
            url.query_pairs_mut().append_pair(FIRST, &first.to_string());
        }
        if let Some(after) = self.after {
            url.query_pairs_mut().append_pair(AFTER, after);
        }

        let req = self.client.http_client().get(url);
        self.client.json(req).await
    }
}
