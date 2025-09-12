mod request;
mod response;
mod types;

pub use request::CclLocale;
pub use response::CclsResponse;
pub use types::Ccl;

endpoints! {
    CclsAPI {
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

#[cfg(test)]
mod tests {
    use crate::ccls::CclsAPI;

    api_test!(get_content_classification_labels, [None]);
}
