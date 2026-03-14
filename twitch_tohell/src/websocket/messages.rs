use std::{fmt, str};

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::{
    EmptyObject, SessionId, Subscription,
    websocket::{MessageType, MetaData},
};

macro_rules! define_message {
    (
        $(#[$meta:meta])*
        struct $name:ident {
            payload: $payload:ty,
            $message_type:expr
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Serialize)]
        pub struct $name {
            pub metadata: MetaData,
            pub payload: $payload,
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                #[derive(Deserialize)]
                #[serde(field_identifier, rename_all = "lowercase")]
                enum Field {
                    Metadata,
                    Payload,
                }

                use serde::de::{Error, Visitor};
                struct EventVisitor;

                impl<'de> Visitor<'de> for EventVisitor {
                    type Value = $name;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str(stringify!($name))
                    }

                    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                    where
                        A: serde::de::MapAccess<'de>,
                    {
                        let mut metadata: Option<MetaData> = None;
                        let mut payload: Option<$payload> = None;

                        while let Some(key) = map.next_key::<Field>()? {
                            match key {
                                Field::Metadata => {
                                    if metadata.is_some() {
                                        return Err(Error::duplicate_field("metadata"));
                                    }

                                    let meta: MetaData = map.next_value()?;
                                    if meta.message_type != $message_type {
                                        return Err(Error::custom(format!(
                                            "Expected message type {:?}, got {:?}",
                                            $message_type, meta.message_type
                                        )));
                                    }

                                    metadata = Some(meta);
                                }
                                Field::Payload => {
                                    if payload.is_some() {
                                        return Err(Error::duplicate_field("payload"));
                                    }
                                    payload = Some(map.next_value()?);
                                }
                            }
                        }

                        let metadata = metadata.ok_or_else(|| Error::missing_field("metadata"))?;
                        let payload = payload.ok_or_else(|| Error::missing_field("payload"))?;

                        Ok($name { metadata, payload })
                    }
                }

                deserializer.deserialize_map(EventVisitor)
            }
        }
    };


    (
        $(#[$meta:meta])*
        struct $name:ident<$generic:ident> {
            payload: $payload:ty,
            $message_type:expr
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Serialize)]
        pub struct $name<$generic> {
            pub metadata: MetaData,
            pub payload: $payload,
        }

        impl<'de, $generic> Deserialize<'de> for $name<$generic>
        where
            $generic: Deserialize<'de>,
        {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                #[derive(Deserialize)]
                #[serde(field_identifier, rename_all = "lowercase")]
                enum Field {
                    Metadata,
                    Payload,
                }

                use serde::de::{Error, Visitor};
                struct EventVisitor<$generic>(std::marker::PhantomData<$generic>);

                impl<'de, $generic> Visitor<'de> for EventVisitor<$generic>
                where
                    $generic: Deserialize<'de>,
                {
                    type Value = $name<$generic>;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str(stringify!($name))
                    }

                    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                    where
                        A: serde::de::MapAccess<'de>,
                    {
                        let mut metadata: Option<MetaData> = None;
                        let mut payload: Option<$payload> = None;

                        while let Some(key) = map.next_key::<Field>()? {
                            match key {
                                Field::Metadata => {
                                    if metadata.is_some() {
                                        return Err(Error::duplicate_field("metadata"));
                                    }

                                    let meta: MetaData = map.next_value()?;
                                    if meta.message_type != $message_type {
                                        return Err(Error::custom(format!(
                                            "Expected message type {:?}, got {:?}",
                                            $message_type, meta.message_type
                                        )));
                                    }

                                    metadata = Some(meta);
                                }
                                Field::Payload => {
                                    if payload.is_some() {
                                        return Err(Error::duplicate_field("payload"));
                                    }
                                    payload = Some(map.next_value()?);
                                }
                            }
                        }

                        let metadata = metadata.ok_or_else(|| Error::missing_field("metadata"))?;
                        let payload = payload.ok_or_else(|| Error::missing_field("payload"))?;

                        Ok($name { metadata, payload })
                    }
                }

                deserializer.deserialize_map(EventVisitor(std::marker::PhantomData))
            }
        }
    };
}

define_message!(
    /// IMPORTANT
    /// By default, you have 10 seconds from the time you receive the Welcome message to subscribe to an event,
    /// unless otherwise specified when connecting.
    /// If you don’t subscribe within this timeframe,
    /// the server closes the connection.
    /// <https://dev.twitch.tv/docs/eventsub/handling-websocket-events/#welcome-message>
    struct Welcome {
        payload: SessionPayload,
        MessageType::SessionWelcome
    }
);

define_message!(
/// <https://dev.twitch.tv/docs/eventsub/handling-websocket-events/#reconnect-message>
    struct Reconnect {
        payload: SessionPayload,
        MessageType::SessionReconnect
    }
);

define_message!(
/// <https://dev.twitch.tv/docs/eventsub/handling-websocket-events/#revocation-message>
    struct Revocation {
        payload: RevocationPayload,
        MessageType::Revocation
    }
);

define_message!(
    /// <https://dev.twitch.tv/docs/eventsub/handling-websocket-events/#keepalive-message>
    // #[serde(deny_unknown_fields)]
    struct Keepalive {
        payload: EmptyObject,
        MessageType::SessionKeepalive
    }
);
define_message!(
/// <https://dev.twitch.tv/docs/eventsub/handling-websocket-events/#notification-message>
    struct Notification<T> {
        payload: NotificationPayload<T>,
        MessageType::Notification
    }
);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionPayload {
    pub session: Session,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// sesion_id
    pub id: SessionId,
    pub status: String,
    pub keepalive_timeout_seconds: Option<u64>,
    pub reconnect_url: Option<String>,
    pub connected_at: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevocationPayload {
    pub subscription: Subscription,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationPayload<T> {
    pub subscription: Subscription,
    pub event: T,
}
