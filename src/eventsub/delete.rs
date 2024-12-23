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
