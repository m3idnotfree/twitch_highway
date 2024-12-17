mod create;
mod delete;
use std::{collections::HashMap, sync::Arc};

use asknothingx2_util::oauth::{AccessToken, ClientId};
pub use create::*;
pub use delete::*;

use once_cell::sync::Lazy;
use url::Url;

#[derive(Debug)]
pub struct EventSubAPI {
    access_token: Arc<AccessToken>,
    client_id: Arc<ClientId>,
    url: Arc<Url>,
}

const TWITCH_EVENTSUB_URL: &str = "https://api.twitch.tv/helix/eventsub/subscriptions";

static EVENTSUB_URL: Lazy<Arc<Url>> =
    Lazy::new(|| Arc::new(Url::parse(TWITCH_EVENTSUB_URL).unwrap()));

impl EventSubAPI {
    pub fn new(access_token: &AccessToken, client_id: &ClientId) -> Self {
        Self {
            access_token: Arc::new(access_token.clone()),
            client_id: Arc::new(client_id.clone()),
            url: EVENTSUB_URL.clone(),
        }
    }

    pub fn set_url(mut self, url: Url) -> Self {
        self.url = Arc::new(url);
        self
    }

    pub fn create<T: Into<String>>(
        &self,
        kind: T,
        version: T,
        condition: HashMap<String, String>,
        transport_method: TransportMethod,
    ) -> CreateEventSub {
        CreateEventSub::new(
            self.access_token.clone(),
            self.client_id.clone(),
            self.url.clone(),
            kind,
            version,
            condition,
            transport_method,
        )
    }

    pub fn delete<T: Into<String>>(&self, id: T) -> DeleteEventSub {
        DeleteEventSub::new(
            self.access_token.clone(),
            self.client_id.clone(),
            self.url.clone(),
            id.into(),
        )
    }
}
