use crate::{
    streams::{GetStreamMarkersResponse, StreamsResponse},
    types::{
        constants::{
            AFTER, BEFORE, FIRST, FOLLOWED, GAME_ID, LANGUAGE, MARKERS, STREAMS, TYPE, USER_ID,
            USER_LOGIN, VIDEO_ID,
        },
        GameId, UserId, VideoId,
    },
    Client, Error,
};

#[derive(Debug)]
pub struct GetStreams<'a> {
    client: &'a Client,
    user_ids: Option<&'a [UserId]>,
    user_logins: Option<&'a [&'a str]>,
    game_ids: Option<&'a [GameId]>,
    kind: Option<GetStreamType>,
    languages: Option<&'a [&'a str]>,
    first: Option<u8>,
    before: Option<&'a str>,
    after: Option<&'a str>,
}

impl<'a> GetStreams<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self {
            client,
            user_ids: None,
            user_logins: None,
            game_ids: None,
            kind: None,
            languages: None,
            first: None,
            before: None,
            after: None,
        }
    }

    pub fn user_ids(mut self, value: &'a [UserId]) -> Self {
        self.user_ids = Some(value);
        self
    }

    pub fn user_logins(mut self, value: &'a [&'a str]) -> Self {
        self.user_logins = Some(value);
        self
    }

    pub fn game_ids(mut self, value: &'a [GameId]) -> Self {
        self.game_ids = Some(value);
        self
    }

    pub fn kind(mut self, value: GetStreamType) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn languages(mut self, value: &'a [&'a str]) -> Self {
        self.languages = Some(value);
        self
    }

    pub fn first(mut self, value: u8) -> Self {
        self.first = Some(value);
        self
    }

    pub fn before(mut self, value: &'a str) -> Self {
        self.before = Some(value);
        self
    }

    pub fn after(mut self, value: &'a str) -> Self {
        self.after = Some(value);
        self
    }

    pub async fn send(self) -> Result<StreamsResponse, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut().unwrap().push(STREAMS);

        if let Some(ids) = self.user_ids {
            url.query_pairs_mut()
                .extend_pairs(ids.iter().map(|id| (USER_ID, id)));
        }
        if let Some(logins) = self.user_logins {
            url.query_pairs_mut()
                .extend_pairs(logins.iter().map(|login| (USER_LOGIN, login)));
        }
        if let Some(ids) = self.game_ids {
            url.query_pairs_mut()
                .extend_pairs(ids.iter().map(|id| (GAME_ID, id)));
        }
        if let Some(val) = self.kind {
            url.query_pairs_mut().append_pair(TYPE, val.as_ref());
        }
        if let Some(val) = self.languages {
            url.query_pairs_mut()
                .extend_pairs(val.iter().map(|l| (LANGUAGE, l)));
        }
        if let Some(val) = self.first {
            url.query_pairs_mut().append_pair(FIRST, &val.to_string());
        }
        if let Some(val) = self.before {
            url.query_pairs_mut().append_pair(BEFORE, val);
        }
        if let Some(val) = self.after {
            url.query_pairs_mut().append_pair(AFTER, val);
        }

        let req = self.client.http_client().get(url);
        self.client.json(req).await
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GetStreamType {
    All,
    Live,
}

impl AsRef<str> for GetStreamType {
    fn as_ref(&self) -> &str {
        match self {
            GetStreamType::All => "all",
            GetStreamType::Live => "live",
        }
    }
}

#[derive(Debug)]
pub struct GetFollowedStreams<'a> {
    client: &'a Client,
    user_id: &'a UserId,
    first: Option<u8>,
    after: Option<&'a str>,
}

impl<'a> GetFollowedStreams<'a> {
    pub fn new(client: &'a Client, user_id: &'a UserId) -> Self {
        Self {
            client,
            user_id,
            first: None,
            after: None,
        }
    }

    pub fn first(mut self, value: u8) -> Self {
        self.first = Some(value);
        self
    }

    pub fn after(mut self, value: &'a str) -> Self {
        self.after = Some(value);
        self
    }

    pub async fn send(self) -> Result<StreamsResponse, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut().unwrap().extend([STREAMS, FOLLOWED]);

        url.query_pairs_mut().append_pair(USER_ID, self.user_id);
        if let Some(val) = self.first {
            url.query_pairs_mut().append_pair(FIRST, &val.to_string());
        }
        if let Some(val) = self.after {
            url.query_pairs_mut().append_pair(AFTER, val);
        }

        let req = self.client.http_client().get(url);
        self.client.json(req).await
    }
}

#[derive(Debug)]
pub enum StreamMarkerSelect<'a> {
    User(&'a UserId),
    Video(&'a VideoId),
}

impl<'a> From<&'a UserId> for StreamMarkerSelect<'a> {
    fn from(value: &'a UserId) -> Self {
        Self::User(value)
    }
}

impl<'a> From<&'a VideoId> for StreamMarkerSelect<'a> {
    fn from(value: &'a VideoId) -> Self {
        Self::Video(value)
    }
}

impl<'a> StreamMarkerSelect<'a> {
    pub(crate) fn append_to_query(&self, url: &mut url::Url) {
        match self {
            Self::User(id) => {
                url.query_pairs_mut().append_pair(USER_ID, id);
            }
            Self::Video(id) => {
                url.query_pairs_mut().append_pair(VIDEO_ID, id);
            }
        }
    }
}

#[derive(Debug)]
pub struct GetStermaMarkers<'a> {
    client: &'a Client,
    select: StreamMarkerSelect<'a>,
    first: Option<u8>,
    before: Option<&'a str>,
    after: Option<&'a str>,
}

impl<'a> GetStermaMarkers<'a> {
    pub(crate) fn new(client: &'a Client, select: impl Into<StreamMarkerSelect<'a>>) -> Self {
        Self {
            client,
            select: select.into(),
            first: None,
            before: None,
            after: None,
        }
    }

    pub fn first(mut self, value: u8) -> Self {
        self.first = Some(value);
        self
    }

    pub fn before(mut self, value: &'a str) -> Self {
        self.before = Some(value);
        self
    }

    pub fn after(mut self, value: &'a str) -> Self {
        self.after = Some(value);
        self
    }

    pub async fn send(self) -> Result<GetStreamMarkersResponse, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut().unwrap().extend([STREAMS, MARKERS]);

        self.select.append_to_query(&mut url);

        if let Some(value) = self.first {
            url.query_pairs_mut().append_pair(FIRST, &value.to_string());
        }
        if let Some(value) = self.before {
            url.query_pairs_mut().append_pair(BEFORE, value.as_ref());
        }
        if let Some(value) = self.after {
            url.query_pairs_mut().append_pair(AFTER, value.as_ref());
        }

        let req = self.client.http_client().get(url);
        self.client.json(req).await
    }
}
