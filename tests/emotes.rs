mod support;

#[cfg(feature = "chat")]
#[test]
fn channel_emotes() {
    use twitch_highway::chat::emotes::GetChannelEmotes;

    let channel_emotes = api_general!(GetChannelEmotes, "141981764");

    let expected_headers = expect_headers!();

    expect_APIRequest!(
        GET,
        expected_headers,
        "https://api.twitch.tv/helix/chat/emotes?broadcaster_id=141981764",
        json = None,
        text = None,
        urlencoded = None,
        channel_emotes
    );
}

#[cfg(feature = "chat")]
#[test]
fn channel_emotes_response() {
    use twitch_highway::chat::emotes::EmoteChannelResponse;

    expect_response_json!("{\n  \"data\": [\n    {\n      \"id\": \"304456832\",\n      \"name\": \"twitchdevPitchfork\",\n      \"images\": {\n        \"url_1x\": \"https://static-cdn.jtvnw.net/emoticons/v2/304456832/static/light/1.0\",\n        \"url_2x\": \"https://static-cdn.jtvnw.net/emoticons/v2/304456832/static/light/2.0\",\n        \"url_4x\": \"https://static-cdn.jtvnw.net/emoticons/v2/304456832/static/light/3.0\"\n      },\n      \"tier\": \"1000\",\n      \"emote_type\": \"subscriptions\",\n      \"emote_set_id\": \"301590448\",\n      \"format\": [\n        \"static\"\n      ],\n      \"scale\": [\n        \"1.0\",\n        \"2.0\",\n        \"3.0\"\n      ],\n      \"theme_mode\": [\n        \"light\",\n        \"dark\"\n      ]\n    },\n    {\n      \"id\": \"emotesv2_4c3b4ed516de493bbcd2df2f5d450f49\",\n      \"name\": \"twitchdevHyperPitchfork\",\n      \"images\": {\n        \"url_1x\": \"https://static-cdn.jtvnw.net/emoticons/v2/emotesv2_4c3b4ed516de493bbcd2df2f5d450f49/static/light/1.0\",\n        \"url_2x\": \"https://static-cdn.jtvnw.net/emoticons/v2/emotesv2_4c3b4ed516de493bbcd2df2f5d450f49/static/light/2.0\",\n        \"url_4x\": \"https://static-cdn.jtvnw.net/emoticons/v2/emotesv2_4c3b4ed516de493bbcd2df2f5d450f49/static/light/3.0\"\n      },\n      \"tier\": \"1000\",\n      \"emote_type\": \"subscriptions\",\n      \"emote_set_id\": \"318939165\",\n      \"format\": [\n        \"static\",\n        \"animated\"\n      ],\n      \"scale\": [\n        \"1.0\",\n        \"2.0\",\n        \"3.0\"\n      ],\n      \"theme_mode\": [\n        \"light\",\n        \"dark\"\n      ]\n    }\n  ],\n  \"template\": \"https://static-cdn.jtvnw.net/emoticons/v2/{{id}}/{{format}}/{{theme_mode}}/{{scale}}\"\n}",
EmoteChannelResponse);
}

#[cfg(feature = "chat")]
#[test]
fn emote_sets() {
    use twitch_highway::chat::emotes::GetEmoteSets;

    let emote_sets = api_general!(GetEmoteSets);
    let emote_sets = emote_sets.add_emote_set_id("1234");

    let expected_headers = expect_headers!();

    expect_APIRequest!(
        GET,
        expected_headers,
        "https://api.twitch.tv/helix/chat/emotes/set?emote_set_id=1234",
        json = None,
        text = None,
        urlencoded = None,
        emote_sets
    );
}
#[cfg(feature = "chat")]
#[test]
fn emotes_sets_id_vec() {
    use twitch_highway::chat::emotes::GetEmoteSets;

    let emote_sets = api_general!(GetEmoteSets);
    let emote_sets = emote_sets.add_emote_set_ids(vec!["1234", "4567"]);

    let expected_headers = expect_headers!();

    expect_APIRequest!(
        GET,
        expected_headers,
        "https://api.twitch.tv/helix/chat/emotes/set?emote_set_id=1234&emote_set_id=4567",
        json = None,
        text = None,
        urlencoded = None,
        emote_sets
    );
}

#[cfg(feature = "chat")]
#[test]
fn emote_sets_response() {
    use twitch_highway::chat::emotes::EmoteSetsResponse;

    expect_response_json!("{\n  \"data\": [\n    {\n      \"id\": \"304456832\",\n      \"name\": \"twitchdevPitchfork\",\n      \"images\": {\n        \"url_1x\": \"https://static-cdn.jtvnw.net/emoticons/v2/304456832/static/light/1.0\",\n        \"url_2x\": \"https://static-cdn.jtvnw.net/emoticons/v2/304456832/static/light/2.0\",\n        \"url_4x\": \"https://static-cdn.jtvnw.net/emoticons/v2/304456832/static/light/3.0\"\n      },\n      \"emote_type\": \"subscriptions\",\n      \"emote_set_id\": \"301590448\",\n      \"owner_id\": \"141981764\",\n      \"format\": [\n        \"static\"\n      ],\n      \"scale\": [\n        \"1.0\",\n        \"2.0\",\n        \"3.0\"\n      ],\n      \"theme_mode\": [\n        \"light\",\n        \"dark\"\n      ]\n    }\n  ],\n  \"template\": \"https://static-cdn.jtvnw.net/emoticons/v2/{{id}}/{{format}}/{{theme_mode}}/{{scale}}\"\n}",
        EmoteSetsResponse);
}

#[cfg(feature = "chat")]
#[test]
fn global_emtoes() {
    use twitch_highway::chat::emotes::GetGlobalEmotes;

    let global_emotes = api_general!(GetGlobalEmotes);

    let expected_headers = expect_headers!();

    expect_APIRequest!(
        GET,
        expected_headers,
        "https://api.twitch.tv/helix/chat/emotes/global",
        json = None,
        text = None,
        urlencoded = None,
        global_emotes
    );
}

#[cfg(feature = "chat")]
#[test]
fn global_emotes_response() {
    use twitch_highway::chat::emotes::EmoteGlobalResponse;

    expect_response_json!("{\n  \"data\": [\n    {\n      \"id\": \"196892\",\n      \"name\": \"TwitchUnity\",\n      \"images\": {\n        \"url_1x\": \"https://static-cdn.jtvnw.net/emoticons/v2/196892/static/light/1.0\",\n        \"url_2x\": \"https://static-cdn.jtvnw.net/emoticons/v2/196892/static/light/2.0\",\n        \"url_4x\": \"https://static-cdn.jtvnw.net/emoticons/v2/196892/static/light/3.0\"\n      },\n      \"format\": [\n        \"static\"\n      ],\n      \"scale\": [\n        \"1.0\",\n        \"2.0\",\n        \"3.0\"\n      ],\n      \"theme_mode\": [\n        \"light\",\n        \"dark\"\n      ]\n    }\n  ],\n  \"template\": \"https://static-cdn.jtvnw.net/emoticons/v2/{{id}}/{{format}}/{{theme_mode}}/{{scale}}\"\n}",
        EmoteGlobalResponse);
}
