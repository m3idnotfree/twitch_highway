mod request;
mod response;
mod types;

pub use request::CclLocale;
pub use response::CclsResponse;
pub use types::Ccl;

endpoints! {
    CclsAPI {
        /// Gets information about Twitch content classification labels
        ///
        /// # Arguments
        ///
        /// * `locale` - Optional locale for the Content Classification Labels.
        ///
        /// # Returns
        ///
        /// Returns a [`CclsResponse`]
        ///
        /// # Example
        ///
        /// ```rust
        /// # use twitch_highway::TwitchAPI;
        /// use twitch_highway::ccls::{CclLocale, CclsAPI};
        ///
        /// # async fn example(api: TwitchAPI) -> Result<(), Box<dyn std::error::Error>> {
        /// let response = api
        ///     .get_content_classification_labels(Some(CclLocale::deDE))
        ///     .json()
        ///     .await?;
        ///
        /// # Ok(())
        /// # }
        /// ```
        ///
        /// # Required Scope
        ///
        /// No scope required
        ///
        /// API Reference
        ///
        /// <https://dev.twitch.tv/docs/api/reference/#get-content-classification-labels>
        fn get_content_classification_labels(
            &self,
            locale: Option<CclLocale>,
        ) -> CclsResponse {
            endpoint_type: GetContentClassificationLabels,
            method: GET,
            path: ["content_classification_labels"],
            query_params: {
                opt("locale", locale)
            }
        }
    }
}
