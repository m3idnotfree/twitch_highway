use crate::{
    base::TwitchAPIBase, types::constants::WHISPERS, EmptyBody, EndpointType, TwitchAPI,
    TwitchAPIRequest,
};
use asknothingx2_util::api::Method;
use request::SendWhisperBody;

pub mod request;

pub trait WhisperAPI: TwitchAPIBase {
    /// NOTE: The user sending the whisper must have a verified phone number (see the Phone Number setting in your Security and Privacy settings).
    fn send_whisper(
        &self,
        from_user_id: &str,
        to_user_id: &str,
        message: &str,
    ) -> TwitchAPIRequest<SendWhisperBody, EmptyBody>;
}

impl WhisperAPI for TwitchAPI {
    fn send_whisper(
        &self,
        from_user_id: &str,
        to_user_id: &str,
        message: &str,
    ) -> TwitchAPIRequest<SendWhisperBody, EmptyBody> {
        let mut url = self.build_url();
        url.path([WHISPERS])
            .query_extend([("from_user_id", from_user_id), ("to_user_id", to_user_id)]);

        let mut headers = self.build_headers();
        headers.json();

        TwitchAPIRequest::new(
            EndpointType::SendWhisper,
            url.build(),
            Method::POST,
            headers.build(),
            SendWhisperBody::new(message),
        )
    }
}
