#![allow(unused_macros)]
#![allow(unused_imports)]

mod token;
pub use token::*;

macro_rules! general_id {
    () => {};
    ($broadcaster_id:ident ) => {
        let $broadcaster_id = twitch_highway::types::BroadcasterId::new(
            std::env::var("TEST_BROADCASTER_ID")
                .expect("TEST_USER_ID environment variable is not set"),
        );
    };
    ($broadcaster_id:ident, $broadcaster_id_secend:ident) => {
        let $broadcaster_id = twitch_highway::types::BroadcasterId::new(
            std::env::var("TEST_BROADCASTER_ID")
                .expect("TEST_USER_ID environment variable is not set"),
        );
        let $broadcaster_id_secend = twitch_highway::types::BroadcasterId::new(
            std::env::var("TEST_USER_SECEND_ID")
                .expect("TEST_USER_ID environment variable is not set"),
        );
    };
}

macro_rules! expected_APIRequest {
    (
        $actual:expr,
        method: $method:ident
        $(, $name:ident: $expected:expr)*$(,)?
    ) => {
        //use asknothingx2_util::api::APIRequest;
        expected_APIRequest!(@method $method, $actual);
        $(expected_APIRequest!(@$name $expected, $actual));*
    };
    (@url $expected:expr, $actual:expr) => {
        pretty_assertions::assert_eq!($expected, $actual.url().to_string());
    };
    (@method $expected:ident,$actual:expr) => {
        pretty_assertions::assert_eq!(asknothingx2_util::api::Method::$expected, $actual.method());
    };
    (@header $expected:expr, $actual:expr) => {
        pretty_assertions::assert_eq!($expected, $actual.headers());
    };
    (@json $expected:expr, $actual:expr) => {
        pretty_assertions::assert_eq!($expected, $actual.json());
    };
    (@text $expected:expr, $actual:expr) => {
        pretty_assertions::assert_eq!($expected, $actual.text());
    };
    (@urlencoded  $expected:expr, $actual:expr) => {
        pretty_assertions::assert_eq!($expected, $actual.urlencoded());
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
    () => {{
        let mut headers = asknothingx2_util::api::HeaderBuilder::new();
        headers
            .authorization("Bearer", "cfabdegwdoklmawdzdo98xt2fo512y")
            .append("Client-Id", "uo6dggojyb8d6soh92zknwmi5ej1q2")
            .unwrap();

        headers.build()
    }};
    (json) => {{
        let mut headers = asknothingx2_util::api::HeaderBuilder::new();
        headers
            .content_type_json()
            .authorization("Bearer", "cfabdegwdoklmawdzdo98xt2fo512y")
            .append("Client-Id", "uo6dggojyb8d6soh92zknwmi5ej1q2")
            .unwrap();

        headers.build()
    }};
    (jwt) => {{
        let mut headers = asknothingx2_util::api::HeaderBuilder::new();
        headers
            .authorization("Bearer", "<your JWT token>")
            .append("Client-Id", "uo6dggojyb8d6soh92zknwmi5ej1q2")
            .unwrap();

        headers.build()
    }};
}

macro_rules! expected_response {
    ($data:literal,$de:ident) => {
        let result = serde_json::from_str::<$de>($data);
        if let Err(ref error) = result {
            eprintln!("deserialize Error : {:#?}", error);
            result.unwrap();
        } else {
            result.unwrap();
        }
    };
}

macro_rules! fn_expected_request {
    (
        modules: [$($module:path),+],
        endpoint: $endpoint:ident,
        token_type: $token_type:ident,
        scopes: $scopes:expr,
        $(args: [$($params:expr),+],)?
        $(json_contain: [$($contain:expr),+],)?
        method: $method:ident
        $(, $name:ident: $expected:expr)+

    ) => {
        fn_expected_request!(
            name: request,
            modules: [$($module),+],
            endpoint: $endpoint,
            token_type: $token_type,
            scopes: $scopes,
            $(args: [$($params),+],)?
            $(json_contain: [$($contain),+],)?
            method: $method
            $(, $name: $expected)+
        );
    };
    (
        name:$sub_name:ident,
        modules: [$($module:path),+],
        endpoint: $endpoint:ident,
        token_type: $token_type:ident,
        scopes: $scopes:expr,
        $(args: [$($params:expr),+],)?
        $(json_contain: [$($contain:expr),+],)?
        method: $method:ident
        $(, $name:ident: $expected:expr)+
    ) => {
        #[test]
        fn $sub_name() {
            $(use $module;)+
            use asknothingx2_util::api::APIRequest;
            use twitch_highway::TokenType;

            let api = api!();
            let endpoint = api.$endpoint($($($params),+)?);

            fn_expected_request!(@required $token_type $scopes, endpoint);
            expected_APIRequest!(@method $method, endpoint);
            $(expected_APIRequest!(@$name $expected, endpoint);)+
        $(
            let js_contain = endpoint.json().unwrap();
            $(assert!(js_contain.contains($contain));)+

        )?
        }
    };
    (
        api: $api:ty,
        endpoint: $endpoint:ident,
        token_type: $token_type:ident,
        scopes: $scopes:expr,
        $(args: [$($params:expr),+],)?
        $(json_contain: [$($contain:expr),+],)?
        method: $method:ident
        $(, $name:ident: $expected:expr)+

    ) => {
        fn_expected_request!(
            name: request,
            api: $api,
            endpoint: $endpoint,
            token_type: $token_type,
            scopes: $scopes,
            $(args: [$($params),+],)?
            $(json_contain: [$($contain),+],)?
            method: $method
            $(, $name: $expected)+
        );
    };
    (
        name:$sub_name:ident,
        api: $api:ty,
        endpoint: $endpoint:ident,
        token_type: $token_type:ident,
        scopes: $scopes:expr,
        $(args: [$($params:expr),+],)?
        $(json_contain: [$($contain:expr),+],)?
        method: $method:ident
        $(, $name:ident: $expected:expr)+
    ) => {
        #[test]
        fn $sub_name() {
            #![allow(unused_imports)]
            use $api;
            use asknothingx2_util::api::APIRequest;
            use twitch_oauth_token::types::Scope;
            use twitch_highway::TokenType;

            let api = api!();
            let endpoint = api.$endpoint($($($params),+)?);

            fn_expected_request!(@required $token_type $scopes, endpoint);
            expected_APIRequest!(@method $method, endpoint);
            $(expected_APIRequest!(@$name $expected, endpoint);)+
        $(
            let js_contain = endpoint.json().unwrap();
            $(assert!(js_contain.contains($contain));)+

        )?
        }
    };
    (@required $token_type:ident $scopes:expr, $actual:expr) => {
        pretty_assertions::assert_eq!(TokenType::$token_type, $actual.kind().token_type());
        pretty_assertions::assert_eq!($scopes, $actual.kind().required_scopes());
    };
}

macro_rules! fn_expected_resopnse {
    (
    payload: $data:literal,
    module: $module:ty,
    de: $response:ident
    ) => {
        fn_expected_resopnse!(name: response, payload: $data, module:$module, de: $response);
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

macro_rules! fn_mock_server {
    (
        $api:ty;
        token_type: $token_type:expr,
        name:$sub_name:ident,
        endpoint: $endpoint:ident,
        scopes: $scopes:expr,
        args:|$param:ident| {$($($arg:expr),+)?},
        url: $paths:expr
        $(, status:$status:ident)?
        $(, response: $response:ty)?
    ) => {
        #[tokio::test]
        async fn $sub_name() {
            #![allow(unused_imports,unused_variables)]
            use asknothingx2_util::api::api_request;
            use twitch_highway::{
                types::{BroadcasterId, Id},
                TestUrl, TestUrlHold, TwitchAPI,
            };
            use twitch_oauth_token::types::Scope;
            use $api;

            use crate::support::user_token;

            let (token, client_id, _, $param) = $token_type($scopes).await.unwrap();
            let api = TwitchAPI::new(token.access_token, client_id)
                .$endpoint($($($arg),+)?)
                .with_url($paths);

            let response = api
                .request()
                .await
                .unwrap();
                //.expect(format!("Falied request get block list:{:#?}",string$sub_name));
                //.expect(format!("Falied request get block list:{:#?}",string$sub_name));
            $(assert_eq!(asknothingx2_util::api::StatusCode::$status, response.status());)?

            $(response
                .into_json::<$response>()
                .unwrap();
                //.expect("Falied Parse block user list response");
            )?
        }
    };
}

macro_rules! new_fn_mock_server {
    (
        $api: ty;
        name: $sub_name:ident,
        token_type: $token_type:ident,
        endpoint: $endpoint:ident,
        scopes: $scopes:expr,
        args:|$param:ident| {$($($arg:expr),+)?},
        url: $paths:expr
        $(, status: $status:ident)?
        $(, error_text: $error_text:expr)?
        $(, response: $response:ty)?
    ) => {
        #[tokio::test]
        async fn $sub_name() {
            #![allow(unused_imports,unused_variables)]
            use asknothingx2_util::api::api_request;
            use twitch_highway::{
                types::{BroadcasterId, Id},
                TestUrl, TestUrlHold, TwitchAPI,
            };
            use twitch_oauth_token::types::Scope;
            use $api;

            use crate::support::user_token;

            let (token, client_id, _, $param) = new_fn_mock_server!(@$token_type $scopes);
            let api = TwitchAPI::new(token.access_token, client_id)
                .$endpoint($($($arg),+)?)
                .with_url($paths);

            let response = api
                .request()
                .await
                .unwrap();

            $(assert_eq!($error_text, response.raw_body());)?
            $(assert_eq!(asknothingx2_util::api::StatusCode::$status, response.status());)?
            $(response
                .into_json::<$response>()
                .unwrap();
                //.expect("Falied Parse block user list response");
            )?

            //response
            //    .parse_response()
            //    .unwrap();

        }
    };
    (@user $scopes:expr)=>{
        user_token($scopes).await
        };
    (@app $scopes:expr) => {
        app_token($scopes).await
        };
}

macro_rules! oauth {
    // User
    (@user) => {
        oauth!(
            @user
            client_id: std::env::var("TEST_CLIENT_ID").expect("TEST_CLIENT_ID environment variable is not set")
        )
    };
    (@user client_id: $client_id:expr $(,)?) => {
        oauth!(
        @user
        client_id: $client_id,
        client_secret: std::env::var("TEST_CLIENT_SECRET").expect("TEST_CLIENT_SECRET environment variable is not set")
        )

    };
    (
    @user
    client_id: $client_id:expr,
    client_secret: $client_secret:expr $(,)?
    ) => {
        oauth!(
            @user
            client_id: $client_id,
            client_secret: $client_secret,
            user_id: std::env::var("TEST_USER_ID").expect("TEST_USER_ID environment variable is not set")
        )

    };
    (
    @user
    client_id: $client_id:expr,
    client_secret: $client_secret:expr,
    user_id: $user_id:expr $(,)?
    ) => {
        oauth!(
            @user
            client_id: $client_id,
            client_secret: $client_secret,
            user_id: $user_id,
            port: std::env::var("TEST_PORT").ok().map(|x| x.parse().unwrap())
        )
};
    (
    @user
    client_id: $client_id:expr,
    client_secret: $client_secret:expr,
    user_id: $user_id:expr,
    port: $port:expr $(,)?
    ) => {{
        let oauth = twitch_oauth_token::TwitchOauth::new(
            $client_id.to_string(),
            $client_secret.to_string(),
            None,
        )
        .expect("Failed to initialize TwitchOAuth with mock credentials")
        .with_url($port);

        oauth.user_token($user_id.to_string())
    }};
    // App
    (@app) => {
    oauth!(
        @app
        client_id: std::env::var("TEST_CLIENT_ID").expect("TEST_CLIENT_ID environment variable is not set")
    )
    };
    (@app client_id: $client_id:expr $(,)?) => {
        oauth!(
        @app
        client_id: $client_id,
        client_secret: std::env::var("TEST_CLIENT_SECRET").expect("TEST_CLIENT_SECRET environment variable is not set")
        )

    };
    (
    @app
    client_id: $client_id:expr,
    client_secret: $client_secret:expr $(,)?
    ) => {
        oauth!(
            @app
            client_id: $client_id,
            client_secret: $client_secret,
            user_id: std::env::var("TEST_USER_ID").expect("TEST_USER_ID environment variable is not set")
        )

    };
    (
    @app
    client_id: $client_id:expr,
    client_secret: $client_secret:expr,
    port: $port:expr $(,)?
    ) => {{
        let oauth = twitch_oauth_token::TwitchOauth::new(
            $client_id.to_string(),
            $client_secret.to_string(),
            None,
        )
        .expect("Failed to initialize TwitchOAuth with mock credentials")
        .with_url($port);

        oauth.app_token($user_id.to_string())
    }};

}

macro_rules! new_fn_mock_server_f {
    (
    $(#[$name_meta:meta])*
    name: $fn_name:ident,
    oauth: {
        @$type:ident,
        module: $scope_module:path,
        scopes: $scopes:ident $(,)?
    },
    api: {
        modules: [$($api_module:path),* $(,)?],
        endpoint: $endpoint:ident,
        args: |$($broadcaster_id:ident),*$(,)?|{$($arg:expr),*$(,)?} $(,)?
    }
    ) =>{
        new_fn_mock_server_f!(
            $(#[$name_meta])*
            name: $fn_name,
            oauth: {
                @$type,
                module: $scope_module,
                scopes: $scopes
            },
            api: {
                modules: [$($api_module),*],
                endpoint: $endpoint,
                args: |$($broadcaster_id),*|{$($arg),*},
                check: false,
                status: OK,
                rep: true
            }
        );
    };
    (
    $(#[$name_meta:meta])*
    name: $fn_name:ident,
    oauth: {
        @$type:ident,
        module: $scope_module:path,
        scopes: $scopes:ident $(,)?
    },
    api: {
        modules: [$($api_module:path),+ $(,)?],
        endpoint: $endpoint:ident,
        args: |$($broadcaster_id:ident),*|{$($arg:expr),+$(,)?},
        check: $body_check:expr $(,)?
    }
    ) =>{
        new_fn_mock_server_f!(
            $(#[$name_meta])*
            name: $fn_name,
            oauth: {
                @$type,
                module: $scope_module,
                scopes: $scopes
            },
            api: {
                modules: [$($api_module),+],
                endpoint: $endpoint,
                args: |$($broadcaster_id),*|{$($arg),+},
                check: $body_check,
                status: OK,
                rep: true
            }
        );
    };
    (
    $(#[$name_meta:meta])*
    name: $fn_name:ident,
    oauth: {
        @$type:ident,
        module: $scope_module:path,
        scopes: $scopes:ident $(,)?
    },
    api: {
        modules: [$($api_module:path),* $(,)?],
        endpoint: $endpoint:ident,
        args: |$($broadcaster_id:ident),* $(,)?|{$($arg:expr),*$(,)?},
        status: $status:ident $(,)?
    }
    ) =>{
        new_fn_mock_server_f!(
            $(#[$name_meta])*
            name: $fn_name,
            oauth: {
                @$type,
                module: $scope_module,
                scopes: $scopes
            },
            api: {
                modules: [$($api_module),*],
                endpoint: $endpoint,
                args: |$($broadcaster_id),*|{$($arg),*},
                check: false,
                status: $status,
                rep: true
            }
        );
    };
        (
    $(#[$name_meta:meta])*
    name: $fn_name:ident,
    oauth: {
        @$type:ident,
        module: $scope_module:path,
        scopes: $scopes:ident $(,)?
    },
    api: {
        modules: [$($api_module:path),* $(,)?],
        endpoint: $endpoint:ident,
        args: |$($broadcaster_id:ident),*|{$($arg:expr),*$(,)?},
        status: $status:ident,
        rep: $parse:expr$(,)?
    }
    ) =>{
        new_fn_mock_server_f!(
            $(#[$name_meta])*
            name: $fn_name,
            oauth: {
                @$type,
                module: $scope_module,
                scopes: $scopes
            },
            api: {
                modules: [$($api_module),*],
                endpoint: $endpoint,
                args: |$($broadcaster_id),*|{$($arg),*},
                check: false,
                status: $status,
                rep: $parse
            }
        );
    };

    (
    $(#[$name_meta:meta])*
    name: $fn_name:ident,
    oauth: {
        @$type:ident,
        module: $scope_module:path,
        scopes: $scopes:ident $(,)?
    },
    api: {
        modules: [$($api_module:path),* $(,)?],
        endpoint: $endpoint:ident,
        args: |$($broadcaster_id:ident),*$(,)?|{$($arg:expr),*$(,)?},
        check:$body_check:expr,
        status:$status:ident,
        rep: $parse:expr $(,)?
    }
    ) => {
        $(#[$name_meta])*
        #[cfg(feature = "test")]
        #[tokio::test]
        async fn $fn_name() {
            use twitch_oauth_token::types::{$scope_module, Token};
            use twitch_highway::{APIError, TwitchAPI, TestUrl};
            use asknothingx2_util::{
                oauth::ClientId,
            };
           $(use $api_module;)*

            dotenv::dotenv().ok();
            general_id!($($broadcaster_id),*);

            let mut oauth = oauth!(@$type);
            oauth.scopes_mut().$scopes();
            let oauth = oauth.request_access_token().await.unwrap();

            if !oauth.is_success() {
                let error: APIError = oauth.into_json().unwrap();
                eprintln!("{:#?}", error);
                return;
            }

            let token: Token = oauth.into_json().unwrap();

            let api = TwitchAPI::new(
                token.access_token,
                ClientId::new(std::env::var("TEST_CLIENT_ID").expect("USER_ID environment variable is not set")),
            );

            let request = api
                .$endpoint($($arg),*)
                .with_url(None, None)
                .request()
                .await
                .unwrap();

            if $body_check {
                assert_eq!("body_check", request.text());
            }

            assert_eq!(asknothingx2_util::api::StatusCode::$status, request.status(),"{}",request.text());
            if $parse {
                    request.parse_response().unwrap();
            }


        }
    };
}
