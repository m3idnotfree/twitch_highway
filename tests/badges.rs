mod support;

#[cfg(feature = "chat")]
#[test]
fn badge_response() {
    use twitch_highway::chat::badges::BadgeResponse;

    expect_response_json!("{\n  \"data\": [\n    {\n      \"set_id\": \"bits\",\n      \"versions\": [\n        {\n          \"id\": \"1\",\n          \"image_url_1x\": \"https://static-cdn.jtvnw.net/badges/v1/743a0f3b-84b3-450b-96a0-503d7f4a9764/1\",\n          \"image_url_2x\": \"https://static-cdn.jtvnw.net/badges/v1/743a0f3b-84b3-450b-96a0-503d7f4a9764/2\",\n          \"image_url_4x\": \"https://static-cdn.jtvnw.net/badges/v1/743a0f3b-84b3-450b-96a0-503d7f4a9764/3\",\n          \"title\": \"cheer 1\",\n          \"description\": \"cheer 1\",\n          \"click_action\": \"visit_url\",\n          \"click_url\": \"https://bits.twitch.tv\"\n        }\n      ]\n    },\n    {\n      \"set_id\": \"subscriber\",\n      \"versions\": [\n        {\n          \"id\": \"0\",\n          \"image_url_1x\": \"https://static-cdn.jtvnw.net/badges/v1/eb4a8a4c-eacd-4f5e-b9f2-394348310442/1\",\n          \"image_url_2x\": \"https://static-cdn.jtvnw.net/badges/v1/eb4a8a4c-eacd-4f5e-b9f2-394348310442/2\",\n          \"image_url_4x\": \"https://static-cdn.jtvnw.net/badges/v1/eb4a8a4c-eacd-4f5e-b9f2-394348310442/3\",\n          \"title\": \"Subscriber\",\n          \"description\": \"Subscriber\",\n          \"click_action\": \"subscribe_to_channel\",\n          \"click_url\": null\n        }\n      ]\n    }\n  ]\n}",
            BadgeResponse
    );
}

#[cfg(feature = "chat")]
#[test]
fn channel_badges() {
    use twitch_highway::chat::badges::GetChannelBadge;

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

#[cfg(feature = "chat")]
#[test]
fn global_badges() {
    use twitch_highway::chat::badges::GetGlobalBadges;

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
