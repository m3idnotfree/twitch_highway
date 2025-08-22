use serde::Serialize;

define_request!(
    #[derive(Debug, Clone, Copy, Serialize)]
    SendWhisperBody<'a> {
        req: {
           message: &'a str
        };
        into_json
    }
);

#[cfg(test)]
mod tests {
    use crate::whispers::request::SendWhisperBody;

    #[test]
    fn send_whisper_body_serialization() {
        let request_body = SendWhisperBody::new("hello");

        let body = request_body.into_json();

        assert!(body.is_some());
        let body = body.unwrap();
        assert_eq!(body, "{\"message\":\"hello\"}");
    }

    #[test]
    fn empty_send_whisper_body_serialization() {
        let request_body = SendWhisperBody::new("");

        let body = request_body.into_json();

        assert!(body.is_some());
        let body = body.unwrap();
        assert_eq!(body, "{\"message\":\"\"}");
    }
}
