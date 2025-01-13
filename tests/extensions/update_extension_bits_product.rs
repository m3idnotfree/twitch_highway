use twitch_highway::{
    extensions::types::BitsProductExtension,
    types::{Cost, CostType},
};

fn_expected_request!(
    api: twitch_highway::extensions::ExtensionsAPI,
    endpoint: update_extension_bits_products,
    token_type: Any,
    scopes: None,
    args: [
        BitsProductExtension::new(
            "1010".to_string(),
            Cost::new(990, CostType::Bits),
            "Rusty Crate 2".to_string()
        )
        .in_development(true)
        .is_broadcast(true)
        .expiration("2021-05-18T09:10:13.397Z".to_string())
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
