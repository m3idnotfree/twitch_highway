use serde::Serialize;

define_request!(
    #[derive(Debug, Serialize)]
    SendWhisperBody<'a> {
        req: {
           message: &'a str
        };
        to_json
    }
);
