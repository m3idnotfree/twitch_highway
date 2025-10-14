use crate::{
    request::{NoContent, TwitchAPIRequest},
    types::{
        constants::{FROM_USER_ID, TO_USER_ID, WHISPERS},
        UserId,
    },
    TwitchAPI,
};

pub trait WhisperAPI {
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
    ) -> TwitchAPIRequest<NoContent>;
}

impl WhisperAPI for TwitchAPI {
    simple_endpoint!(
        fn send_whisper(
            from_user_id: &UserId [key = FROM_USER_ID],
            to_user_id: &UserId [key = TO_USER_ID],
            message: &str [skip],
        ) -> NoContent;
            endpoint: SendWhisper,
            method: POST,
            path: [WHISPERS],
            headers: [json],
            body: {
                Some(serde_json::json!({"message":message}).to_string())
            }
    );
}
