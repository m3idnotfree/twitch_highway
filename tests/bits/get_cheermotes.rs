use twitch_highway::types::BroadcasterId;

fn_expected_request!(
    api: twitch_highway::bits::BitsAPI,
    endpoint: get_cheermotes,
    token_type: Any,
    scopes: None,
    args: [None],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/bits/cheermotes",
    json: None,
    text: None,
    urlencoded: None
);

fn_expected_request!(
    name: broadcaster_id,
    api: twitch_highway::bits::BitsAPI,
    endpoint: get_cheermotes,
    token_type: Any,
    scopes: None,
    args: [Some(BroadcasterId::new("41245072"))],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/bits/cheermotes?broadcaster_id=41245072",
    json: None,
    text: None,
    urlencoded: None
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"prefix\": \"Cheer\",\n      \"tiers\": [\n        {\n          \"min_bits\": 1,\n          \"id\": \"1\",\n          \"color\": \"#979797\",\n          \"images\": {\n            \"dark\": {\n              \"animated\": {\n                \"1\": \"https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/animated/1/1.gif\",\n                \"1.5\": \"https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/animated/1/1.5.gif\",\n                \"2\": \"https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/animated/1/2.gif\",\n                \"3\": \"https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/animated/1/3.gif\",\n                \"4\": \"https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/animated/1/4.gif\"\n              },\n              \"static\": {\n                \"1\": \"https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/static/1/1.png\",\n                \"1.5\": \"https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/static/1/1.5.png\",\n                \"2\": \"https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/static/1/2.png\",\n                \"3\": \"https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/static/1/3.png\",\n                \"4\": \"https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/static/1/4.png\"\n              }\n            },\n            \"light\": {\n              \"animated\": {\n                \"1\": \"https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/animated/1/1.gif\",\n                \"1.5\": \"https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/animated/1/1.5.gif\",\n                \"2\": \"https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/animated/1/2.gif\",\n                \"3\": \"https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/animated/1/3.gif\",\n                \"4\": \"https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/animated/1/4.gif\"\n              },\n              \"static\": {\n                \"1\": \"https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/static/1/1.png\",\n                \"1.5\": \"https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/static/1/1.5.png\",\n                \"2\": \"https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/static/1/2.png\",\n                \"3\": \"https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/static/1/3.png\",\n                \"4\": \"https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/static/1/4.png\"\n              }\n            }\n          },\n          \"can_cheer\": true,\n          \"show_in_bits_card\": true\n        }\n\n      ],\n      \"type\": \"global_first_party\",\n      \"order\": 1,\n      \"last_updated\": \"2018-05-22T00:06:04Z\",\n      \"is_charitable\": false\n    }\n  ]\n}",
    module: twitch_highway::bits::response::CheermotesResponse,
    de: CheermotesResponse
);
