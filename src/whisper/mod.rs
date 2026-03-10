mod types;

use std::future::Future;

use crate::{
    types::{
        constants::{FROM_USER_ID, TO_USER_ID, WHISPERS},
        UserId,
    },
    Client, Error,
};

use types::WhisperBody;

pub trait WhisperAPI {
    /// Sends a whisper message to the specified user
    /// # Arguments
    ///
    /// * `from_user_id` - this user must have a verified phone number.
    /// * `to_user_id` - user to receive the whisper.
    /// * `message`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     types::UserId,
    ///     whisper::WhisperAPI
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .send_whisper(
    ///         &UserId::from("8456"),
    ///         &UserId::from("2574"),
    ///         "message"
    ///     )
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    /// # Required Scope
    ///
    /// `user:manage:whispers`
    ///
    /// # API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#send-whisper>
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
