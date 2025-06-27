use serde::Serialize;

define_request!(
    #[derive(Debug, Serialize)]
    SendWhisperBody {
        req: {
           message: String
        };
        into_request_body
    }
);
