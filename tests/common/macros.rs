/// Generates standardized integration tests for API endpoints.
///
/// # Patterns:
/// - `api_test!(endpoint, [params])` → Test with JSON response validation
/// - `api_test!(send endpoint, [params])` → Test with HTTP response validation only
/// - `api_test!(extra endpoint, test_name [params])` → Extra Test with custom mock setup
///
/// # Requirements:
/// - `HttpMock` must implement `async fn endpoint(&self)` for mock setup
/// - API trait must have corresponding `fn endpoint(&self, params) -> TwitchAPIRequest<T>`
///
/// # Example:
/// ```rust,ignore
/// api_test!(get_users, [UserId::from("123456789")]);                    // JSON response
/// api_test!(send create_reward, [broadcaster_id, reward_request]);     // No content (204)
/// api_test!(get_global_emotes, []);                                    // No parameters
/// api_test!(extra get_analytics, extra_get_analytics, []);             // Custom test name
/// ```
macro_rules! api_test {
    ($endpoint:ident, $([$($param:expr),* $(,)?])?) => {
        #[tokio::test]
        pub(crate) async fn $endpoint() {
            let suite = crate::common::HttpMock::new().await;

            suite.$endpoint().await;

            let _ = suite
                .execute(|api| {
                    api.$endpoint($($($param),*)?)
                })
                .json()
                .await
                .unwrap();
        }
    };
    (build $endpoint:ident |$api:ident| {$($body:tt)*}) => {
        #[tokio::test]
        pub(crate) async fn $endpoint() {
            let suite = crate::common::HttpMock::new().await;

            suite.$endpoint().await;

            let _ = suite
                .execute(|$api| {
                    $($body)*.build()
                })
                .json()
                .await
                .unwrap();
        }
    };
    (build_send $endpoint:ident |$api:ident| {$($body:tt)*}) => {
        #[tokio::test]
        pub(crate) async fn $endpoint() {
            let suite = crate::common::HttpMock::new().await;

            suite.$endpoint().await;

            let _ = suite
                .execute(|$api| {
                    $($body)*.build()
                })
                .send()
                .await
                .unwrap();
        }
    };
    (build_extra $endpoint:ident, $test_endpoint:ident |$api:ident| {$($body:tt)*}) => {
        #[tokio::test]
        pub(crate) async fn $test_endpoint() {
            let suite = crate::common::HttpMock::new().await;

            suite.$test_endpoint().await;

            let _ = suite
                .execute(|$api| {
                    $($body)*.build()
                })
                .json()
                .await
                .unwrap();
        }
    };

    (send $endpoint:ident, $([$($param:expr),* $(,)?])?) => {
        #[tokio::test]
        pub(crate) async fn $endpoint() {
            let suite = crate::common::HttpMock::new().await;

            suite.$endpoint().await;

            let _ = suite
                .execute(|api| {
                    api.$endpoint($($($param),*)?)
                })
                .send()
                .await
                .unwrap();
        }
    };

    (extra $endpoint:ident, $test_endpoint:ident, $([$($param:expr),* $(,)?])?) => {
        #[tokio::test]
        async fn $test_endpoint() {
            let suite = crate::common::HttpMock::new().await;

            suite.$test_endpoint().await;

            let _ = suite
                .execute(|api| {
                    api.$endpoint($($($param),*)?)
                })
                .json()
                .await
                .unwrap();
        }
    };
}
