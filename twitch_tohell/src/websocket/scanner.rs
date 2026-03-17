use crate::{SubscriptionType, websocket::MessageType};

#[derive(Debug, Clone, Copy)]
pub struct Scanner {
    metadata: liver_shot::Span,
    payload: liver_shot::Span,
    pub message_type: MessageType,
    pub subscription_type: Option<SubscriptionType>,
}

impl Scanner {
    pub fn new(data: &str) -> Result<Self, ScanError> {
        let metadata = liver_shot::find("metadata", data)?;
        let payload = liver_shot::find("payload", data)?;

        let mt_span = metadata.find("message_type", data)?;
        let message_type: MessageType = data[mt_span.start + 1..mt_span.end - 1]
            .parse()
            .map_err(ScanError::parse)?;

        let subscription_type = match metadata.find("subscription_type", data) {
            Ok(span) => {
                let s = &data[span.start + 1..span.end - 1];
                Some(s.parse::<SubscriptionType>().map_err(ScanError::parse)?)
            }
            Err(e) if e.is_not_found() => None,
            Err(e) => return Err(e.into()),
        };

        Ok(Self {
            metadata,
            payload,
            message_type,
            subscription_type,
        })
    }

    #[inline]
    pub fn is_notification(&self) -> bool {
        matches!(self.message_type, MessageType::Notification)
    }

    #[inline]
    pub fn is_welcome(&self) -> bool {
        matches!(self.message_type, MessageType::SessionWelcome)
    }

    #[inline]
    pub fn is_keepalive(&self) -> bool {
        matches!(self.message_type, MessageType::SessionKeepalive)
    }

    #[inline]
    pub fn is_reconnect(&self) -> bool {
        matches!(self.message_type, MessageType::SessionReconnect)
    }

    #[inline]
    pub fn is_revocation(&self) -> bool {
        matches!(self.message_type, MessageType::Revocation)
    }

    #[inline]
    pub fn get_metadata<'a>(&self, data: &'a str) -> &'a str {
        self.metadata.get(data)
    }

    #[inline]
    pub fn get_payload<'a>(&self, data: &'a str) -> &'a str {
        self.payload.get(data)
    }

    #[inline]
    pub fn get_session<'a>(&self, data: &'a str) -> Result<&'a str, ScanError> {
        self.find_in_payload(data, "session")
    }

    #[inline]
    pub fn get_subscription<'a>(&self, data: &'a str) -> Result<&'a str, ScanError> {
        self.find_in_payload(data, "subscription")
    }

    #[inline]
    pub fn get_event<'a>(&self, data: &'a str) -> Result<&'a str, ScanError> {
        self.find_in_payload(data, "event")
    }

    #[inline]
    pub fn get_reconnect_url<'a>(&self, data: &'a str) -> Result<&'a str, ScanError> {
        let session = liver_shot::find("session", data)?;
        find_str(data, &session, "reconnect_url")
    }

    fn find_in_payload<'a>(&self, data: &'a str, field_name: &str) -> Result<&'a str, ScanError> {
        Ok(self.payload.find(field_name, data)?.get(data))
    }
}

fn find_str<'a>(
    data: &'a str,
    span: &liver_shot::Span,
    field_name: &str,
) -> Result<&'a str, ScanError> {
    let value_span = span.find(field_name, data)?;
    Ok(&data[value_span.start + 1..value_span.end - 1])
}

#[derive(Debug, thiserror::Error)]
pub enum ScanError {
    #[error("{0}")]
    Parse(String),
    #[error(transparent)]
    Json(#[from] liver_shot::Error),
}

impl ScanError {
    pub fn parse(error: impl ToString) -> Self {
        Self::Parse(error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Subscription, SubscriptionType,
        websocket::{MessageType, MetaData, Scanner, Session},
    };

    #[test]
    fn welcome() {
        let data = r#"
{
  "metadata": {
    "message_id": "96a3f3b5-5dec-4eed-908e-e11ee657416c",
    "message_type": "session_welcome",
    "message_timestamp": "2023-07-19T14:56:51.634234626Z"
  },
  "payload": {
    "session": {
      "id": "AQoQILE98gtqShGmLD7AM6yJThAB",
      "status": "connected",
      "connected_at": "2023-07-19T14:56:51.616329898Z",
      "keepalive_timeout_seconds": 10,
      "reconnect_url": null
    }
  }
}
"#;

        let scanner = Scanner::new(data).unwrap();

        assert_eq!(scanner.message_type, MessageType::SessionWelcome);
        assert_eq!(scanner.subscription_type, None);

        let _: MetaData = serde_json::from_str(scanner.get_metadata(data)).unwrap();
        let _: Session = serde_json::from_str(scanner.get_session(data).unwrap()).unwrap();
    }

    #[test]
    fn keepalive() {
        let data = r#"
{
    "metadata": {
        "message_id": "84c1e79a-2a4b-4c13-ba0b-4312293e9308",
        "message_type": "session_keepalive",
        "message_timestamp": "2023-07-19T10:11:12.634234626Z"
    },
    "payload": {}
}
"#;

        let scanner = Scanner::new(data).unwrap();

        assert_eq!(scanner.message_type, MessageType::SessionKeepalive);
        assert_eq!(scanner.subscription_type, None);

        let _: MetaData = serde_json::from_str(scanner.get_metadata(data)).unwrap();
    }

    #[test]
    fn notification() {
        let data = r#"
{
    "metadata": {
        "message_id": "befa7b53-d79d-478f-86b9-120f112b044e",
        "message_type": "notification",
        "message_timestamp": "2022-11-16T10:11:12.464757833Z",
        "subscription_type": "channel.follow",
        "subscription_version": "1"
    },
    "payload": {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "status": "enabled",
            "type": "channel.follow",
            "version": "1",
            "cost": 1,
            "condition": {
                "broadcaster_user_id": "12826"
            },
            "transport": {
                "method": "websocket",
                "session_id": "AQoQexAWVYKSTIu4ec_2VAxyuhAB"
            },
            "created_at": "2022-11-16T10:11:12.464757833Z"
        },
        "event": {
            "user_id": "1337",
            "user_login": "awesome_user",
            "user_name": "Awesome_User",
            "broadcaster_user_id": "12826",
            "broadcaster_user_login": "twitch",
            "broadcaster_user_name": "Twitch",
            "followed_at": "2023-07-15T18:16:11.17106713Z"
        }
    }
}
"#;

        let scanner = Scanner::new(data).unwrap();

        assert_eq!(scanner.message_type, MessageType::Notification);
        assert_eq!(
            scanner.subscription_type,
            Some(SubscriptionType::ChannelFollow)
        );

        let _: MetaData = serde_json::from_str(scanner.get_metadata(data)).unwrap();
        let _: Subscription =
            serde_json::from_str(scanner.get_subscription(data).unwrap()).unwrap();
    }

    #[test]
    fn reconnect() {
        let data = r#"
{
    "metadata": {
        "message_id": "84c1e79a-2a4b-4c13-ba0b-4312293e9308",
        "message_type": "session_reconnect",
        "message_timestamp": "2022-11-18T09:10:11.634234626Z"
    },
    "payload": {
        "session": {
           "id": "AQoQexAWVYKSTIu4ec_2VAxyuhAB",
           "status": "reconnecting",
           "keepalive_timeout_seconds": null,
           "reconnect_url": "wss://eventsub.wss.twitch.tv?...",
           "connected_at": "2022-11-16T10:11:12.634234626Z"
        }
    }
}
"#;

        let scanner = Scanner::new(data).unwrap();

        assert_eq!(scanner.message_type, MessageType::SessionReconnect);
        assert_eq!(scanner.subscription_type, None);

        let _: MetaData = serde_json::from_str(scanner.get_metadata(data)).unwrap();
        let _: Session = serde_json::from_str(scanner.get_session(data).unwrap()).unwrap();
    }

    #[test]
    fn revocation() {
        let data = r#"
{

    "metadata": {
        "message_id": "84c1e79a-2a4b-4c13-ba0b-4312293e9308",
        "message_type": "revocation",
        "message_timestamp": "2022-11-16T10:11:12.464757833Z",
        "subscription_type": "channel.follow",
        "subscription_version": "1"
    },
    "payload": {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "status": "authorization_revoked",
            "type": "channel.follow",
            "version": "1",
            "cost": 1,
            "condition": {
                "broadcaster_user_id": "12826"
            },
            "transport": {
                "method": "websocket",
                "session_id": "AQoQexAWVYKSTIu4ec_2VAxyuhAB"
            },
            "created_at": "2022-11-16T10:11:12.464757833Z"
        }
    }
}
"#;
        let scanner = Scanner::new(data).unwrap();

        assert_eq!(scanner.message_type, MessageType::Revocation);
        assert_eq!(
            scanner.subscription_type,
            Some(SubscriptionType::ChannelFollow)
        );

        let _: MetaData = serde_json::from_str(scanner.get_metadata(data)).unwrap();
        let _: Subscription =
            serde_json::from_str(scanner.get_subscription(data).unwrap()).unwrap();
    }
}
