use std::sync::Arc;

use asknothingx2_util::{
    api::{APIRequest, HeaderBuilder, HeaderMap, Method},
    oauth::{AccessToken, ClientId},
};
use url::Url;

/// https://dev.twitch.tv/docs/api/reference/#delete-eventsub-subscription
#[derive(Debug)]
pub struct DeleteEventSub {
    access_token: Arc<AccessToken>,
    client_id: Arc<ClientId>,
    url: Arc<Url>,
    id: String,
}

impl DeleteEventSub {
    pub fn new<T: Into<String>>(
        access_token: Arc<AccessToken>,
        client_id: Arc<ClientId>,
        url: Arc<Url>,
        id: T,
    ) -> Self {
        Self {
            access_token,
            client_id,
            url,
            id: id.into(),
        }
    }
}

impl APIRequest for DeleteEventSub {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn headers(&self) -> HeaderMap {
        HeaderBuilder::new()
            .authorization("Bearer", self.access_token.secret().as_str())
            .client_id(self.client_id.as_str())
            .build()
    }

    fn url(&self) -> Url {
        let mut url = Url::parse(self.url.as_str()).unwrap();
        url.query_pairs_mut().append_pair("id", &self.id);

        url
    }
}

#[cfg(test)]
mod tests {
    use crate::{api_general, expect_APIRequest, expect_headers};
    use asknothingx2_util::api::APIRequest;

    use super::DeleteEventSub;

    #[test]
    fn delete_eventsub() {
        let delete_eventsub = api_general!(
            DeleteEventSub,
            "https://api.twitch.tv/helix/eventsub/subscriptions",
            "26b1c993-bfcf-44d9-b876-379dacafe75a"
        );

        let expected_headers = expect_headers!();

        expect_APIRequest!(
            DELETE,
            expected_headers,
            "https://api.twitch.tv/helix/eventsub/subscriptions?id=26b1c993-bfcf-44d9-b876-379dacafe75a",
            delete_eventsub
        );
    }
}
