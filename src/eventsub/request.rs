use serde::Serialize;

use crate::{
    eventsub::{
        subscription::Status, subscription_types::SubscriptionType, transport::TransportType,
    },
    types::UserId,
};

#[derive(Debug, Serialize)]
pub struct CreateEventSubRequest<Condition, T: TransportType> {
    #[serde(rename = "type")]
    pub kind: SubscriptionType,
    pub version: String,
    pub condition: Condition,
    pub transport: T,
}

impl<Condition, T> CreateEventSubRequest<Condition, T>
where
    Condition: Serialize,
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
    #[derive(Debug, Serialize)]
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
