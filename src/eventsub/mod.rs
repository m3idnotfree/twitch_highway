use std::collections::HashMap;

use asknothingx2_util::oauth::{AccessToken, ClientId};
use create::CreateEventSub;
use delete::DeleteEventSub;
use get::GetEventSub;

use crate::types::{SubscriptionTypes, Transport};

pub mod create;
pub mod delete;
pub mod get;

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

    pub fn create(&self, kind: SubscriptionTypes, transport: Transport) -> CreateEventSub {
        CreateEventSub::new(
            self.access_token.clone(),
            self.client_id.clone(),
            kind,
            HashMap::new(),
            transport,
        )
    }

    pub fn delete<T: Into<String>>(&self, id: T) -> DeleteEventSub {
        DeleteEventSub::new(self.access_token.clone(), self.client_id.clone(), id.into())
    }

    pub fn get(&self) -> GetEventSub {
        GetEventSub::new(self.access_token.clone(), self.client_id.clone())
    }
}
