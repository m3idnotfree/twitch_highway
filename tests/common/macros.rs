/// Generates standardized integration tests for API endpoints.
///
/// # Patterns:
/// - `api_test!(endpoint [params])` → Test with JSON response validation
/// - `api_test!(endpoint, [params], send)` → Test with HTTP response validation only
/// - `api_test!(endpoint as test_endpoint, [params])` → Extra Test with custom mock setup
///
/// # Requirements:
/// - `HttpMock` must implement `async fn endpoint(&self)` for mock setup
/// - API trait must have corresponding `fn endpoint(&self, params) -> TwitchAPIRequest<T>`
///
/// # Example:
/// ```rust,ignore
/// api_test!(get_users [UserId::from("123456789")]);                   // JSON response
/// api_test!(create_reward [broadcaster_id, reward_request], send);    // No content (204)
/// api_test!(get_global_emotes []);                                    // No parameters
/// api_test!(get_analytics as extra_get_analytics, []);                // Custom test name
/// ```
#[allow(unused_macros)]
macro_rules! api_test {
    ($endpoint:ident $([$($param:expr),*$(,)?])? $(, $res:ident)?) => {
        api_test!(@impl $endpoint, $endpoint, [$($($param),*)?], $($res)?);
    };
    ($endpoint:ident as $test_endpoint:ident $([$($param:expr),*$(,)?])? $(, $res:ident)?) => {
        api_test!(@impl $endpoint, $test_endpoint, [$($($param),*)?], $($res)?);
    };

    ($endpoint:ident |$api:ident| {$($body:tt)*} $(, $res:ident)?) => {
        api_test!(@impl_build $endpoint, $endpoint, $api, {$($body)*}, $($res)?);
    };
    ($endpoint:ident as $test_endpoint:ident |$api:ident| {$($body:tt)*} $(, $res:ident)?) => {
        api_test!(@impl_build $endpoint, $test_endpoint, $api, {$($body)*}, $($res)?);
    };

    ($($endpoint:ident $(as $test_endpoint:ident)? $([$($param:expr),*$(,)?])?),+ $(,)?) => {
        $(
            api_test!($endpoint $(as $test_endpoint)? [$($($param),*)?]);
        )+
    };

    (@impl $endpoint:ident, $test_endpoint:ident, [$($param:expr),*$(,)?], ) => {
        api_test!(@impl_ $endpoint, $test_endpoint, [$($param),*], json);
    };
    (@impl $endpoint:ident, $test_endpoint:ident, [$($param:expr),*$(,)?], json) => {
        api_test!(@impl_ $endpoint, $test_endpoint, [$($param),*], json);
    };
    (@impl $endpoint:ident, $test_endpoint:ident, [$($param:expr),*$(,)?], send) => {
        api_test!(@impl_ $endpoint, $test_endpoint, [$($param),*], send);
    };

    (@impl_ $endpoint:ident, $test_endpoint:ident, [$($param:expr),*$(,)?], $res:ident) => {
        #[tokio::test]
        pub(crate) async fn $test_endpoint() {
            let suite = crate::common::HttpMock::new().await;
            suite.$test_endpoint().await;

            let result = suite
                .execute(|api| api.$endpoint($($param),*))
                .$res()
                .await;

            match result {
                Ok(_) => {},
                Err(e) => panic!(
                    "\n❌ Test '{}' failed\n   Endpoint: {}\n   Error: {:?}\n",
                    stringify!($test_endpoint),
                    stringify!($endpoint),
                    e
                ),
            }
        }
    };

    (@impl_build $endpoint:ident, $test_endpoint:ident, $api:ident, $body:expr, ) => {
        api_test!(@impl_build_ $endpoint, $test_endpoint, $api, $body, json);
    };
    (@impl_build $endpoint:ident, $test_endpoint:ident, $api:ident, $body:expr, json) => {
        api_test!(@impl_build_ $endpoint, $test_endpoint, $api, $body, json);
    };
    (@impl_build $endpoint:ident, $test_endpoint:ident, $api:ident, $body:expr, send) => {
        api_test!(@impl_build_ $endpoint, $test_endpoint, $api, $body, send);
    };
    (@impl_build_ $endpoint:ident, $test_endpoint:ident, $api:ident, $body:expr, $res:ident) => {
        #[tokio::test]
        pub(crate) async fn $test_endpoint() {
            let suite = crate::common::HttpMock::new().await;
            suite.$test_endpoint().await;

            let result = suite
                .execute(|$api| $body.build())
                .$res()
                .await;

            match result {
                Ok(_) => {},
                Err(e) => panic!(
                    "\n❌ Test '{}' failed\n   Endpoint: {}\n   Error: {:?}\n",
                    stringify!($test_endpoint),
                    stringify!($endpoint),
                    e
                ),
            }
        }
    };
}
