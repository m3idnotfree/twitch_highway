use asknothingx2_util::api::APIRequest;
use url::Url;

crate::impl_endpoint!(
/// https://dev.twitch.tv/docs/api/reference/#delete-eventsub-subscription
    DeleteEventSub {
        id: String,
    };
    new = {
        params = {
            id: impl Into<String>
        },
        init = {
            id: id.into(),
        }
    },
    url = ["eventsub", "subscriptions"]

);

impl APIRequest for DeleteEventSub {
    crate::impl_api_request_method!(DELETE);
    crate::impl_api_request_header!();

    fn url(&self) -> Url {
        let mut url = self.get_url();
        url.query_pairs_mut().append_pair("id", &self.id);

        url
    }
}

#[cfg(test)]
mod tests {
    use crate::{api_general, expect_APIRequest, expect_headers};

    use super::DeleteEventSub;

    #[test]
    fn delete_eventsub() {
        let delete_eventsub = api_general!(DeleteEventSub, "26b1c993-bfcf-44d9-b876-379dacafe75a");

        let expected_headers = expect_headers!();

        expect_APIRequest!(
            DELETE,
            expected_headers,
            "https://api.twitch.tv/helix/eventsub/subscriptions?id=26b1c993-bfcf-44d9-b876-379dacafe75a",
            json = None,
            text = None,
            urlencoded = None,
            delete_eventsub
        );
    }
}
