fn_expected_request!(
    api: twitch_highway::extensions::ExtensionsAPI,
    endpoint: get_extension_bits_products,
    token_type: Any,
    scopes: None,
    args: [Some(true)],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/bits/extensions?should_include_all=true"
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"sku\": \"1010\",\n      \"cost\": {\n        \"amount\": 990,\n        \"type\": \"bits\"\n      },\n      \"in_development\": true,\n      \"display_name\": \"Rusty Crate 2\",\n      \"expiration\": \"2021-05-18T09:10:13.397Z\",\n      \"is_broadcast\": false\n    }\n  ]\n}",
    module: twitch_highway::extensions::response::ExtensionsBitsProductsResponse,
    de: ExtensionsBitsProductsResponse
);
