use asknothingx2_util::api::Method;
use request::CclsLocale;
use response::CclsResponse;

use crate::{base::TwitchAPIBase, EmptyBody, EndpointType, TwitchAPI, TwitchAPIRequest};

pub mod request;
pub mod response;
pub mod types;

pub trait CclsAPI: TwitchAPIBase {
    fn get_content_classification_labels(
        &self,
        locale: Option<CclsLocale>,
    ) -> TwitchAPIRequest<EmptyBody, CclsResponse>;
}

impl CclsAPI for TwitchAPI {
    fn get_content_classification_labels(
        &self,
        locale: Option<CclsLocale>,
    ) -> TwitchAPIRequest<EmptyBody, CclsResponse> {
        let mut url = self.build_url();
        url.path(["content_classification_labels"])
            .query_opt("locale", locale);

        TwitchAPIRequest::new(
            EndpointType::GetContentClassificationLabels,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
}
