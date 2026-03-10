use crate::{
    types::{
        constants::{
            AFTER, BEFORE, FIRST, GAME_ID, ID, LANGUAGE, PERIOD, SORT, TYPE, USER_ID, VIDEOS,
        },
        GameId, UserId, VideoId,
    },
    videos::{Period, Sort, Type, VideosResponse},
    Client, Error,
};

#[derive(Debug)]
pub enum VideoSelect<'a> {
    User(&'a UserId),
    Game(&'a GameId),
    Ids(&'a [VideoId]),
}

impl<'a> From<&'a UserId> for VideoSelect<'a> {
    fn from(value: &'a UserId) -> Self {
        Self::User(value)
    }
}

impl<'a> From<&'a GameId> for VideoSelect<'a> {
    fn from(value: &'a GameId) -> Self {
        Self::Game(value)
    }
}

impl<'a> From<&'a Vec<VideoId>> for VideoSelect<'a> {
    fn from(value: &'a Vec<VideoId>) -> Self {
        Self::Ids(value)
    }
}

impl<'a> From<&'a [VideoId]> for VideoSelect<'a> {
    fn from(value: &'a [VideoId]) -> Self {
        Self::Ids(value)
    }
}

impl<'a, const N: usize> From<&'a [VideoId; N]> for VideoSelect<'a> {
    fn from(value: &'a [VideoId; N]) -> Self {
        Self::Ids(value)
    }
}

impl<'a> VideoSelect<'a> {
    pub(crate) fn append_to_query(&self, url: &mut url::Url) {
        match self {
            Self::User(id) => {
                url.query_pairs_mut().append_pair(USER_ID, id);
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
pub struct GetVideosBuilder<'a> {
    client: &'a Client,
    select: VideoSelect<'a>,
    language: Option<&'a str>,
    period: Option<Period>,
    sort: Option<Sort>,
    kind: Option<Type>,
    first: Option<&'a str>,
    after: Option<&'a str>,
    before: Option<&'a str>,
}

impl<'a> GetVideosBuilder<'a> {
    pub(crate) fn new(client: &'a Client, select: impl Into<VideoSelect<'a>>) -> Self {
        Self {
            client,
            select: select.into(),
            language: None,
            period: None,
            sort: None,
            kind: None,
            first: None,
            after: None,
            before: None,
        }
    }

    pub fn period(mut self, value: Period) -> Self {
        self.period = Some(value);
        self
    }

    pub fn sort(mut self, value: Sort) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn kind(mut self, value: Type) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn first(mut self, value: &'a str) -> Self {
        self.first = Some(value);
        self
    }

    pub fn after(mut self, value: &'a str) -> Self {
        self.after = Some(value);
        self
    }

    pub fn before(mut self, value: &'a str) -> Self {
        self.before = Some(value);
        self
    }

    pub async fn send(self) -> Result<VideosResponse, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut().unwrap().push(VIDEOS);

        self.select.append_to_query(&mut url);

        if let Some(value) = self.language {
            url.query_pairs_mut().append_pair(LANGUAGE, value);
        }
        if let Some(value) = self.period {
            url.query_pairs_mut().append_pair(PERIOD, value.as_ref());
        }
        if let Some(value) = self.sort {
            url.query_pairs_mut().append_pair(SORT, value.as_ref());
        }
        if let Some(value) = self.kind {
            url.query_pairs_mut().append_pair(TYPE, value.as_ref());
        }

        if let Some(value) = self.first {
            url.query_pairs_mut().append_pair(FIRST, value.as_ref());
        }

        if let Some(value) = self.before {
            url.query_pairs_mut().append_pair(BEFORE, value.as_ref());
        }
        if let Some(value) = self.after {
            url.query_pairs_mut().append_pair(AFTER, value.as_ref());
        }

        self.client.json(self.client.http_client().get(url)).await
    }
}
