mod create;
use std::{collections::HashMap, sync::Arc};

use asknothingx2_util::oauth::{AccessToken, ClientId};
pub use create::*;

use url::Url;

#[derive(Debug)]
pub struct EventSub {
    access_token: Arc<AccessToken>,
    client_id: Arc<ClientId>,
    url: Url,
}

impl EventSub {
    pub fn new(access_token: &AccessToken, client_id: &ClientId) -> Self {
        Self {
            access_token: Arc::new(access_token.clone()),
            client_id: Arc::new(client_id.clone()),
            url: Url::parse("https://api.twitch.tv/helix/eventsub/subscriptions").unwrap(),
        }
    }

    pub fn set_url(mut self, url: Url) -> Self {
        self.url = url;
        self
    }

    pub fn create<'a>(
        &self,
        kind: &'a str,
        version: &'a str,
        condition: HashMap<&'a str, &'a str>,
        transport_method: TransportMethod,
    ) -> CreateEventSub<'a> {
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
}
