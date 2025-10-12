mod request;

use request::SendWhisperBody;

use crate::{
    request::NoContent,
    types::{
        constants::{FROM_USER_ID, TO_USER_ID},
        UserId,
    },
};

const WHISPERS: &str = "whispers";

endpoints! {
    WhisperAPI {
        /// Sends a whisper message to the specified user
        /// # Arguments
        ///
        /// * `from_user_id` - this user must have a verified phone number.
        /// * `to_user_id` - user to receive the whisper.
        ///
        /// # Return
        ///
        /// Return a [`NoContent`]
        ///
        /// # Example
        ///
        /// ```rust
        /// # use twitch_highway::TwitchAPI;
        /// use twitch_highway::{
        ///     types::UserId,
        ///     whisper::WhisperAPI
        /// };
        ///
        /// # async fn example(api: TwitchAPI) -> Result<(), Box<dyn std::error::Error>> {
        /// let response = api
        ///     .send_whisper(
        ///         &UserId::from("8456"),
        ///         &UserId::from("2574"),
        ///         "message"
        ///     )
        ///     .send()
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
        ) -> NoContent {
            endpoint_type: SendWhisper,
            method: POST,
            path: [WHISPERS],
            query_params: {
                query(FROM_USER_ID, from_user_id),
                query(TO_USER_ID, to_user_id)
            },
            headers: [json],
            body: SendWhisperBody::new(message).into_json()
        }
    }
}
