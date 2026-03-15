mod types;

use std::future::Future;

use crate::{
    Client, Error,
    types::{
        UserId,
        constants::{FROM_USER_ID, TO_USER_ID, WHISPERS},
    },
};

use types::WhisperBody;

pub trait WhisperAPI {
    /// See <https://dev.twitch.tv/docs/api/reference/#send-whisper>
    fn send_whisper(
        &self,
        from_user_id: &UserId,
        to_user_id: &UserId,
        message: &str,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

impl WhisperAPI for Client {
    async fn send_whisper(
        &self,
        from_user_id: &UserId,
        to_user_id: &UserId,
        message: &str,
    ) -> Result<(), Error> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().push(WHISPERS);

        url.query_pairs_mut()
            .append_pair(FROM_USER_ID, from_user_id)
            .append_pair(TO_USER_ID, to_user_id);

        let req = self.http_client().post(url).json(&WhisperBody { message });
        self.no_content(req).await
    }
}
