use crate::{
    games::GamesResponse,
    types::{
        constants::{AFTER, BEFORE, FIRST, GAMES, ID, IGDB_ID, NAME, TOP},
        GameId,
    },
    Client, Error,
};

#[derive(Debug)]
pub struct GetTopGames<'a> {
    client: &'a Client,
    first: Option<u8>,
    after: Option<&'a str>,
    before: Option<&'a str>,
}

impl<'a> GetTopGames<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self {
            client,
            first: None,
            after: None,
            before: None,
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

    pub fn before(mut self, value: &'a str) -> Self {
        self.before = Some(value);
        self
    }

    pub async fn send(self) -> Result<GamesResponse, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut().unwrap().extend([GAMES, TOP]);

        if let Some(val) = self.first {
            url.query_pairs_mut().append_pair(FIRST, &val.to_string());
        }
        if let Some(val) = self.after {
            url.query_pairs_mut().append_pair(AFTER, val);
        }
        if let Some(val) = self.before {
            url.query_pairs_mut().append_pair(BEFORE, val);
        }

        let req = self.client.http_client().get(url);
        self.client.json(req).await
    }
}

#[derive(Debug)]
pub struct GetGames<'a> {
    client: &'a Client,
    ids: Option<&'a [GameId]>,
    names: Option<&'a [&'a str]>,
    igdb_ids: Option<&'a [&'a str]>,
}

impl<'a> GetGames<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self {
            client,
            ids: None,
            names: None,
            igdb_ids: None,
        }
    }

    pub fn ids(mut self, value: &'a [GameId]) -> Self {
        self.ids = Some(value);
        self
    }

    pub fn names(mut self, value: &'a [&'a str]) -> Self {
        self.names = Some(value);
        self
    }

    pub fn igdb_ids(mut self, value: &'a [&'a str]) -> Self {
        self.igdb_ids = Some(value);
        self
    }

    pub async fn send(self) -> Result<GamesResponse, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut().unwrap().push(GAMES);

        if let Some(ids) = self.ids {
            url.query_pairs_mut()
                .extend_pairs(ids.iter().map(|id| (ID, id)));
        }
        if let Some(names) = self.names {
            url.query_pairs_mut()
                .extend_pairs(names.iter().map(|name| (NAME, name)));
        }
        if let Some(ids) = self.igdb_ids {
            url.query_pairs_mut()
                .extend_pairs(ids.iter().map(|id| (IGDB_ID, id)));
        }

        let req = self.client.http_client().get(url);
        self.client.json(req).await
    }
}
