pub mod request;

use request::SendWhisperBody;

use crate::{request::NoContent, types::UserId};

const WHISPERS: &str = "whispers";

endpoints! {
    WhisperAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#send-whisper>
        fn send_whisper(
            &self,
            from_user_id: &UserId,
            to_user_id: &UserId,
            message: &str,
        ) -> NoContent {
            endpoint_type: SendWhisper,
            method: POST,
            path: [WHISPERS],
            query_params: {
                query("from_user_id", from_user_id),
                query("to_user_id", to_user_id)
            },
            headers: [json],
            body: SendWhisperBody::new(message).into_json()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{types::UserId, whispers::WhisperAPI};

    api_test!(
        send_whisper,
        [&UserId::from("123"), &UserId::from("456"), "hello"]
    );
}
