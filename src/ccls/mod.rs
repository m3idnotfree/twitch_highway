mod request;
mod response;
mod types;

pub use request::CclLocale;
pub use response::CclsResponse;
pub use types::Ccl;

use crate::{
    request::TwitchAPIRequest,
    types::constants::{CONTENT_CLASSIFICATION_LABELS, LOCALE},
    Client,
};

pub trait CclsAPI {
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
    /// # use twitch_highway::Client;
    /// use twitch_highway::ccls::{CclLocale, CclsAPI};
    ///
    /// # async fn example(api: Client) -> Result<(), Box<dyn std::error::Error>> {
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
    ) -> TwitchAPIRequest<CclsResponse>;
}

impl CclsAPI for Client {
    simple_endpoint!(
    fn get_content_classification_labels(
        locale: Option<CclLocale> [opt, key = LOCALE, convert = as_ref],
    ) -> CclsResponse;
        endpoint: GetContentClassificationLabels,
        method: GET,
        path: [CONTENT_CLASSIFICATION_LABELS],
    );
}
