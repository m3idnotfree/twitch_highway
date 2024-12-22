mod create;
mod delete;
mod get;
use std::collections::HashMap;

use asknothingx2_util::oauth::{AccessToken, ClientId};

pub use create::*;
pub use delete::*;
pub use get::*;

use crate::SubscriptionTypes;

#[derive(Debug)]
pub struct EventSubAPI {
    access_token: AccessToken,
    client_id: ClientId,
}

impl EventSubAPI {
    pub fn new(access_token: AccessToken, client_id: ClientId) -> Self {
        Self {
            access_token,
            client_id,
        }
    }

    pub fn create(
        &self,
        kind: SubscriptionTypes,
        transport_method: TransportMethod,
    ) -> CreateEventSub {
        CreateEventSub::new(
            self.access_token.clone(),
            self.client_id.clone(),
            kind,
            HashMap::new(),
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
