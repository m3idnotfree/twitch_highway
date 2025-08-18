use asknothingx2_util::{
    api::preset,
    oauth::{AccessToken, ClientId},
};
use serde::de::DeserializeOwned;
use serde_json::json;
use url::Url;
use wiremock::{
    matchers::{body_json, header, method, path, query_param},
    Mock, MockBuilder, MockServer, ResponseTemplate,
};

use crate::{request::TwitchAPIRequest, TwitchAPI};

#[derive(Debug)]
pub struct TwitchApiTest {
    api: TwitchAPI,
    pub server: MockServer,
}

impl TwitchApiTest {
    pub async fn new() -> Self {
        let mock_server = MockServer::start().await;
        let mut url_parsed = Url::parse(&mock_server.uri()).unwrap();
        url_parsed.set_path("/helix");

        let api = TwitchAPI::new(
            AccessToken::new("2gbdx6oar67tqtcmt49t3wpcgycthx".to_string()),
            ClientId::new("wbmytr93xzw8zbg0p1izqyzzc5mbiz".to_string()),
        )
        .set_url(url_parsed);

        TwitchApiTest {
            api,
            server: mock_server,
        }
    }

    fn basic_mock(&self, http_method: &str, endpoint: &str) -> MockBuilder {
        Mock::given(method(http_method)).and(path(format!("/helix{}", endpoint)))
    }

    fn api_mock(&self, http_method: &str, endpoint: &str) -> wiremock::MockBuilder {
        self.basic_mock(http_method, endpoint)
            .and(header(
                "authorization",
                "Bearer 2gbdx6oar67tqtcmt49t3wpcgycthx",
            ))
            .and(header("client-id", "wbmytr93xzw8zbg0p1izqyzzc5mbiz"))
    }

    pub fn execute<F, T>(&self, path: &str, f: F) -> TwitchAPIRequest<T>
    where
        F: FnOnce(&TwitchAPI) -> TwitchAPIRequest<T>,
        T: DeserializeOwned,
    {
        let resp = f(&self.api);

        resp.set_client(
            preset::testing("twitch-highway-test/1.0")
                .build_client()
                .unwrap(),
        )
        .set_url(
            self.server
                .uri()
                .parse::<Url>()
                .unwrap()
                .join(&format!("/helix{path}"))
                .unwrap(),
        )
    }
}

#[cfg(feature = "ads")]
impl TwitchApiTest {
    pub async fn start_commercial(&self) {
        self.api_mock("POST", "/channels/commercial")
            .and(header("content-type", "application/json"))
            .and(body_json(json!({
                "broadcaster_id": "141981764",
                "length": 60
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                  "data": [
                    {
                        "length" : 60,
                        "message" : "",
                        "retry_after" : 480
                    }
                ]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn get_ad_schedule(&self) {
        self.api_mock("GET", "/channels/ads")
            .and(query_param("broadcaster_id", "141981764"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "next_ad_at" : "2023-08-01T23:08:18+00:00",
                        "last_ad_at" : "2023-08-01T23:08:18+00:00",
                        "duration" : "60",
                        "preroll_free_time" : "90",
                        "snooze_count" : "1",
                        "snooze_refresh_at" : "2023-08-01T23:08:18+00:00",
                    },
                ],
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn snooze_next_ad(&self) {
        self.api_mock("POST", "/channels/ads/schedule/snooze")
            .and(query_param("broadcaster_id", "141981764"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "snooze_count" : "1",
                        "snooze_refresh_at" : "2023-08-01T23:08:18+00:00",
                        "next_ad_at" : "2023-08-01T23:08:18+00:00",
                    },
                ],
            })))
            .mount(&self.server)
            .await;
    }
}

pub mod params {
    #![allow(non_snake_case)]
    #[cfg(feature = "ads")]
    pub mod AdsAPI {
        use crate::types::BroadcasterId;

        pub struct StartCommercialParams {
            pub broadcaster_id: BroadcasterId,
            pub length: u64,
        }

        pub fn start_commercial() -> StartCommercialParams {
            StartCommercialParams {
                broadcaster_id: BroadcasterId::new("141981764"),
                length: 60,
            }
        }

        pub struct BroadcasterIdParam {
            pub broadcaster_id: BroadcasterId,
        }

        pub fn get_ad_schedule() -> BroadcasterIdParam {
            BroadcasterIdParam {
                broadcaster_id: BroadcasterId::new("141981764"),
            }
        }

        pub fn snooze_next_ad() -> BroadcasterIdParam {
            BroadcasterIdParam {
                broadcaster_id: BroadcasterId::new("141981764"),
            }
        }
    }
}
