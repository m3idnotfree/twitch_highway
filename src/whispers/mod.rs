use crate::{base::TwitchAPIBase, EndpointType, TwitchAPI, TwitchAPIRequest};
use asknothingx2_util::api::Method;
use request::SendWhisperRequest;

pub mod request;

pub trait WhisperAPI: TwitchAPIBase {
    /// NOTE: The user sending the whisper must have a verified phone number (see the Phone Number setting in your Security and Privacy settings).
    fn send_whisper(
        &self,
        from_user_id: &str,
        to_user_id: &str,
        message: &str,
    ) -> TwitchAPIRequest<SendWhisperRequest>;
}

impl WhisperAPI for TwitchAPI {
    fn send_whisper(
        &self,
        from_user_id: &str,
        to_user_id: &str,
        message: &str,
    ) -> TwitchAPIRequest<SendWhisperRequest> {
        let mut url = self.build_url();
        url.path(["whispers"])
            .query([("from_user_id", from_user_id), ("to_user_id", to_user_id)]);

        let mut headers = self.build_headers();
        headers.json();

        TwitchAPIRequest::new(
            EndpointType::SendWhisper,
            url.build(),
            Method::POST,
            headers.build(),
            SendWhisperRequest::new(message.to_string()),
        )
    }
}
