pub mod request;
pub mod response;
pub mod types;

use asknothingx2_util::api::Method;
use request::CclLocale;
use response::CclsResponse;

use crate::request::EndpointType;

endpoints! {
    CclsAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#get-content-classification-labels>
        fn get_content_classification_labels(
            &self,
            locale: Option<CclLocale>,
        ) -> CclsResponse {
            endpoint_type: EndpointType::GetContentClassificationLabels,
            method: Method::GET,
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
