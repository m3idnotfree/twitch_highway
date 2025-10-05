use serde::Serialize;

use crate::{
    eventsub::{subscription_types::SubscriptionType, transport::TransportType, Condition},
    types::{Status, UserId},
};

#[derive(Debug, Clone, Serialize)]
pub struct CreateEventSubRequest<T: TransportType> {
    #[serde(rename = "type")]
    pub kind: SubscriptionType,
    pub version: String,
    pub condition: Condition,
    pub transport: T,
}

impl<T> CreateEventSubRequest<T>
where
    T: TransportType + Serialize,
{
    pub fn new(kind: SubscriptionType, condition: Condition, transport: T) -> Self {
        Self {
            version: kind.version().to_string(),
            kind,
            condition,
            transport,
        }
    }

    pub fn into_json(self) -> Option<String> {
        Some(serde_json::to_string(&self).unwrap())
    }
}

define_request! {
    #[derive(Debug, Clone, Serialize)]
    GetEventRequest<'a> {
        opts: {
            status: Status => "status",
            #[serde(rename = "type")]
            kind: SubscriptionType => TYPE,
            user_id: &'a UserId => USER_ID,
        };
        into_query
    }
}
