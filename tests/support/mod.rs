macro_rules! expected_APIRequest {
    (
        $actual:expr,
        method: $method:ident
        $(, $name:ident: $expected:expr)*$(,)?
    ) => {
        use asknothingx2_util::api::APIRequest;
        expected_APIRequest!(@method $method, $actual);
        $(expected_APIRequest!(@$name $expected, $actual));*
    };
    (@url $expected:expr, $actual:expr) => {
        assert_eq!($expected, $actual.url().to_string());
    };
    (@method $expected:ident,$actual:expr) => {
        assert_eq!(asknothingx2_util::api::Method::$expected, $actual.method());
    };
    (@header $expected:expr, $actual:expr) => {
        assert_eq!($expected, $actual.headers());
    };
    (@json $expected:expr, $actual:expr) => {
        assert_eq!($expected, $actual.json());
    };
    (@text $expected:expr, $actual:expr) => {
        assert_eq!($expected, $actual.text());
    };
    (@urlencoded  $expected:expr, $actual:expr) => {
        assert_eq!($expected, $actual.urlencoded());
    };
}

macro_rules! api {
    () => {
        twitch_highway::TwitchAPI::new(
            asknothingx2_util::oauth::AccessToken::new(
                "cfabdegwdoklmawdzdo98xt2fo512y".to_string(),
            ),
            asknothingx2_util::oauth::ClientId::new("uo6dggojyb8d6soh92zknwmi5ej1q2".to_string()),
        )
    };
}

macro_rules! expected_headers {
    () => {
        asknothingx2_util::api::HeaderBuilder::new()
            .authorization("Bearer", "cfabdegwdoklmawdzdo98xt2fo512y")
            .append("Client-Id", "uo6dggojyb8d6soh92zknwmi5ej1q2")
            .unwrap()
            .build()
    };
    (json) => {
        asknothingx2_util::api::HeaderBuilder::new()
            .content_type_json()
            .authorization("Bearer", "cfabdegwdoklmawdzdo98xt2fo512y")
            .append("Client-Id", "uo6dggojyb8d6soh92zknwmi5ej1q2")
            .unwrap()
            .build()
    };
}

macro_rules! expected_response {
    ($data:literal,$de:ident) => {
        let result = serde_json::from_str::<$de>($data);

        result.unwrap();
    };
}

macro_rules! fn_expected_request {
    (
        api: $api:ty,
        endpoint: $endpoint:ident,
        token_type: $token_type:ident,
        scopes: $scopes:expr,
        args: [$($params:expr),+],
        method: $method:ident
        $(, $name:ident: $expected:expr)+$(,)?
    ) => {
        #[test]
        fn request() {
            #![allow(unused_imports)]
            use $api;
            use asknothingx2_util::api::APIRequest;
            use twitch_oauth_token::types::Scope;
            use twitch_highway::TokenType;

            let api = api!();
            let endpoint = api.$endpoint($($params),+);
            assert_eq!(TokenType::$token_type, endpoint.kind().token_type());
            assert_eq!($scopes, endpoint.kind().required_scopes());
            expected_APIRequest!(@method $method, endpoint);
            $(expected_APIRequest!(@$name $expected, endpoint);)+
        }
    };
    (
        api: $api:ty,
        endpoint: $endpoint:ident,
        token_type: $token_type:ident,
        scopes: $scopes:expr,
        method: $method:ident
        $(, $name:ident: $expected:expr)+$(,)?
    ) => {
        #[test]
        fn request() {
            #![allow(unused_imports)]
            use $api;
            use asknothingx2_util::api::APIRequest;
            use twitch_oauth_token::types::Scope;
            use twitch_highway::TokenType;

            let api = api!();
            let endpoint = api.$endpoint();
            assert_eq!(TokenType::$token_type, endpoint.kind().token_type());
            assert_eq!($scopes, endpoint.kind().required_scopes());
            expected_APIRequest!(@method $method, endpoint);
            $(expected_APIRequest!(@$name $expected, endpoint);)+
        }
    };

    (
        name:$sub_name:ident,
        api: $api:ty,
        endpoint: $endpoint:ident,
        token_type: $token_type:ident,
        scopes: $scopes:expr,
        args: [$($params:expr),+],
        method: $method:ident
        $(, $name:ident: $expected:expr)+$(,)?
    ) => {
        #[test]
        fn $sub_name() {
            #![allow(unused_imports)]
            use $api;
            use asknothingx2_util::api::APIRequest;
            use twitch_oauth_token::types::Scope;
            use twitch_highway::TokenType;

            let api = api!();
            let endpoint = api.$endpoint($($params),+);
            assert_eq!(TokenType::$token_type, endpoint.kind().token_type());
            assert_eq!($scopes, endpoint.kind().required_scopes());
            expected_APIRequest!(@method $method, endpoint);
            $(expected_APIRequest!(@$name $expected, endpoint);)+
        }
    };
}

macro_rules! fn_expected_resopnse {
    (
    payload: $data:literal,
    module: $module:ty,
    de: $response:ident
    ) => {
        #[test]
        fn response() {
            use $module;
            expected_response!($data, $response);
        }
    };

    (
    name:$sub_name:ident,
    payload: $data:literal,
    module: $module:ty,
    de: $response:ident
    ) => {
        #[test]
        fn $sub_name() {
            use $module;
            expected_response!($data, $response);
        }
    };
}
