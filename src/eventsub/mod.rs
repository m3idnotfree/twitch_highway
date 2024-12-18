mod create;
mod delete;
mod get;
use std::{collections::HashMap, sync::Arc};

use asknothingx2_util::oauth::{AccessToken, ClientId};

pub use create::*;
pub use delete::*;
pub use get::*;

#[derive(Debug)]
pub struct EventSubAPI {
    access_token: Arc<AccessToken>,
    client_id: Arc<ClientId>,
}

impl EventSubAPI {
    pub fn new(access_token: &AccessToken, client_id: &ClientId) -> Self {
        Self {
            access_token: Arc::new(access_token.clone()),
            client_id: Arc::new(client_id.clone()),
        }
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
            kind,
            version,
            condition,
            transport_method,
        )
    }

    pub fn delete<T: Into<String>>(&self, id: T) -> DeleteEventSub {
        DeleteEventSub::new(self.access_token.clone(), self.client_id.clone(), id.into())
    }

    pub fn get(&self) -> GetEventSub {
        GetEventSub::new(self.access_token.clone(), self.client_id.clone())
    }
}
