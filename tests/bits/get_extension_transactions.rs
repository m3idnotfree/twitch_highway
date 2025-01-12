use twitch_highway::bits::request::ExtensionTransactionRequest;

fn_expected_request!(
    api: twitch_highway::bits::BitsAPI,
    endpoint: get_extension_transactions,
    token_type: App,
    scopes: None,
    args: [ExtensionTransactionRequest::new("1234".to_string())],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/extensions/transactions?extension_id=1234",
    json: None,
    text: None,
    urlencoded: None
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"id\": \"74c52265-e214-48a6-91b9-23b6014e8041\",\n      \"timestamp\": \"2019-01-28T04:15:53.325Z\",\n      \"broadcaster_id\": \"439964613\",\n      \"broadcaster_login\": \"chikuseuma\",\n      \"broadcaster_name\": \"chikuseuma\",\n      \"user_id\": \"424596340\",\n      \"user_login\": \"quotrok\",\n      \"user_name\": \"quotrok\",\n      \"product_type\": \"BITS_IN_EXTENSION\",\n      \"product_data\": {\n        \"domain\": \"twitch.ext.uo6dggojyb8d6soh92zknwmi5ej1q2\",\n        \"sku\": \"testSku100\",\n        \"cost\": {\n          \"amount\": 100,\n          \"type\": \"bits\"\n        },\n        \"inDevelopment\": false,\n        \"displayName\": \"Test Product 100\",\n        \"expiration\": \"\",\n        \"broadcast\": false\n      }\n    },\n    {\n      \"id\": \"8d303dc6-a460-4945-9f48-59c31d6735cb\",\n      \"timestamp\": \"2019-01-18T09:10:13.397Z\",\n      \"broadcaster_id\": \"439964613\",\n      \"broadcaster_login\": \"chikuseuma\",\n      \"broadcaster_name\": \"chikuseuma\",\n      \"user_id\": \"439966926\",\n      \"user_login\": \"liscuit\",\n      \"user_name\": \"liscuit\",\n      \"product_type\": \"BITS_IN_EXTENSION\",\n      \"product_data\": {\n        \"domain\": \"twitch.ext.uo6dggojyb8d6soh92zknwmi5ej1q2\",\n        \"sku\": \"testSku200\",\n        \"cost\": {\n          \"amount\": 200,\n          \"type\": \"bits\"\n        },\n        \"inDevelopment\": false,\n        \"displayName\": \"Test Product 200\",\n        \"expiration\": \"\",\n        \"broadcast\": false\n      }\n    }\n  ],\n  \"pagination\": {\n    \"cursor\": \"cursorString\"\n  }\n}",
    module: twitch_highway::bits::response::ExtensionTransactionsResponse,
    de: ExtensionTransactionsResponse
);
