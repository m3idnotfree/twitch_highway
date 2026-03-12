mod request;
mod response;
mod types;

pub use request::CclLocale;
pub use response::CclsResponse;
pub use types::Ccl;

use std::future::Future;

use crate::{
    types::constants::{CONTENT_CLASSIFICATION_LABELS, LOCALE},
    Client, Error,
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
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_content_classification_labels(Some(CclLocale::deDE))
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
    ) -> impl Future<Output = Result<CclsResponse, Error>> + Send;
}

impl CclsAPI for Client {
    async fn get_content_classification_labels(
        &self,
        locale: Option<CclLocale>,
    ) -> Result<CclsResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .push(CONTENT_CLASSIFICATION_LABELS);

        if let Some(l) = locale {
            url.query_pairs_mut().append_pair(LOCALE, l.as_ref());
        }

        self.json(self.http_client().get(url)).await
    }
}
