use asknothingx2_util::api::APIRequest;
use url::Url;

endpoint!(
/// https://dev.twitch.tv/docs/api/reference/#get-channel-chat-badges
    GetChannelBadge {
        broadcaster_id: String,
    };
    new = {
        params = {
            broadcaster_id: impl Into<String>,
       },
        init = {
            broadcaster_id: broadcaster_id.into(),
        }
    },
    url = ["chat","badges"]
);

impl APIRequest for GetChannelBadge {
    impl_api_request_method!(GET);
    impl_api_request_header!();

    fn url(&self) -> Url {
        let mut url = self.get_url();
        url.query_pairs_mut()
            .append_pair("broadcaster_id", &self.broadcaster_id);
        url
    }
}
