use asknothingx2_util::api::APIRequest;
use url::Url;

crate::impl_endpoint!(
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
    crate::impl_api_request_method!(GET);
    crate::impl_api_request_header!();

    fn url(&self) -> Url {
        let mut url = self.get_url();
        url.query_pairs_mut()
            .append_pair("broadcaster_id", &self.broadcaster_id);
        url
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{api_general, expect_APIRequest, expect_headers};

    use super::GetChannelBadge;

    #[test]
    fn channel_badges() {
        let channel_badges = api_general!(GetChannelBadge, "135093069");

        let expected_headers = expect_headers!();

        expect_APIRequest!(
            GET,
            expected_headers,
            "https://api.twitch.tv/helix/chat/badges?broadcaster_id=135093069",
            json = None,
            text = None,
            urlencoded = None,
            channel_badges
        );
    }
}
