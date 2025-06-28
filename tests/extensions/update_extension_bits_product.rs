fn_expected_request!(
    modules: [
        twitch_highway::extensions::ExtensionsAPI,
        twitch_highway::extensions::request::UpdateExtensoinBitsProductsRequest,
        twitch_highway::types::Cost,
        twitch_highway::types::CostType
    ],
    endpoint: update_extension_bits_products,
    token_type: Any,
    scopes: None,
    args: [
        "1010",
        Cost::new(990, CostType::Bits),
        "Rusty Crate 2",
        Some(
            UpdateExtensoinBitsProductsRequest::new()
                .in_development(true)
                .is_broadcast(true)
                .expiration("2021-05-18T09:10:13.397Z")
        )
    ],
    method: PUT,
    header: expected_headers!(json),
    url: "https://api.twitch.tv/helix/bits/extensions"
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"sku\": \"1010\",\n      \"cost\": {\n        \"amount\": 990,\n        \"type\": \"bits\"\n      },\n      \"in_development\": true,\n      \"display_name\": \"Rusty Crate 2\",\n      \"expiration\": \"2021-05-18T09:10:13.397Z\",\n      \"is_broadcast\": true\n    }\n  ]\n}",
    module: twitch_highway::extensions::response::ExtensionsBitsProductsResponse,
    de: ExtensionsBitsProductsResponse
);
