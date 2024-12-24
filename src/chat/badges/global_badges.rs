use asknothingx2_util::api::APIRequest;
use url::Url;

endpoint!(
/// https://dev.twitch.tv/docs/api/reference/#get-global-chat-badges
    GetGlobalBadges {};
    new = {
        params = {},
        init = {}
    },
    url = ["chat","badges"]
);

impl APIRequest for GetGlobalBadges {
    impl_api_request_method!(GET);
    impl_api_request_header!();

    fn url(&self) -> Url {
        let mut url = self.get_url();
        url.path_segments_mut().unwrap().push("global");
        url
    }
}
