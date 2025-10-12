use crate::{
    request::TwitchAPIRequest,
    types::{
        constants::{
            AFTER, BEFORE, FIRST, GAME_ID, ID, LANGUAGE, PERIOD, SORT, TYPE, USER_ID, VIDEOS,
        },
        GameId, UserId, VideoId,
    },
    videos::{Period, Sort, Type, VideosResponse},
    TwitchAPI,
};

#[derive(Debug)]
pub enum VideoSelect<'a> {
    Ids(&'a [VideoId]),
    User(&'a UserId),
    Game(&'a GameId),
}

#[derive(Debug)]
pub struct GetVideosBuilder<'a> {
    api: &'a TwitchAPI,
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
    pub fn game_id(api: &'a TwitchAPI, game_id: &'a GameId) -> Self {
        Self {
            api,
            select: VideoSelect::Game(game_id),
            language: None,
            period: None,
            sort: None,
            kind: None,
            first: None,
            after: None,
            before: None,
        }
    }
    pub fn user_id(api: &'a TwitchAPI, user_id: &'a UserId) -> Self {
        Self {
            api,
            select: VideoSelect::User(user_id),
            language: None,
            period: None,
            sort: None,
            kind: None,
            first: None,
            after: None,
            before: None,
        }
    }
    pub fn ids(api: &'a TwitchAPI, ids: &'a [VideoId]) -> Self {
        Self {
            api,
            select: VideoSelect::Ids(ids),
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

    pub fn build(self) -> TwitchAPIRequest<VideosResponse> {
        let mut url = self.api.build_url();

        url.path_segments_mut().unwrap().extend(&[VIDEOS]);

        let mut query = url.query_pairs_mut();

        match self.select {
            VideoSelect::User(id) => {
                query.append_pair(USER_ID, id);
            }
            VideoSelect::Game(id) => {
                query.append_pair(GAME_ID, id);
            }
            VideoSelect::Ids(ids) => {
                query.extend_pairs(ids.iter().map(|id| (ID, id)));
            }
        }

        if let Some(value) = self.language {
            query.append_pair(LANGUAGE, value);
        }
        if let Some(value) = self.period {
            query.append_pair(PERIOD, value.as_ref());
        }
        if let Some(value) = self.sort {
            query.append_pair(SORT, value.as_ref());
        }
        if let Some(value) = self.kind {
            query.append_pair(TYPE, value.as_ref());
        }

        if let Some(value) = self.first {
            query.append_pair(FIRST, value.as_ref());
        }

        if let Some(value) = self.before {
            query.append_pair(BEFORE, value.as_ref());
        }
        if let Some(value) = self.after {
            query.append_pair(AFTER, value.as_ref());
        }

        drop(query);

        TwitchAPIRequest::new(
            crate::request::EndpointType::GetVideos,
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

    pub async fn json(self) -> Result<VideosResponse, crate::Error> {
        self.build().json().await
    }
}
