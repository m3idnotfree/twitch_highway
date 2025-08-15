use asknothingx2_util::{
    api::preset,
    oauth::{AccessToken, ClientId},
};
use chrono::{DateTime, Datelike, FixedOffset, Timelike};
use serde::de::DeserializeOwned;
use serde_json::json;
use url::Url;
use wiremock::{
    matchers::{body_json, header, method, path, query_param},
    Mock, MockServer, ResponseTemplate,
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

    pub async fn mock_ads_success(&self) {
        let expected_response = json!({
            "data": [{
                "length": 60,
                "message": "",
                "retry_after": 480
            }]
        });

        Mock::given(method("POST"))
            .and(path("/helix/channels/commercial"))
            .and(header(
                "authorization",
                "Bearer 2gbdx6oar67tqtcmt49t3wpcgycthx",
            ))
            .and(header("client-id", "wbmytr93xzw8zbg0p1izqyzzc5mbiz"))
            .and(header("content-type", "application/json"))
            .and(body_json(json!({
                "broadcaster_id": "141981764",
                "length": 60
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(expected_response))
            .mount(&self.server)
            .await;

        let expected_response = json!({
            "data": [{
                "next_ad_at" : "2023-08-01T23:08:18+00:00",
                "last_ad_at" : "2023-08-01T23:08:18+00:00",
                "duration" : "60",
                "preroll_free_time" : "90",
                "snooze_count" : "1",
                "snooze_refresh_at" : "2023-08-01T23:08:18+00:00",
            }],
        });

        Mock::given(method("GET"))
            .and(path("/helix/channels/ads"))
            .and(query_param("broadcaster_id", "123456789"))
            .and(header(
                "authorization",
                "Bearer 2gbdx6oar67tqtcmt49t3wpcgycthx",
            ))
            .and(header("client-id", "wbmytr93xzw8zbg0p1izqyzzc5mbiz"))
            .respond_with(ResponseTemplate::new(200).set_body_json(expected_response))
            .mount(&self.server)
            .await;

        let expected_response = json!({
            "data": [{
                "snooze_count": "1",
                "snooze_refresh_at" : "2023-08-01T23:08:18+00:00",
                "next_ad_at" : "2023-08-01T23:08:18+00:00",
            }]
        });

        Mock::given(method("POST"))
            .and(path("/helix/channels/ads/schedule/snooze"))
            .and(query_param("broadcaster_id", "123456789"))
            .and(header(
                "authorization",
                "Bearer 2gbdx6oar67tqtcmt49t3wpcgycthx",
            ))
            .and(header("client-id", "wbmytr93xzw8zbg0p1izqyzzc5mbiz"))
            .respond_with(ResponseTemplate::new(200).set_body_json(expected_response))
            .mount(&self.server)
            .await;
    }

    pub async fn mock_ads_failure(&self) {
        let error_response = json!({
            "error": "Bad Request",
            "status": 400,
            "message": ""
        });

        Mock::given(method("POST"))
            .and(path("/helix/channels/commercial"))
            .respond_with(ResponseTemplate::new(400).set_body_json(error_response))
            .mount(&self.server)
            .await;
    }

    pub async fn mock_analytics_success(&self) {
        let expected_response = json!({
            "data": [
                {
                    "extension_id": "ext123456",
                    "URL": "https://twitch-analytics.s3-us-west-2.amazonaws.com/extension123.csv",
                    "type": "overview_v2",
                    "date_range": {
                        "started_at": "2023-12-01T00:00:00Z",
                        "ended_at": "2023-12-01T23:59:59Z"
                    }
                }
            ],
            "pagination": {
                "cursor": "eyJiI..."
            }
        });

        Mock::given(method("GET"))
            .and(path("/helix/analytics/extensions"))
            .and(query_param("extension_id", "ext123456"))
            .and(query_param("type", "overview_v2"))
            .and(query_param("first", "20"))
            .and(header(
                "authorization",
                "Bearer 2gbdx6oar67tqtcmt49t3wpcgycthx",
            ))
            .and(header("client-id", "wbmytr93xzw8zbg0p1izqyzzc5mbiz"))
            .respond_with(ResponseTemplate::new(200).set_body_json(expected_response))
            .mount(&self.server)
            .await;

        let expected_response = json!({
            "data": [
                {
                    "game_id": "12345",
                    "URL": "https://twitch-analytics.s3-us-west-2.amazonaws.com/game12345.csv",
                    "type": "overview_v2",
                    "date_range": {
                        "started_at": "2023-12-01T00:00:00Z",
                        "ended_at": "2023-12-01T23:59:59Z"
                    }
                }
            ]
        });

        Mock::given(method("GET"))
            .and(path("/helix/analytics/games"))
            .and(query_param("game_id", "12345"))
            .and(query_param("started_at", "2023-12-01T00:00:00Z"))
            .and(query_param("ended_at", "2023-12-01T23:59:59Z"))
            .and(header(
                "authorization",
                "Bearer 2gbdx6oar67tqtcmt49t3wpcgycthx",
            ))
            .and(header("client-id", "wbmytr93xzw8zbg0p1izqyzzc5mbiz"))
            .respond_with(ResponseTemplate::new(200).set_body_json(expected_response))
            .mount(&self.server)
            .await;
    }

    pub async fn mock_analytics_extra(&self) {
        let expected_response = json!({
            "data": [
                {
                    "extension_id": "ext123456",
                    "URL": "https://twitch-analytics.s3-us-west-2.amazonaws.com/extension123.csv",
                    "type": "overview_v2",
                    "date_range": {
                        "started_at": "2023-12-01T00:00:00Z",
                        "ended_at": "2023-12-01T23:59:59Z"
                    }
                }
            ]
        });

        Mock::given(method("GET"))
            .and(path("/helix/analytics/extensions"))
            .and(header(
                "authorization",
                "Bearer 2gbdx6oar67tqtcmt49t3wpcgycthx",
            ))
            .and(header("client-id", "wbmytr93xzw8zbg0p1izqyzzc5mbiz"))
            .respond_with(ResponseTemplate::new(200).set_body_json(expected_response))
            .mount(&self.server)
            .await;
    }

    pub async fn mock_analytics_failure(&self) {
        let error_response = json!({
            "error": "Unauthorized",
            "status": 401,
            "message": "Invalid OAuth token"
        });

        Mock::given(method("GET"))
            .and(path("/helix/analytics/extensions"))
            .respond_with(ResponseTemplate::new(401).set_body_json(error_response))
            .mount(&self.server)
            .await;
    }

    pub async fn mock_bits_success(&self) {
        let expected_response = json!({
            "data": [
                {
                    "user_id": "123456789",
                    "user_login": "testuser",
                    "user_name": "TestUser",
                    "rank": 1,
                    "score": 12543
                }
            ],
            "date_range": {
                "started_at": "2023-12-01T00:00:00Z",
                "ended_at": "2023-12-07T23:59:59Z"
            },
            "total": 1
        });

        Mock::given(method("GET"))
            .and(path("/helix/bits/leaderboard"))
            .and(query_param("count", "10"))
            .and(query_param("period", "week"))
            .and(header(
                "authorization",
                "Bearer 2gbdx6oar67tqtcmt49t3wpcgycthx",
            ))
            .and(header("client-id", "wbmytr93xzw8zbg0p1izqyzzc5mbiz"))
            .respond_with(ResponseTemplate::new(200).set_body_json(expected_response))
            .mount(&self.server)
            .await;

        let expected_response = json!({
            "data": [
                {
                    "prefix": "Cheer",
                    "tiers": [
                        {
                            "min_bits": 1,
                            "id": "1",
                            "color": "#979797",
                            "images": {
                                "dark": {
                                    "animated": {
                                        "1": "https://example.com/1.gif",
                                        "1.5": "https://example.com/1.5.gif",
                                        "2": "https://example.com/2.gif",
                                        "3": "https://example.com/3.gif",
                                        "4": "https://example.com/4.gif"
                                    },
                                    "static": {
                                        "1": "https://example.com/1.png",
                                        "1.5": "https://example.com/1.5.png",
                                        "2": "https://example.com/2.png",
                                        "3": "https://example.com/3.png",
                                        "4": "https://example.com/4.png"
                                    }
                                },
                                "light": {
                                    "animated": {
                                        "1": "https://example.com/light/1.gif",
                                        "1.5": "https://example.com/light/1.5.gif",
                                        "2": "https://example.com/light/2.gif",
                                        "3": "https://example.com/light/3.gif",
                                        "4": "https://example.com/light/4.gif"
                                    },
                                    "static": {
                                        "1": "https://example.com/light/1.png",
                                        "1.5": "https://example.com/light/1.5.png",
                                        "2": "https://example.com/light/2.png",
                                        "3": "https://example.com/light/3.png",
                                        "4": "https://example.com/light/4.png"
                                    }
                                }
                            },
                            "can_cheer": true,
                            "show_in_bits_card": true
                        }
                    ],
                    "type": "global_first_party",
                    "order": 1,
                    "last_updated": "2023-12-01T15:30:00Z",
                    "is_charitable": false
                }
            ]
        });

        Mock::given(method("GET"))
            .and(path("/helix/bits/cheermotes"))
            .and(query_param("broadcaster_id", "broadcaster123"))
            .and(header(
                "authorization",
                "Bearer 2gbdx6oar67tqtcmt49t3wpcgycthx",
            ))
            .and(header("client-id", "wbmytr93xzw8zbg0p1izqyzzc5mbiz"))
            .respond_with(ResponseTemplate::new(200).set_body_json(expected_response))
            .mount(&self.server)
            .await;

        let expected_response = json!({
            "data": [
                {
                    "id": "trans123456",
                    "timestamp": "2023-12-01T15:30:00Z",
                    "broadcaster_id": "broadcaster123",
                    "broadcaster_login": "testbroadcaster",
                    "broadcaster_name": "TestBroadcaster",
                    "user_id": "user123456",
                    "user_login": "testuser",
                    "user_name": "TestUser",
                    "product_type": "BITS_IN_EXTENSION",
                    "product_data": {
                        "domain": "twitch-ext",
                        "sku": "test-sku",
                        "cost": {
                            "amount": 100,
                            "type": "bits"
                        },
                        "inDevelopment": false,
                        "displayName": "Test Product",
                        "expiration": "2024-12-01T15:30:00Z",
                        "broadcast": true
                    }
                }
            ],
            "pagination": {
                "cursor": "eyJiI..."
            }
        });

        Mock::given(method("GET"))
            .and(path("/helix/extensions/transactions"))
            .and(query_param("extension_id", "ext123456"))
            .and(query_param("id", "trans123456"))
            .and(query_param("first", "20"))
            .and(header(
                "authorization",
                "Bearer 2gbdx6oar67tqtcmt49t3wpcgycthx",
            ))
            .and(header("client-id", "wbmytr93xzw8zbg0p1izqyzzc5mbiz"))
            .respond_with(ResponseTemplate::new(200).set_body_json(expected_response))
            .mount(&self.server)
            .await;
    }

    pub async fn mock_bits_extra(&self) {
        let expected_response = json!({
            "data": [],
            "date_range": {
                "started_at": "2023-12-01T00:00:00Z",
                "ended_at": "2023-12-07T23:59:59Z"
            },
            "total": 0
        });

        Mock::given(method("GET"))
            .and(path("/helix/bits/leaderboard"))
            .and(header(
                "authorization",
                "Bearer 2gbdx6oar67tqtcmt49t3wpcgycthx",
            ))
            .and(header("client-id", "wbmytr93xzw8zbg0p1izqyzzc5mbiz"))
            .respond_with(ResponseTemplate::new(200).set_body_json(expected_response))
            .mount(&self.server)
            .await;
    }

    pub async fn mock_bits_failure(&self) {
        let error_response = json!({
            "error": "Forbidden",
            "status": 403,
            "message": "Missing required scope"
        });

        Mock::given(method("GET"))
            .and(path("/helix/bits/leaderboard"))
            .respond_with(ResponseTemplate::new(403).set_body_json(error_response))
            .mount(&self.server)
            .await;
    }
}

#[track_caller]
pub fn assert_datetime(
    actual: DateTime<FixedOffset>,
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    min: u32,
) {
    assert_eq!(actual.year(), year, "Year mismatch");
    assert_eq!(actual.month(), month, "Month mismatch");
    assert_eq!(actual.day(), day, "Day mismatch");
    assert_eq!(actual.hour(), hour, "Hour mismatch");
    assert_eq!(actual.minute(), min, "Minute mismatch");
}
