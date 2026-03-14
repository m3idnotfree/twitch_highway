use crate::{SubscriptionType, websocket::MessageType};

const METADATA: &str = "\"metadata\"";
const PAYLOAD: &str = "\"payload\"";
const EVENT: &str = "\"event\"";
const SUBSCRIPTION: &str = "\"subscription\"";
const SESSION: &str = "\"session\"";
const SUBSCRIPTION_TYPE: &str = "\"subscription_type\"";
const MESSAGE_TYPE: &str = "\"message_type\"";
const RECONNECT_URL: &str = "\"reconnect_url\"";

#[derive(Debug, Default, Clone, Copy)]
struct Section {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Scanner {
    metadata: Section,
    payload: Section,
    pub message_type: MessageType,
    pub subscription_type: Option<SubscriptionType>,
}

impl Scanner {
    pub fn new(data: &str) -> Result<Self, ScanError> {
        let metadata = InternalScanner::find_section(data, METADATA)?;
        let payload = InternalScanner::find_section(data, PAYLOAD)?;

        let message_type: MessageType = Self::extract_string_field(data, &metadata, MESSAGE_TYPE)?
            .parse()
            .map_err(ScanError::parse_error)?;

        let subscription_type = Self::extract_string_field(data, &metadata, SUBSCRIPTION_TYPE)
            .ok()
            .map(|s| {
                s.parse::<SubscriptionType>()
                    .map_err(ScanError::parse_error)
            })
            .transpose()?;

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
        &data[self.metadata.start..self.metadata.end]
    }

    #[inline]
    pub fn get_payload<'a>(&self, data: &'a str) -> &'a str {
        &data[self.payload.start..self.payload.end]
    }

    #[inline]
    pub fn get_session<'a>(&self, data: &'a str) -> Result<&'a str, ScanError> {
        self.extract_object_from_payload(data, SESSION)
    }

    #[inline]
    pub fn get_subscription<'a>(&self, data: &'a str) -> Result<&'a str, ScanError> {
        self.extract_object_from_payload(data, SUBSCRIPTION)
    }

    #[inline]
    pub fn get_event<'a>(&self, data: &'a str) -> Result<&'a str, ScanError> {
        self.extract_object_from_payload(data, EVENT)
    }

    #[inline]
    pub fn get_reconnect_url<'a>(&self, data: &'a str) -> Result<&'a str, ScanError> {
        let session = InternalScanner::find_section(data, SESSION)?;
        Self::extract_string_field(data, &session, RECONNECT_URL)
    }

    #[inline]
    fn extract_object_from_payload<'a>(
        &self,
        data: &'a str,
        field_name: &str,
    ) -> Result<&'a str, ScanError> {
        let payload = self.get_payload(data);
        let start = InternalScanner::find_section_start(payload, field_name)?;
        let end = InternalScanner::skip_object(payload, start)?;

        Ok(&payload[start..end])
    }

    #[inline]
    fn extract_string_field<'a>(
        data: &'a str,
        section: &Section,
        field_name: &str,
    ) -> Result<&'a str, ScanError> {
        let payload = &data[section.start..section.end];
        let start = InternalScanner::find_section_start(payload, field_name)?;
        let end = InternalScanner::skip_string(payload, start)?;

        Ok(&payload[start + 1..end - 1])
    }
}

struct InternalScanner;

impl InternalScanner {
    #[inline]
    pub fn find_section(data: &str, pattern: &str) -> Result<Section, ScanError> {
        let start = Self::find_section_start(data, pattern)?;
        let end = Self::skip_object(data, start)?;
        Ok(Section { start, end })
    }

    #[inline]
    pub fn find_section_start(data: &str, pattern: &str) -> Result<usize, ScanError> {
        let end = data
            .find(pattern)
            .ok_or(ScanError::field_not_found(pattern, "JSON input"))?
            + pattern.len();

        let bytes = data.as_bytes();
        let mut pos = end;

        while pos < bytes.len() {
            match bytes[pos] {
                b':' => {
                    pos += 1; // skip colon

                    while pos < bytes.len() && matches!(bytes[pos], b' ' | b'\t' | b'\n' | b'\r') {
                        pos += 1;
                    }

                    return Ok(pos);
                }
                b' ' | b'\t' | b'\n' | b'\r' => pos += 1,
                _ => {
                    return Err(ScanError::InvalidFieldFormat {
                        field_name: pattern.to_string(),
                        position: pos,
                        expected: "whitespace or ':'".to_string(),
                    });
                }
            }
        }

        Err(ScanError::unexpected_eof(pos, "':'"))
    }

    /// The input position must point to an opening brace '{'
    fn skip_object(data: &str, start_pos: usize) -> Result<usize, ScanError> {
        let bytes = data.as_bytes();
        if start_pos >= bytes.len() {
            return Err(ScanError::out_of_bounds(start_pos, bytes.len()));
        }

        if bytes[start_pos] != b'{' {
            return Err(ScanError::invalid_json_at(start_pos, "expected '{'"));
        }

        let mut pos = start_pos + 1; // skip '{'
        let mut depth = 1;

        while pos < bytes.len() && depth > 0 {
            match bytes[pos] {
                b'{' => depth += 1,
                b'}' => depth -= 1,
                _ => {}
            }
            pos += 1;
        }

        if depth != 0 {
            Err(ScanError::unmatched_brackets("{", start_pos, depth))
        } else {
            Ok(pos)
        }
    }

    /// The input position must point to an opening quote '"'
    #[inline]
    fn skip_string(data: &str, start_pos: usize) -> Result<usize, ScanError> {
        let bytes = data.as_bytes();

        if start_pos >= bytes.len() {
            return Err(ScanError::out_of_bounds(start_pos, bytes.len()));
        }

        if bytes[start_pos] != b'"' {
            return Err(ScanError::invalid_json_at(start_pos, "expected '\"'"));
        }

        let mut pos = start_pos + 1; // skip quote

        while pos < bytes.len() {
            match bytes[pos] {
                b'"' => {
                    return Ok(pos + 1);
                }
                b'\\' => pos += 2, // skip escaped character
                _ => pos += 1,
            }
        }

        Err(ScanError::unterminated_string(start_pos))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ScanError {
    #[error("Position {position} is out of bounds for input of length {input_length}")]
    OutOfBounds {
        position: usize,
        input_length: usize,
    },
    #[error("Invalid JSON at position {position}: {reason}")]
    InvalidJsonAt { position: usize, reason: String },
    #[error("Unterminated string starting at position {start_position}")]
    UnterminatedString { start_position: usize },
    #[error("Unmatched {bracket_type} at position {position} (depth: {depth})")]
    UnmatchedBrackets {
        bracket_type: String,
        position: usize,
        depth: i32,
    },
    #[error("Field '{field_name}' not found in {context}")]
    FieldNotFound { field_name: String, context: String },
    #[error("Invalid format for field '{field_name}' at position {position}: expected {expected}")]
    InvalidFieldFormat {
        field_name: String,
        position: usize,
        expected: String,
    },
    #[error("Unexpected end of input at position {position}: expected {expected}")]
    UnexpectedEof { expected: String, position: usize },
    #[error("{0}")]
    ParseError(String),
}

impl ScanError {
    pub fn invalid_json_at(position: usize, reason: impl Into<String>) -> Self {
        Self::InvalidJsonAt {
            position,
            reason: reason.into(),
        }
    }

    pub fn field_not_found(field_name: impl Into<String>, context: impl Into<String>) -> Self {
        Self::FieldNotFound {
            field_name: field_name.into(),
            context: context.into(),
        }
    }

    pub fn unterminated_string(start_position: usize) -> Self {
        Self::UnterminatedString { start_position }
    }

    pub fn unmatched_brackets(
        bracket_type: impl Into<String>,
        position: usize,
        depth: i32,
    ) -> Self {
        Self::UnmatchedBrackets {
            bracket_type: bracket_type.into(),
            position,
            depth,
        }
    }

    pub fn unexpected_eof(position: usize, expected: impl Into<String>) -> Self {
        Self::UnexpectedEof {
            position,
            expected: expected.into(),
        }
    }

    pub fn parse_error(error: impl Into<String>) -> Self {
        Self::ParseError(error.into())
    }

    pub fn out_of_bounds(position: usize, input_length: usize) -> Self {
        Self::OutOfBounds {
            position,
            input_length,
        }
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
