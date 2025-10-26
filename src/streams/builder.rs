use crate::{
    request::TwitchAPIRequest,
    streams::{GetStreamMarkersResponse, StreamsResponse},
    types::{
        constants::{
            AFTER, BEFORE, FIRST, FOLLOWED, GAME_ID, LANGUAGE, MARKERS, STREAMS, TYPE, USER_ID,
            USER_LOGIN, VIDEO_ID,
        },
        GameId, UserId, VideoId,
    },
    TwitchAPI,
};

define_request_builder! {
    #[derive(Debug)]
    GetStreamsBuilder<'a> {
            user_ids	:&'a [UserId] [key = USER_ID, convert = extend],
            user_logins:&'a [&'a str] [key = USER_LOGIN, convert = extend],
            game_ids:&'a [GameId] [key = GAME_ID, convert = extend],
            kind: GetStreamType [key = TYPE, convert = as_ref],
            languages: &'a [&'a str] [key = LANGUAGE, convert = extend],
            first: u8 [key = FIRST, convert = to_string],
            before: &'a str [key = BEFORE],
            after: &'a str [key = AFTER],

    } -> StreamsResponse;
    endpoint: GetStreams,
    method: GET,
    path: [STREAMS],
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

define_request_builder! {
    #[derive(Debug)]
    GetFollowedStreamsBuilder<'a> {
        req: {user_id: &'a UserId [key = USER_ID]},
        opts: {
            first: u8 [key = FIRST, convert = to_string],
            after: &'a str [key = AFTER],
        }
    } -> StreamsResponse;
    endpoint: GetFollowedStreams,
    method: GET,
    path: [STREAMS, FOLLOWED],
}

#[derive(Debug)]
pub enum StreamMarkerSelect<'a> {
    User(&'a UserId),
    Video(&'a VideoId),
}

#[derive(Debug)]
pub struct GetStermaMarkersBuilder<'a> {
    api: &'a TwitchAPI,
    select: StreamMarkerSelect<'a>,
    first: Option<u8>,
    before: Option<&'a str>,
    after: Option<&'a str>,
}

impl<'a> GetStermaMarkersBuilder<'a> {
    pub fn user_id(api: &'a TwitchAPI, user_id: &'a UserId) -> Self {
        Self {
            api,
            select: StreamMarkerSelect::User(user_id),
            first: None,
            before: None,
            after: None,
        }
    }

    pub fn video_id(api: &'a TwitchAPI, video_id: &'a VideoId) -> Self {
        Self {
            api,
            select: StreamMarkerSelect::Video(video_id),
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

    pub fn build(self) -> TwitchAPIRequest<GetStreamMarkersResponse> {
        let mut url = self.api.build_url();

        url.path_segments_mut().unwrap().extend(&[STREAMS, MARKERS]);

        let mut query = url.query_pairs_mut();

        match self.select {
            StreamMarkerSelect::User(id) => {
                query.append_pair(USER_ID, id);
            }
            StreamMarkerSelect::Video(id) => {
                query.append_pair(VIDEO_ID, id);
            }
        }

        if let Some(value) = self.first {
            query.append_pair(FIRST, &value.to_string());
        }

        if let Some(value) = self.before {
            query.append_pair(BEFORE, value.as_ref());
        }
        if let Some(value) = self.after {
            query.append_pair(AFTER, value.as_ref());
        }

        drop(query);

        TwitchAPIRequest::new(
            crate::request::EndpointType::GetStreamMarkers,
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

    pub async fn json(self) -> Result<GetStreamMarkersResponse, crate::Error> {
        self.build().json().await
    }
}
