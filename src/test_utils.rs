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

    pub async fn mock_ccls_success(&self) {
        let expected_response = json!({
            "data": [
                {
                    "id": "DrugsIntoxication",
                    "description": "Excessive tobacco glorification, any marijuana consumption/use, legal drug and alcohol induced intoxication, discussions of illegal drugs.",
                    "name": "Drugs, Intoxication, or Excessive Tobacco Use"
                },
                {
                    "id": "SexualThemes",
                    "description": "Content that focuses on sexualized physical attributes and activities, sexual topics, or experiences.",
                    "name": "Sexual Themes"
                }
            ]
        });

        Mock::given(method("GET"))
            .and(path("/helix/content_classification_labels"))
            .and(query_param("locale", "en-US"))
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
                    "id": "ViolentGraphic",
                    "description": "Simulated or real violence with sustained visceral content including mutilation, gore, or death.",
                    "name": "Violent and Graphic Depictions"
                },
                {
                    "id": "Gambling",
                    "description": "Games/activities that utilize or simulate gambling/betting with real or virtual currencies or items.",
                    "name": "Gambling"
                }
            ]
        });

        Mock::given(method("GET"))
            .and(path("/helix/content_classification_labels"))
            .and(header(
                "authorization",
                "Bearer 2gbdx6oar67tqtcmt49t3wpcgycthx",
            ))
            .and(header("client-id", "wbmytr93xzw8zbg0p1izqyzzc5mbiz"))
            .respond_with(ResponseTemplate::new(200).set_body_json(expected_response))
            .mount(&self.server)
            .await;
    }

    pub async fn mock_ccls_success_without_locale(&self) {
        let expected_response = json!({
            "data": [
                {
                    "id": "ViolentGraphic",
                    "description": "Simulated or real violence with sustained visceral content including mutilation, gore, or death.",
                    "name": "Violent and Graphic Depictions"
                },
                {
                    "id": "Gambling",
                    "description": "Games/activities that utilize or simulate gambling/betting with real or virtual currencies or items.",
                    "name": "Gambling"
                }
            ]
        });

        Mock::given(method("GET"))
            .and(path("/helix/content_classification_labels"))
            .and(header(
                "authorization",
                "Bearer 2gbdx6oar67tqtcmt49t3wpcgycthx",
            ))
            .and(header("client-id", "wbmytr93xzw8zbg0p1izqyzzc5mbiz"))
            .respond_with(ResponseTemplate::new(200).set_body_json(expected_response))
            .mount(&self.server)
            .await;
    }

    pub async fn mock_ccls_failure(&self) {
        let error_response = json!({
            "error": "Bad Request",
            "status": 400,
            "message": "Invalid locale parameter"
        });

        Mock::given(method("GET"))
            .and(path("/helix/content_classification_labels"))
            .respond_with(ResponseTemplate::new(400).set_body_json(error_response))
            .mount(&self.server)
            .await;
    }

    pub async fn mock_channel_points_success(&self) {
        let expected_response = json!({
            "data": [
                {
                    "broadcaster_id": "123456789",
                    "broadcaster_login": "testbroadcaster",
                    "broadcaster_name": "TestBroadcaster",
                    "id": "reward123",
                    "title": "New Test Reward",
                    "prompt": "This is a new test reward",
                    "cost": 1500,
                    "default_image": {
                        "url_1x": "https://static-cdn.jtvnw.net/default-reward-images/1x.png",
                        "url_2x": "https://static-cdn.jtvnw.net/default-reward-images/2x.png",
                        "url_4x": "https://static-cdn.jtvnw.net/default-reward-images/4x.png"
                    },
                    "background_color": "#9147FF",
                    "is_enabled": true,
                    "is_user_input_required": true,
                    "max_per_stream_setting": {
                        "is_enabled": false
                    },
                    "max_per_user_per_stream_setting": {
                        "is_enabled": false
                    },
                    "global_cooldown_setting": {
                        "is_enabled": false
                    },
                    "is_paused": false,
                    "is_in_stock": true,
                    "should_redemptions_skip_request_queue": false
                }
            ]
        });

        let expected_request_body = json!({
            "title": "New Test Reward",
            "cost": 1500,
            "prompt": "This is a new test reward",
            "is_user_input_required": true
        });

        Mock::given(method("POST"))
            .and(path("/helix/channel_points/custom_rewards"))
            .and(query_param("broadcaster_id", "123456789"))
            .and(header(
                "authorization",
                "Bearer 2gbdx6oar67tqtcmt49t3wpcgycthx",
            ))
            .and(header("client-id", "wbmytr93xzw8zbg0p1izqyzzc5mbiz"))
            .and(header("content-type", "application/json"))
            .and(body_json(expected_request_body))
            .respond_with(ResponseTemplate::new(200).set_body_json(expected_response))
            .mount(&self.server)
            .await;

        let expected_response = json!({
            "data": [
                {
                    "broadcaster_id": "123456789",
                    "broadcaster_login": "testbroadcaster",
                    "broadcaster_name": "TestBroadcaster",
                    "id": "reward123",
                    "title": "Existing Reward",
                    "prompt": "An existing reward",
                    "cost": 1000,
                    "default_image": {
                        "url_1x": "https://static-cdn.jtvnw.net/default-reward-images/1x.png",
                        "url_2x": "https://static-cdn.jtvnw.net/default-reward-images/2x.png",
                        "url_4x": "https://static-cdn.jtvnw.net/default-reward-images/4x.png"
                    },
                    "background_color": "#9147FF",
                    "is_enabled": true,
                    "is_user_input_required": false,
                    "max_per_stream_setting": {
                        "is_enabled": false
                    },
                    "max_per_user_per_stream_setting": {
                        "is_enabled": false
                    },
                    "global_cooldown_setting": {
                        "is_enabled": false
                    },
                    "is_paused": false,
                    "is_in_stock": true,
                    "should_redemptions_skip_request_queue": false
                }
            ]
        });

        Mock::given(method("GET"))
            .and(path("/helix/channel_points/custom_rewards"))
            .and(query_param("broadcaster_id", "123456789"))
            .and(query_param("id", "reward123"))
            .and(query_param("only_manageable_rewards", "true"))
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
                    "broadcaster_id": "123456789",
                    "broadcaster_login": "testbroadcaster",
                    "broadcaster_name": "TestBroadcaster",
                    "id": "redemption123",
                    "user_id": "user123",
                    "user_name": "TestUser",
                    "user_input": "Please give me the reward!",
                    "status": "UNFULFILLED",
                    "redeemed_at": "2023-12-01T15:30:00Z",
                    "reward": {
                        "id": "reward123",
                        "title": "Test Reward",
                        "prompt": "Test reward prompt",
                        "cost": 1000
                    }
                }
            ]
        });

        Mock::given(method("GET"))
            .and(path("/helix/channel_points/custom_rewards/redemptions"))
            .and(query_param("broadcaster_id", "123456789"))
            .and(query_param("reward_id", "reward123"))
            .and(query_param("status", "UNFULFILLED"))
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
                    "broadcaster_id": "123456789",
                    "broadcaster_login": "testbroadcaster",
                    "broadcaster_name": "TestBroadcaster",
                    "id": "redemption123",
                    "user_id": "user123",
                    "user_name": "TestUser",
                    "user_input": "Please give me the reward!",
                    "status": "FULFILLED",
                    "redeemed_at": "2023-12-01T15:30:00Z",
                    "reward": {
                        "id": "reward123",
                        "title": "Test Reward",
                        "prompt": "Test reward prompt",
                        "cost": 1000
                    }
                }
            ]
        });

        let expected_request_body = json!({
            "status": "FULFILLED"
        });

        Mock::given(method("PATCH"))
            .and(path("/helix/channel_points/custom_rewards/redemptions"))
            .and(query_param("broadcaster_id", "123456789"))
            .and(query_param("reward_id", "reward123"))
            .and(query_param("id", "redemption123"))
            .and(header(
                "authorization",
                "Bearer 2gbdx6oar67tqtcmt49t3wpcgycthx",
            ))
            .and(header("client-id", "wbmytr93xzw8zbg0p1izqyzzc5mbiz"))
            .and(header("content-type", "application/json"))
            .and(body_json(expected_request_body))
            .respond_with(ResponseTemplate::new(200).set_body_json(expected_response))
            .mount(&self.server)
            .await;

        Mock::given(method("DELETE"))
            .and(path("/helix/channel_points/custom_rewards"))
            .and(query_param("broadcaster_id", "123456789"))
            .and(query_param("id", "reward123"))
            .and(header(
                "authorization",
                "Bearer 2gbdx6oar67tqtcmt49t3wpcgycthx",
            ))
            .and(header("client-id", "wbmytr93xzw8zbg0p1izqyzzc5mbiz"))
            .respond_with(ResponseTemplate::new(204)) // No content
            .mount(&self.server)
            .await;

        let expected_response = json!({
            "data": [
                {
                    "broadcaster_id": "123456789",
                    "broadcaster_login": "testbroadcaster",
                    "broadcaster_name": "TestBroadcaster",
                    "id": "reward123",
                    "title": "Updated Reward Title",
                    "prompt": "Updated reward prompt",
                    "cost": 1500,
                    "default_image": {
                        "url_1x": "https://static-cdn.jtvnw.net/default-reward-images/1x.png",
                        "url_2x": "https://static-cdn.jtvnw.net/default-reward-images/2x.png",
                        "url_4x": "https://static-cdn.jtvnw.net/default-reward-images/4x.png"
                    },
                    "background_color": "#FF6B6B",
                    "is_enabled": false,
                    "is_user_input_required": true,
                    "max_per_stream_setting": {
                        "is_enabled": false
                    },
                    "max_per_user_per_stream_setting": {
                        "is_enabled": false
                    },
                    "global_cooldown_setting": {
                        "is_enabled": false
                    },
                    "is_paused": true,
                    "is_in_stock": true,
                    "should_redemptions_skip_request_queue": false
                }
            ]
        });

        let expected_request_body = json!({
            "title": "Updated Reward Title",
            "prompt": "Updated reward prompt",
            "cost": 1500,
            "is_enabled": false,
            "is_paused": true
        });

        Mock::given(method("PATCH"))
            .and(path("/helix/channel_points/custom_rewards"))
            .and(query_param("broadcaster_id", "123456789"))
            .and(query_param("id", "reward123"))
            .and(header(
                "authorization",
                "Bearer 2gbdx6oar67tqtcmt49t3wpcgycthx",
            ))
            .and(header("client-id", "wbmytr93xzw8zbg0p1izqyzzc5mbiz"))
            .and(header("content-type", "application/json"))
            .and(body_json(expected_request_body))
            .respond_with(ResponseTemplate::new(200).set_body_json(expected_response))
            .mount(&self.server)
            .await;
    }

    pub async fn mock_channel_points_failure(&self) {
        let error_response = json!({
            "error": "Bad Request",
            "status": 400,
            "message": "The reward cost must be between 1 and 2000000"
        });

        Mock::given(method("POST"))
            .and(path("/helix/channel_points/custom_rewards"))
            .respond_with(ResponseTemplate::new(400).set_body_json(error_response))
            .mount(&self.server)
            .await;
    }

    pub async fn mock_channels_success(&self) {
        let expected_response = json!({
            "data": [
                {
                    "broadcaster_id": "123456789",
                    "broadcaster_login": "teststreamer",
                    "broadcaster_name": "TestStreamer",
                    "broadcaster_language": "en",
                    "game_id": "509658",
                    "game_name": "Just Chatting",
                    "title": "Testing My Stream",
                    "delay": 0,
                    "tags": ["English", "Gaming"],
                    "content_classification_labels": [],
                    "is_branded_content": false
                }
            ]
        });

        Mock::given(method("GET"))
            .and(path("/helix/channels"))
            .and(query_param("broadcaster_id", "123456789"))
            .and(header(
                "authorization",
                "Bearer 2gbdx6oar67tqtcmt49t3wpcgycthx",
            ))
            .and(header("client-id", "wbmytr93xzw8zbg0p1izqyzzc5mbiz"))
            .respond_with(ResponseTemplate::new(200).set_body_json(expected_response))
            .mount(&self.server)
            .await;

        let expected_request_body = json!({
            "title": "Updated Stream Title",
            "game_id": "509658",
            "broadcaster_language": "en",
            "tags": ["Gaming", "English", "Fun"],
            "is_branded_content": true
        });

        Mock::given(method("PATCH"))
            .and(path("/helix/channels"))
            .and(query_param("broadcaster_id", "123456789"))
            .and(header(
                "authorization",
                "Bearer 2gbdx6oar67tqtcmt49t3wpcgycthx",
            ))
            .and(header("client-id", "wbmytr93xzw8zbg0p1izqyzzc5mbiz"))
            .and(header("content-type", "application/json"))
            .and(body_json(expected_request_body))
            .respond_with(ResponseTemplate::new(204)) // No content
            .mount(&self.server)
            .await;

        let expected_response = json!({
            "data": [
                {
                    "user_id": "editor123",
                    "user_name": "ChannelEditor",
                    "created_at": "2023-12-01T15:30:00Z"
                }
            ]
        });

        Mock::given(method("GET"))
            .and(path("/helix/channels/editors"))
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
            "data": [
                {
                    "broadcaster_id": "123456789",
                    "broadcaster_login": "followedstreamer",
                    "broadcaster_name": "FollowedStreamer",
                    "followed_at": "2023-12-01T15:30:00Z"
                }
            ],
            "total": 1,
            "pagination": {
                "cursor": "eyJiI..."
            }
        });

        Mock::given(method("GET"))
            .and(path("/helix/channels/followed"))
            .and(query_param("user_id", "user123"))
            .and(query_param("broadcaster_id", "123456789"))
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
                    "user_id": "follower123",
                    "user_login": "testfollower",
                    "user_name": "TestFollower",
                    "followed_at": "2023-12-01T15:30:00Z"
                }
            ],
            "total": 1,
            "pagination": {
                "cursor": "eyJiI..."
            }
        });

        Mock::given(method("GET"))
            .and(path("/helix/channels/followers"))
            .and(query_param("broadcaster_id", "123456789"))
            .and(query_param("user_id", "follower123"))
            .and(query_param("first", "100"))
            .and(header(
                "authorization",
                "Bearer 2gbdx6oar67tqtcmt49t3wpcgycthx",
            ))
            .and(header("client-id", "wbmytr93xzw8zbg0p1izqyzzc5mbiz"))
            .respond_with(ResponseTemplate::new(200).set_body_json(expected_response))
            .mount(&self.server)
            .await;
    }

    pub async fn mock_channels_failure(&self) {
        let error_response = json!({
            "error": "Unauthorized",
            "status": 401,
            "message": "Invalid OAuth token"
        });

        Mock::given(method("GET"))
            .and(path("/helix/channels"))
            .respond_with(ResponseTemplate::new(401).set_body_json(error_response))
            .mount(&self.server)
            .await;
    }

    pub async fn mock_channels_extra(&self) {
        let expected_response = json!({
            "data": [
                {
                    "broadcaster_id": "123456789",
                    "broadcaster_login": "streamer1",
                    "broadcaster_name": "Streamer1",
                    "broadcaster_language": "en",
                    "game_id": "509658",
                    "game_name": "Just Chatting",
                    "title": "Stream 1",
                    "delay": 0,
                    "tags": ["English"],
                    "content_classification_labels": [],
                    "is_branded_content": false
                },
                {
                    "broadcaster_id": "987654321",
                    "broadcaster_login": "streamer2",
                    "broadcaster_name": "Streamer2",
                    "broadcaster_language": "es",
                    "game_id": "32982",
                    "game_name": "Grand Theft Auto V",
                    "title": "Stream 2",
                    "delay": 30,
                    "tags": ["Spanish"],
                    "content_classification_labels": ["ViolentGraphic"],
                    "is_branded_content": true
                }
            ]
        });

        Mock::given(method("GET"))
            .and(path("/helix/channels"))
            .and(query_param("broadcaster_id", "123456789"))
            .and(query_param("broadcaster_id", "987654321"))
            .and(header(
                "authorization",
                "Bearer 2gbdx6oar67tqtcmt49t3wpcgycthx",
            ))
            .and(header("client-id", "wbmytr93xzw8zbg0p1izqyzzc5mbiz"))
            .respond_with(ResponseTemplate::new(200).set_body_json(expected_response))
            .mount(&self.server)
            .await;
    }

    pub async fn mock_charity_success(&self) {
        let expected_response = json!({
            "data": [
                {
                    "id": "campaign123",
                    "broadcaster_id": "123456789",
                    "broadcaster_login": "charitablestreamer",
                    "broadcaster_name": "CharitableStreamer",
                    "charity_name": "Doctors Without Borders",
                    "charity_description": "International medical humanitarian organization providing aid in conflict zones and disaster areas.",
                    "charity_logo": "https://static-cdn.jtvnw.net/jtv_user_pictures/msf-logo.png",
                    "charity_website": "https://www.doctorswithoutborders.org/",
                    "current_amount": {
                        "value": 125000,
                        "decimal_places": 2,
                        "currency": "USD"
                    },
                    "target_amount": {
                        "value": 200000,
                        "decimal_places": 2,
                        "currency": "USD"
                    }
                }
            ]
        });

        Mock::given(method("GET"))
            .and(path("/helix/charity/campaigns"))
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
            "data": [
                {
                    "id": "donation001",
                    "campaign_id": "campaign123",
                    "user_id": "donor001",
                    "user_login": "generous_donor",
                    "user_name": "GenerousDonor",
                    "amount": {
                        "value": 5000,
                        "decimal_places": 2,
                        "currency": "USD"
                    }
                },
                {
                    "id": "donation002",
                    "campaign_id": "campaign123",
                    "user_id": "donor002",
                    "user_login": "kind_supporter",
                    "user_name": "KindSupporter",
                    "amount": {
                        "value": 2500,
                        "decimal_places": 2,
                        "currency": "USD"
                    }
                }
            ],
            "pagination": {
                "cursor": "eyJiI..."
            }
        });

        Mock::given(method("GET"))
            .and(path("/helix/charity/donations"))
            .and(query_param("broadcaster_id", "123456789"))
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

    pub async fn mock_charity_extra(&self) {
        let expected_response = json!({
            "data": [
                {
                    "id": "donation001",
                    "campaign_id": "campaign123",
                    "user_id": "donor001",
                    "user_login": "solo_donor",
                    "user_name": "SoloDonor",
                    "amount": {
                        "value": 10000,
                        "decimal_places": 2,
                        "currency": "USD"
                    }
                }
            ]
        });

        Mock::given(method("GET"))
            .and(path("/helix/charity/donations"))
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

    pub async fn mock_charity_failure(&self) {
        let error_response = json!({
            "error": "Not Found",
            "status": 404,
            "message": "No active charity campaign found for this broadcaster"
        });

        Mock::given(method("GET"))
            .and(path("/helix/charity/campaigns"))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_response))
            .mount(&self.server)
            .await;
    }

    pub async fn mock_chat_success(&self) {
        let expected_response = json!({
            "data": [
                {
                    "user_id": "123456789",
                    "user_login": "activeuser1",
                    "user_name": "ActiveUser1"
                },
                {
                    "user_id": "987654321",
                    "user_login": "activeuser2",
                    "user_name": "ActiveUser2"
                }
            ],
            "pagination": {
                "cursor": "eyJiI..."
            },
            "total": 150
        });

        Mock::given(method("GET"))
            .and(path("/helix/chat/chatters"))
            .and(query_param("broadcaster_id", "123456789"))
            .and(query_param("moderator_id", "987654321"))
            .and(query_param("first", "100"))
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
                    "id": "emotesv2_123",
                    "name": "testEmote1",
                    "images": {
                        "url_1x": "https://static-cdn.jtvnw.net/emoticons/v2/emotesv2_123/static/light/1.0",
                        "url_2x": "https://static-cdn.jtvnw.net/emoticons/v2/emotesv2_123/static/light/2.0",
                        "url_4x": "https://static-cdn.jtvnw.net/emoticons/v2/emotesv2_123/static/light/3.0"
                    },
                    "tier": "1000",
                    "emote_type": "subscriptions",
                    "emote_set_id": "123",
                    "format": ["static"],
                    "scale": ["1.0", "2.0", "3.0"],
                    "theme_mode": ["light", "dark"]
                }
            ],
            "template": "https://static-cdn.jtvnw.net/emoticons/v2/{{id}}/{{format}}/{{theme_mode}}/{{scale}}"
        });

        Mock::given(method("GET"))
            .and(path("/helix/chat/emotes"))
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
            "data": [
                {
                    "message_id": "msg123456789",
                    "is_sent": true,
                    "drop_reason": null
                }
            ]
        });

        Mock::given(method("POST"))
            .and(path("/helix/chat/messages"))
            .and(header(
                "authorization",
                "Bearer 2gbdx6oar67tqtcmt49t3wpcgycthx",
            ))
            .and(header("client-id", "wbmytr93xzw8zbg0p1izqyzzc5mbiz"))
            .and(header("content-type", "application/json"))
            .respond_with(ResponseTemplate::new(200).set_body_json(expected_response))
            .mount(&self.server)
            .await;

        let expected_response = json!({
            "data": [
                {
                    "broadcaster_id": "123456789",
                    "emote_mode": false,
                    "follower_mode": true,
                    "follower_mode_duration": 300,
                    "moderator_id": "987654321",
                    "non_moderator_chat_delay": false,
                    "non_moderator_chat_delay_duration": 2,
                    "slow_mode": false,
                    "show_mode_wait_time": 30,
                    "subscriber_mode": false,
                    "unique_chat_mode": true
                }
            ]
        });

        Mock::given(method("GET"))
            .and(path("/helix/chat/settings"))
            .and(query_param("broadcaster_id", "123456789"))
            .and(query_param("moderator_id", "987654321"))
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
                    "user_id": "123456789",
                    "user_name": "TestUser",
                    "user_login": "testuser",
                    "color": "#FF0000"
                }
            ]
        });

        Mock::given(method("GET"))
            .and(path("/helix/chat/color"))
            .and(query_param("user_id", "123456789"))
            .and(header(
                "authorization",
                "Bearer 2gbdx6oar67tqtcmt49t3wpcgycthx",
            ))
            .and(header("client-id", "wbmytr93xzw8zbg0p1izqyzzc5mbiz"))
            .respond_with(ResponseTemplate::new(200).set_body_json(expected_response))
            .mount(&self.server)
            .await;
    }

    pub async fn mock_chat_failure(&self) {
        let error_response = json!({
            "error": "Forbidden",
            "status": 403,
            "message": "User does not have permission to access chat"
        });

        Mock::given(method("GET"))
            .and(path("/helix/chat/chatters"))
            .respond_with(ResponseTemplate::new(403).set_body_json(error_response))
            .mount(&self.server)
            .await;
    }

    pub async fn mock_clips_success(&self) {
        let expected_response = json!({
            "data": [
                {
                    "edit_url": "https://clips.twitch.tv/edit/FantasticClip123",
                    "id": "FantasticClip123"
                }
            ]
        });

        Mock::given(method("POST"))
            .and(path("/helix/clips"))
            .and(query_param("broadcaster_id", "123456789"))
            .and(query_param("has_delay", "false"))
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
                    "edit_url": "https://clips.twitch.tv/edit/InstantClip456",
                    "id": "InstantClip456"
                }
            ]
        });

        Mock::given(method("POST"))
            .and(path("/helix/clips"))
            .and(query_param("broadcaster_id", "987654321"))
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
                    "id": "BroadcasterClip123",
                    "url": "https://clips.twitch.tv/BroadcasterClip123",
                    "embed_url": "https://clips.twitch.tv/embed?clip=BroadcasterClip123",
                    "broadcaster_id": "123456789",
                    "broadcaster_name": "MainStreamer",
                    "creator_id": "555666777",
                    "creator_name": "BigFan",
                    "video_id": "v444444444",
                    "game_id": "509658",
                    "language": "en",
                    "title": "Best Moment Ever",
                    "view_count": 10000,
                    "created_at": "2024-01-18T16:00:00Z",
                    "thumbnail_url": "https://clips-media-assets2.twitch.tv/best_moment.jpg",
                    "duration": 60.0,
                    "vod_offset": 5400,
                    "is_featured": true
                }
            ],
            "pagination": {
                "cursor": "eyJiI..."
            }
        });

        Mock::given(method("GET"))
            .and(path("/helix/clips"))
            .and(query_param("broadcaster_id", "123456789"))
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
                    "id": "GameClip789",
                    "url": "https://clips.twitch.tv/GameClip789",
                    "embed_url": "https://clips.twitch.tv/embed?clip=GameClip789",
                    "broadcaster_id": "987654321",
                    "broadcaster_name": "GameMaster",
                    "creator_id": "222333444",
                    "creator_name": "GameFan",
                    "video_id": "v555555555",
                    "game_id": "21779",
                    "language": "en",
                    "title": "Incredible Play",
                    "view_count": 7500,
                    "created_at": "2024-01-19T18:30:00Z",
                    "thumbnail_url": "https://clips-media-assets2.twitch.tv/incredible_play.jpg",
                    "duration": 35.5,
                    "vod_offset": 2700,
                    "is_featured": false
                }
            ]
        });

        Mock::given(method("GET"))
            .and(path("/helix/clips"))
            .and(query_param("game_id", "21779"))
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
                    "id": "SpecificClip1",
                    "url": "https://clips.twitch.tv/SpecificClip1",
                    "embed_url": "https://clips.twitch.tv/embed?clip=SpecificClip1",
                    "broadcaster_id": "111222333",
                    "broadcaster_name": "SpecificStreamer",
                    "creator_id": "444555666",
                    "creator_name": "SpecificCreator",
                    "video_id": "v666666666",
                    "game_id": "509658",
                    "language": "en",
                    "title": "First Specific Clip",
                    "view_count": 500,
                    "created_at": "2024-01-20T10:00:00Z",
                    "thumbnail_url": "https://clips-media-assets2.twitch.tv/specific1.jpg",
                    "duration": 15.5,
                    "vod_offset": 1200,
                    "is_featured": false
                },
                {
                    "id": "SpecificClip2",
                    "url": "https://clips.twitch.tv/SpecificClip2",
                    "embed_url": "https://clips.twitch.tv/embed?clip=SpecificClip2",
                    "broadcaster_id": "111222333",
                    "broadcaster_name": "SpecificStreamer",
                    "creator_id": "777888999",
                    "creator_name": "AnotherCreator",
                    "video_id": "v777777777",
                    "game_id": "21779",
                    "language": "en",
                    "title": "Second Specific Clip",
                    "view_count": 750,
                    "created_at": "2024-01-20T11:00:00Z",
                    "thumbnail_url": "https://clips-media-assets2.twitch.tv/specific2.jpg",
                    "duration": 25.0,
                    "vod_offset": null,
                    "is_featured": true
                }
            ]
        });

        Mock::given(method("GET"))
            .and(path("/helix/clips"))
            .and(query_param("id", "SpecificClip1"))
            .and(query_param("id", "SpecificClip2"))
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
                    "id": "RecentClip123",
                    "url": "https://clips.twitch.tv/RecentClip123",
                    "embed_url": "https://clips.twitch.tv/embed?clip=RecentClip123",
                    "broadcaster_id": "123456789",
                    "broadcaster_name": "RecentStreamer",
                    "creator_id": "999000111",
                    "creator_name": "RecentCreator",
                    "video_id": "v888888888",
                    "game_id": "516575",
                    "language": "en",
                    "title": "Recent Amazing Play",
                    "view_count": 3000,
                    "created_at": "2024-01-15T12:00:00Z",
                    "thumbnail_url": "https://clips-media-assets2.twitch.tv/recent_clip.jpg",
                    "duration": 40.0,
                    "vod_offset": 3000,
                    "is_featured": true
                }
            ]
        });

        Mock::given(method("GET"))
            .and(path("/helix/clips"))
            .and(query_param("broadcaster_id", "123456789"))
            .and(query_param("started_at", "2024-01-01T00:00:00Z"))
            .and(query_param("ended_at", "2024-01-31T23:59:59Z"))
            .and(query_param("is_featured", "true"))
            .and(header(
                "authorization",
                "Bearer 2gbdx6oar67tqtcmt49t3wpcgycthx",
            ))
            .and(header("client-id", "wbmytr93xzw8zbg0p1izqyzzc5mbiz"))
            .respond_with(ResponseTemplate::new(200).set_body_json(expected_response))
            .mount(&self.server)
            .await;
    }

    pub async fn mock_clips_failure(&self) {
        let error_response = json!({
            "error": "Unauthorized",
            "status": 401,
            "message": "User does not have permission to create clips"
        });

        Mock::given(method("POST"))
            .and(path("/helix/clips"))
            .respond_with(ResponseTemplate::new(401).set_body_json(error_response))
            .mount(&self.server)
            .await;
    }

    pub async fn mock_entitlements_success(&self) {
        let expected_response = json!({
            "data": [
                {
                    "id": "entitlement123",
                    "benefit_id": "benefit456",
                    "timestamp": "2024-01-15T10:30:00Z",
                    "user_id": "user789",
                    "game_id": "game012",
                    "fulfillment_status": "CLAIMED",
                    "last_updated": "2024-01-15T11:00:00Z"
                },
                {
                    "id": "entitlement456",
                    "benefit_id": "benefit789",
                    "timestamp": "2024-01-14T15:20:00Z",
                    "user_id": "user123",
                    "game_id": "game345",
                    "fulfillment_status": "FULFILLED",
                    "last_updated": "2024-01-14T16:45:00Z"
                }
            ],
            "pagination": {
                "cursor": "eyJiI..."
            }
        });

        Mock::given(method("GET"))
            .and(path("/helix/entitlements/drops"))
            .and(query_param("user_id", "user789"))
            .and(query_param("game_id", "game012"))
            .and(query_param("fulfillment_status", "CLAIMED"))
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
                    "id": "specific_ent_1",
                    "benefit_id": "specific_benefit_1",
                    "timestamp": "2024-01-15T08:00:00Z",
                    "user_id": "specific_user",
                    "game_id": "specific_game",
                    "fulfillment_status": "CLAIMED",
                    "last_updated": "2024-01-15T08:30:00Z"
                },
                {
                    "id": "specific_ent_2",
                    "benefit_id": "specific_benefit_2",
                    "timestamp": "2024-01-15T09:00:00Z",
                    "user_id": "specific_user",
                    "game_id": "specific_game",
                    "fulfillment_status": "FULFILLED",
                    "last_updated": "2024-01-15T09:45:00Z"
                }
            ]
        });

        Mock::given(method("GET"))
            .and(path("/helix/entitlements/drops"))
            .and(query_param("id", "specific_ent_1"))
            .and(query_param("id", "specific_ent_2"))
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
                    "id": "all_ent_1",
                    "benefit_id": "all_benefit_1",
                    "timestamp": "2024-01-10T12:00:00Z",
                    "user_id": "all_user_1",
                    "game_id": "all_game_1",
                    "fulfillment_status": "CLAIMED",
                    "last_updated": "2024-01-10T12:30:00Z"
                }
            ]
        });

        Mock::given(method("GET"))
            .and(path("/helix/entitlements/drops"))
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
                    "status": "SUCCESS",
                    "ids": ["update_ent_1", "update_ent_2"]
                },
                {
                    "status": "NOT_FOUND",
                    "ids": ["update_ent_3"]
                }
            ]
        });

        Mock::given(method("PATCH"))
            .and(path("/helix/entitlements/drops"))
            .and(header(
                "authorization",
                "Bearer 2gbdx6oar67tqtcmt49t3wpcgycthx",
            ))
            .and(header("client-id", "wbmytr93xzw8zbg0p1izqyzzc5mbiz"))
            .and(header("content-type", "application/json"))
            .and(body_json(json!({
                "entitlement_ids": [
                    "update_ent_1",
                    "update_ent_2",
                    "update_ent_3"
                ],
                "fulfillment_status": "FULFILLED"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(expected_response))
            .mount(&self.server)
            .await;
    }

    pub async fn mock_entitlements_extra(&self) {
        let expected_response = json!({
            "data": []
        });

        Mock::given(method("PATCH"))
            .and(path("/helix/entitlements/drops"))
            .and(header(
                "authorization",
                "Bearer 2gbdx6oar67tqtcmt49t3wpcgycthx",
            ))
            .and(header("client-id", "wbmytr93xzw8zbg0p1izqyzzc5mbiz"))
            .and(header("content-type", "application/json"))
            .respond_with(ResponseTemplate::new(200).set_body_json(expected_response))
            .mount(&self.server)
            .await;
    }

    pub async fn mock_entitlements_failure(&self) {
        let error_response = json!({
            "error": "Forbidden",
            "status": 403,
            "message": "User does not have permission to access entitlements"
        });

        Mock::given(method("GET"))
            .and(path("/helix/entitlements/drops"))
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
