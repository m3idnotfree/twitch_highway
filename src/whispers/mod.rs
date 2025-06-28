use asknothingx2_util::api::Method;
use request::SendWhisperBody;

use crate::{
    request::{EmptyBody, EndpointType, TwitchAPIRequest},
    types::constants::WHISPERS,
    TwitchAPI,
};

pub mod request;

#[cfg_attr(docsrs, doc(cfg(feature = "whispers")))]
pub trait WhisperAPI {
    /// <https://dev.twitch.tv/docs/api/reference/#send-whisper>
    fn send_whisper(
        &self,
        from_user_id: &str,
        to_user_id: &str,
        message: &str,
    ) -> TwitchAPIRequest<EmptyBody>;
}

impl WhisperAPI for TwitchAPI {
    fn send_whisper(
        &self,
        from_user_id: &str,
        to_user_id: &str,
        message: &str,
    ) -> TwitchAPIRequest<EmptyBody> {
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
            SendWhisperBody::new(message).to_json(),
        )
    }
}
