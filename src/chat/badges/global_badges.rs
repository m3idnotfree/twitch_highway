use asknothingx2_util::api::APIRequest;
use url::Url;

crate::impl_endpoint!(
/// https://dev.twitch.tv/docs/api/reference/#get-global-chat-badges
    GetGlobalBadges {};
    new = {
        params = {},
        init = {}
    },
    url = ["chat","badges"]
);

impl APIRequest for GetGlobalBadges {
    crate::impl_api_request_method!(GET);
    crate::impl_api_request_header!();

    fn url(&self) -> Url {
        let mut url = self.get_url();
        url.path_segments_mut().unwrap().push("global");
        url
    }
}
#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{api_general, expect_APIRequest, expect_headers};

    use super::GetGlobalBadges;

    #[test]
    fn global_badges() {
        let global_badges = api_general!(GetGlobalBadges);

        let expected_headers = expect_headers!();

        expect_APIRequest!(
            GET,
            expected_headers,
            "https://api.twitch.tv/helix/chat/badges/global",
            json = None,
            text = None,
            urlencoded = None,
            global_badges
        );
    }
}
