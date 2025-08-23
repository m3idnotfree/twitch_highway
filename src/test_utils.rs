#![allow(unused_imports)]
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

    pub fn execute<F, T>(&self, f: F) -> TwitchAPIRequest<T>
    where
        F: FnOnce(&TwitchAPI) -> TwitchAPIRequest<T>,
        T: DeserializeOwned,
    {
        let resp = f(&self.api);

        let url = resp.url().clone();

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
                .join(url.path())
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
                "data": [{
                    "length": 60,
                    "message": "",
                    "retry_after": 480
                }]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn get_ad_schedule(&self) {
        self.api_mock("GET", "/channels/ads")
            .and(query_param("broadcaster_id", "123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [{
                    "next_ad_at": "2023-08-01T23:08:18+00:00",
                    "last_ad_at": "2023-08-01T23:08:18+00:00",
                    "duration": "60",
                    "preroll_free_time": "90",
                    "snooze_count": "1",
                    "snooze_refresh_at": "2023-08-01T23:08:18+00:00"
                }]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn snooze_next_ad(&self) {
        self.api_mock("POST", "/channels/ads/schedule/snooze")
            .and(query_param("broadcaster_id", "123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [{
                    "snooze_count": "1",
                    "snooze_refresh_at": "2023-08-01T23:08:18+00:00",
                    "next_ad_at": "2023-08-01T23:08:18+00:00"
                }]
            })))
            .mount(&self.server)
            .await;
    }
}

#[cfg(feature = "analytics")]
impl TwitchApiTest {
    pub async fn get_extension_analytics(&self) {
        self.api_mock("GET", "/analytics/extensions")
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [{
                    "extension_id": "efgh",
                    "URL": "https://twitch-piper-reports.s3-us-west-2.amazonaws.com/dynamic/LoL%20ADC...",
                    "type": "overview_v2",
                    "date_range": {
                        "started_at": "2018-03-01T00:00:00Z",
                        "ended_at": "2018-06-01T00:00:00Z"
                    }
                }],
                "pagination": {
                    "cursor": "eyJiIjpudWxsLCJhIjp7Ik9mZnNldCI6NX19"
                }
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn get_game_analytics(&self) {
        self.api_mock("GET", "/analytics/games")
            .and(query_param("game_id", "493057"))
            .and(query_param("started_at", "2018-01-01T00:00:00Z"))
            .and(query_param("ended_at", "2018-03-01T00:00:00Z"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [{
                    "game_id": "493057",
                    "URL": "https://twitch-piper-reports.s3-us-west-2.amazonaws.com/games/66170/overview/15183...",
                    "type": "overview_v2",
                    "date_range": {
                        "started_at": "2018-01-01T00:00:00Z",
                        "ended_at": "2018-03-01T00:00:00Z"
                    }
                }]
            })))
            .mount(&self.server)
            .await;
    }
    pub async fn get_game_analytics2(&self) {
        self.api_mock("GET", "/analytics/games")
            .and(query_param("first", "5"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "game_id": "9821",
                        "URL": "https://twitch-piper-reports.s3-us-west-2.amazonaws.com/games/9821/overview/152642...",
                        "type" : "overview_v2",
                        "date_range" : {
                            "started_at" : "2018-03-13T00:00:00Z",
                            "ended_at" : "2018-06-13T00:00:00Z"
                        }
                    },
                ],
                "pagination": {"cursor": "eyJiIjpudWxsLJxhIjoiIn0gf5"}
            })))
            .mount(&self.server)
            .await;
    }
}

#[cfg(feature = "bits")]
impl TwitchApiTest {
    pub async fn get_bits_leaderboard(&self) {
        self.api_mock("GET", "/bits/leaderboard")
            .and(query_param("count", "2"))
            .and(query_param("period", "week"))
            .and(query_param("started_at", "2018-02-05T08:00:00Z"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [{
                    "user_id": "158010205",
                    "user_login": "tundracowboy",
                    "user_name": "TundraCowboy",
                    "rank": 1,
                    "score": 12543
                }, {
                    "user_id": "7168163",
                    "user_login": "topramens",
                    "user_name": "Topramens",
                    "rank": 2,
                    "score": 6900
                }],
                "date_range": {
                    "started_at": "2018-02-05T08:00:00Z",
                    "ended_at": "2018-02-12T08:00:00Z"
                },
                "total": 2
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn get_cheermotes(&self) {
        self.api_mock("GET", "/bits/cheermotes")
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [{
                    "prefix": "Cheer",
                    "tiers": [{
                        "min_bits": 1,
                        "id": "1",
                        "color": "#979797",
                        "images": {
                            "dark": {
                                "animated": {
                                    "1": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/animated/1/1.gif",
                                    "1.5": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/animated/1/1.5.gif",
                                    "2": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/animated/1/2.gif",
                                    "3": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/animated/1/3.gif",
                                    "4": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/animated/1/4.gif"
                                },
                                "static": {
                                    "1": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/static/1/1.png",
                                    "1.5": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/static/1/1.5.png",
                                    "2": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/static/1/2.png",
                                    "3": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/static/1/3.png",
                                    "4": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/static/1/4.png"
                                }
                            },
                            "light": {
                                "animated": {
                                    "1": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/animated/1/1.gif",
                                    "1.5": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/animated/1/1.5.gif",
                                    "2": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/animated/1/2.gif",
                                    "3": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/animated/1/3.gif",
                                    "4": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/animated/1/4.gif"
                                },
                                "static": {
                                    "1": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/static/1/1.png",
                                    "1.5": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/static/1/1.5.png",
                                    "2": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/static/1/2.png",
                                    "3": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/static/1/3.png",
                                    "4": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/static/1/4.png"
                                }
                            }
                        },
                        "can_cheer": true,
                        "show_in_bits_card": true
                    }],
                    "type": "global_first_party",
                    "order": 1,
                    "last_updated": "2018-05-22T00:06:04Z",
                    "is_charitable": false
                }]
            })))
            .mount(&self.server)
            .await;
    }
    pub async fn get_cheermotes2(&self) {
        self.api_mock("GET", "/bits/cheermotes")
            .and(query_param("broadcaster_id", "41245072"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [{
                    "prefix": "Cheer",
                    "tiers": [{
                        "min_bits": 1,
                        "id": "1",
                        "color": "#979797",
                        "images": {
                            "dark": {
                                "animated": {
                                    "1": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/animated/1/1.gif",
                                    "1.5": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/animated/1/1.5.gif",
                                    "2": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/animated/1/2.gif",
                                    "3": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/animated/1/3.gif",
                                    "4": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/animated/1/4.gif"
                                },
                                "static": {
                                    "1": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/static/1/1.png",
                                    "1.5": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/static/1/1.5.png",
                                    "2": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/static/1/2.png",
                                    "3": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/static/1/3.png",
                                    "4": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/dark/static/1/4.png"
                                }
                            },
                            "light": {
                                "animated": {
                                    "1": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/animated/1/1.gif",
                                    "1.5": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/animated/1/1.5.gif",
                                    "2": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/animated/1/2.gif",
                                    "3": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/animated/1/3.gif",
                                    "4": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/animated/1/4.gif"
                                },
                                "static": {
                                    "1": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/static/1/1.png",
                                    "1.5": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/static/1/1.5.png",
                                    "2": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/static/1/2.png",
                                    "3": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/static/1/3.png",
                                    "4": "https://d3aqoihi2n8ty8.cloudfront.net/actions/cheer/light/static/1/4.png"
                                }
                            }
                        },
                        "can_cheer": true,
                        "show_in_bits_card": true
                    }],
                    "type": "global_first_party",
                    "order": 1,
                    "last_updated": "2018-05-22T00:06:04Z",
                    "is_charitable": false
                }]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn get_extension_transactions(&self) {
        self.api_mock("GET", "/extensions/transactions")
            .and(query_param("extension_id", "1234"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [{
                    "id": "74c52265-e214-48a6-91b9-23b6014e8041",
                    "timestamp": "2019-01-28T04:15:53.325Z",
                    "broadcaster_id": "439964613",
                    "broadcaster_login": "chikuseuma",
                    "broadcaster_name": "chikuseuma",
                    "user_id": "424596340",
                    "user_login": "quotrok",
                    "user_name": "quotrok",
                    "product_type": "BITS_IN_EXTENSION",
                    "product_data": {
                        "domain": "twitch.ext.uo6dggojyb8d6soh92zknwmi5ej1q2",
                        "sku": "testSku100",
                        "cost": {
                            "amount": 100,
                            "type": "bits"
                        },
                        "inDevelopment": false,
                        "displayName": "Test Product 100",
                        "expiration": "",
                        "broadcast": false
                    }
                }],
                "pagination": {
                    "cursor": "cursorString"
                }
            })))
            .mount(&self.server)
            .await;
    }
}

#[cfg(feature = "channels")]
impl TwitchApiTest {
    pub async fn get_channel_info(&self) {
        self.api_mock("GET", "/channels")
            .and(query_param("broadcaster_id", "141981764"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [{
                    "broadcaster_id": "141981764",
                    "broadcaster_login": "twitchdev",
                    "broadcaster_name": "TwitchDev",
                    "broadcaster_language": "en",
                    "game_id": "509670",
                    "game_name": "Science & Technology",
                    "title": "TwitchDev Monthly Update // May 6, 2021",
                    "delay": 0,
                    "tags": ["DevsInTheKnow"],
                    "content_classification_labels": ["Gambling", "DrugsIntoxication", "MatureGame"],
                    "is_branded_content": false
                }]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn modify_channel_info(&self) {
        self.api_mock("PATCH", "/channels")
            .and(query_param("broadcaster_id", "41245072"))
            .and(header("content-type", "application/json"))
            .and(body_json(json!({
                "game_id": "33214",
                "title": "there are helicopters in the game? REASON TO PLAY FORTNITE found",
                "broadcaster_language": "en",
                "tags": ["LevelingUp"]
            })))
            .respond_with(ResponseTemplate::new(204))
            .mount(&self.server)
            .await;
    }
    pub async fn modify_channel_info2(&self) {
        self.api_mock("PATCH", "/channels")
            .and(query_param("broadcaster_id", "41245072"))
            .and(header("content-type", "application/json"))
            .and(body_json(json!({
                "game_id": "SomeGameID",
                "content_classification_labels": [
                    {"id": "Gambling", "is_enabled": true},
                    {"id": "DrugsIntoxication", "is_enabled": false}
                ],
                "is_branded_content": true
            })))
            .respond_with(ResponseTemplate::new(204))
            .mount(&self.server)
            .await;
    }

    pub async fn get_channel_editors(&self) {
        self.api_mock("GET", "/channels/editors")
            .and(query_param("broadcaster_id", "141981764"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [{
                    "user_id": "182891647",
                    "user_name": "mauerbac",
                    "created_at": "2019-02-15T21:19:50.380833Z"
                }, {
                    "user_id": "135093069",
                    "user_name": "BlueLava",
                    "created_at": "2018-03-07T16:28:29.872937Z"
                }]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn get_followed_channels(&self) {
        self.api_mock("GET", "/channels/followed")
            .and(query_param("user_id", "123456"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "total": 8,
                "data": [{
                    "broadcaster_id": "11111",
                    "broadcaster_login": "userloginname",
                    "broadcaster_name": "UserDisplayName",
                    "followed_at": "2022-05-24T22:22:08Z"
                }],
                "pagination": {
                    "cursor": "eyJiIjpudWxsLCJhIjp7Ik9mZnNldCI6NX19"
                }
            })))
            .mount(&self.server)
            .await;
    }
    pub async fn get_followed_channels2(&self) {
        self.api_mock("GET", "/channels/followed")
            .and(query_param("user_id", "123456"))
            .and(query_param("broadcaster_id", "654321"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "total": 8,
                "data": [
                    {
                        "broadcaster_id": "654321",
                        "broadcaster_login": "basketweaver101",
                        "broadcaster_name": "BasketWeaver101",
                        "followed_at": "2022-05-24T22:22:08Z"
                    }
                ],
                "pagination": {}
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn get_channel_followers(&self) {
        self.api_mock("GET", "/channels/followers")
            .and(query_param("broadcaster_id", "123456"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "total": 8,
                "data": [{
                    "user_id": "11111",
                    "user_name": "UserDisplayName",
                    "user_login": "userloginname",
                    "followed_at": "2022-05-24T22:22:08Z"
                }],
                "pagination": {
                    "cursor": "eyJiIjpudWxsLCJhIjp7Ik9mZnNldCI6NX19"
                }
            })))
            .mount(&self.server)
            .await;
    }
    pub async fn get_channel_followers2(&self) {
        self.api_mock("GET", "/channels/followers")
            .and(query_param("broadcaster_id", "123456"))
            .and(query_param("user_id", "654321"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "total": 8,
                "data": [],
                "pagination": {}
            })))
            .mount(&self.server)
            .await;
    }
}

#[cfg(feature = "channel-points")]
impl TwitchApiTest {
    pub async fn create_custom_rewards(&self) {
        self.api_mock("POST", "/channel_points/custom_rewards")
            .and(query_param("broadcaster_id", "274637212"))
            .and(header("content-type", "application/json"))
            .and(body_json(json!({
                "title": "game analysis 1v1",
                "cost": 50000
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [{
                    "broadcaster_name": "torpedo09",
                    "broadcaster_login": "torpedo09",
                    "broadcaster_id": "274637212",
                    "id": "afaa7e34-6b17-49f0-a19a-d1e76eaaf673",
                    "image": null,
                    "background_color": "#00E5CB",
                    "is_enabled": true,
                    "cost": 50000,
                    "title": "game analysis 1v1",
                    "prompt": "",
                    "is_user_input_required": false,
                    "max_per_stream_setting": {
                        "is_enabled": false,
                        "max_per_stream": 0
                    },
                    "max_per_user_per_stream_setting": {
                        "is_enabled": false,
                        "max_per_user_per_stream": 0
                    },
                    "global_cooldown_setting": {
                        "is_enabled": false,
                        "global_cooldown_seconds": 0
                    },
                    "is_paused": false,
                    "is_in_stock": true,
                    "default_image": {
                        "url_1x": "https://static-cdn.jtvnw.net/custom-reward-images/default-1.png",
                        "url_2x": "https://static-cdn.jtvnw.net/custom-reward-images/default-2.png",
                        "url_4x": "https://static-cdn.jtvnw.net/custom-reward-images/default-4.png"
                    },
                    "should_redemptions_skip_request_queue": false,
                    "redemptions_redeemed_current_stream": null,
                    "cooldown_expires_at": null
                }]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn delete_custom_reward(&self) {
        self.api_mock("DELETE", "/channel_points/custom_rewards")
            .and(query_param("broadcaster_id", "274637212"))
            .and(query_param("id", "b045196d-9ce7-4a27-a9b9-279ed341ab28"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&self.server)
            .await;
    }

    pub async fn get_custom_reward(&self) {
        self.api_mock("GET", "/channel_points/custom_rewards")
            .and(query_param("broadcaster_id", "274637212"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [{
                    "broadcaster_name": "torpedo09",
                    "broadcaster_login": "torpedo09",
                    "broadcaster_id": "274637212",
                    "id": "92af127c-7326-4483-a52b-b0da0be61c01",
                    "image": null,
                    "background_color": "#00E5CB",
                    "is_enabled": true,
                    "cost": 50000,
                    "title": "game analysis",
                    "prompt": "",
                    "is_user_input_required": false,
                    "max_per_stream_setting": {
                        "is_enabled": false,
                        "max_per_stream": 0
                    },
                    "max_per_user_per_stream_setting": {
                        "is_enabled": false,
                        "max_per_user_per_stream": 0
                    },
                    "global_cooldown_setting": {
                        "is_enabled": false,
                        "global_cooldown_seconds": 0
                    },
                    "is_paused": false,
                    "is_in_stock": true,
                    "default_image": {
                        "url_1x": "https://static-cdn.jtvnw.net/custom-reward-images/default-1.png",
                        "url_2x": "https://static-cdn.jtvnw.net/custom-reward-images/default-2.png",
                        "url_4x": "https://static-cdn.jtvnw.net/custom-reward-images/default-4.png"
                    },
                    "should_redemptions_skip_request_queue": false,
                    "redemptions_redeemed_current_stream": null,
                    "cooldown_expires_at": null
                }]
            })))
            .mount(&self.server)
            .await;
    }
    pub async fn get_custom_reward2(&self) {
        self.api_mock("GET", "/channel_points/custom_rewards")
            .and(query_param("broadcaster_id", "274637212"))
            .and(query_param("only_manageable_rewards", "true"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "broadcaster_name": "torpedo09",
                        "broadcaster_id": "274637212",
                        "id": "92af127c-7326-4483-a52b-b0da0be61c01",
                        "image": null,
                        "background_color": "#00E5CB",
                        "is_enabled": true,
                        "cost": 50000,
                        "title": "game analysis",
                        "prompt": "",
                        "is_user_input_required": false,
                        "max_per_stream_setting": {
                            "is_enabled": false,
                            "max_per_stream": 0
                        },
                        "max_per_user_per_stream_setting": {
                            "is_enabled": false,
                            "max_per_user_per_stream": 0
                        },
                        "global_cooldown_setting": {
                            "is_enabled": false,
                            "global_cooldown_seconds": 0
                        },
                        "is_paused": false,
                        "is_in_stock": true,
                        "default_image": {
                            "url_1x": "https://static-cdn.jtvnw.net/custom-reward-images/default-1.png",
                            "url_2x": "https://static-cdn.jtvnw.net/custom-reward-images/default-2.png",
                            "url_4x": "https://static-cdn.jtvnw.net/custom-reward-images/default-4.png"
                        },
                        "should_redemptions_skip_request_queue": false,
                        "redemptions_redeemed_current_stream": null,
                        "cooldown_expires_at": null
                    }
                ]
            })))
            .mount(&self.server)
            .await;
    }
    pub async fn get_custom_reward3(&self) {
        self.api_mock("GET", "/channel_points/custom_rewards")
            .and(query_param("broadcaster_id", "274637212"))
            .and(query_param("id", "2af127c-7326-4483-a52b-b0da0be61c01"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "broadcaster_name": "torpedo09",
                        "broadcaster_id": "274637212",
                        "id": "92af127c-7326-4483-a52b-b0da0be61c01",
                        "image": null,
                        "background_color": "#00E5CB",
                        "is_enabled": true,
                        "cost": 50000,
                        "title": "game analysis",
                        "prompt": "",
                        "is_user_input_required": false,
                        "max_per_stream_setting": {
                            "is_enabled": false,
                            "max_per_stream": 0
                        },
                        "max_per_user_per_stream_setting": {
                            "is_enabled": false,
                            "max_per_user_per_stream": 0
                        },
                        "global_cooldown_setting": {
                            "is_enabled": false,
                            "global_cooldown_seconds": 0
                        },
                        "is_paused": false,
                        "is_in_stock": true,
                        "default_image": {
                            "url_1x": "https://static-cdn.jtvnw.net/custom-reward-images/default-1.png",
                            "url_2x": "https://static-cdn.jtvnw.net/custom-reward-images/default-2.png",
                            "url_4x": "https://static-cdn.jtvnw.net/custom-reward-images/default-4.png"
                        },
                        "should_redemptions_skip_request_queue": false,
                        "redemptions_redeemed_current_stream": null,
                        "cooldown_expires_at": null
                    }
                ]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn get_custom_reward_redemption(&self) {
        self.api_mock("GET", "/channel_points/custom_rewards/redemptions")
            .and(query_param("broadcaster_id", "274637212"))
            .and(query_param("reward_id", "92af127c-7326-4483-a52b-b0da0be61c01"))
            .and(query_param("status", "CANCELED"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [{
                    "broadcaster_name": "torpedo09",
                    "broadcaster_login": "torpedo09",
                    "broadcaster_id": "274637212",
                    "id": "17fa2df1-ad76-4804-bfa5-a40ef63efe63",
                    "user_login": "torpedo09",
                    "user_id": "274637212",
                    "user_name": "torpedo09",
                    "user_input": "",
                    "status": "CANCELED",
                    "redeemed_at": "2020-07-01T18:37:32Z",
                    "reward": {
                        "id": "92af127c-7326-4483-a52b-b0da0be61c01",
                        "title": "game analysis",
                        "prompt": "",
                        "cost": 50000
                    }
                }],
                "pagination": {
                    "cursor": "eyJiIjpudWxsLCJhIjp7IkN1cnNvciI6Ik1UZG1ZVEprWmpFdFlXUTNOaTAwT0RBMExXSm1ZVFV0WVRRd1pXWTJNMlZtWlRZelgxOHlNREl3TFRBM0xUQXhWREU0T2pNM09qTXlMakl6TXpFeU56RTFOMW89In19"
                }
            })))
            .mount(&self.server)
            .await;
    }
    pub async fn get_custom_reward_redemption2(&self) {
        self.api_mock("GET", "/channel_points/custom_rewards/redemptions")
            .and(query_param("broadcaster_id", "274637212"))
            .and(query_param(
                "reward_id",
                "92af127c-7326-4483-a52b-b0da0be61c01",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "broadcaster_name": "torpedo09",
                        "broadcaster_login": "torpedo09",
                        "broadcaster_id": "274637212",
                        "id": "17fa2df1-ad76-4804-bfa5-a40ef63efe63",
                        "user_id": "274637212",
                        "user_name": "torpedo09",
                        "user_input": "",
                        "status": "CANCELED",
                        "redeemed_at": "2020-07-01T18:37:32Z",
                        "reward": {
                            "id": "92af127c-7326-4483-a52b-b0da0be61c01",
                            "title": "game analysis",
                            "prompt": "",
                            "cost": 50000
                        }
                    }
                ]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn update_custom_reward(&self) {
        self.api_mock("PATCH", "/channel_points/custom_rewards")
            .and(query_param("broadcaster_id", "274637212"))
            .and(query_param("id", "92af127c-7326-4483-a52b-b0da0be61c01"))
            .and(header("content-type", "application/json"))
            .and(body_json(json!({
                "is_enabled": false
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [{
                    "broadcaster_name": "torpedo09",
                    "broadcaster_login": "torpedo09",
                    "broadcaster_id": "274637212",
                    "id": "92af127c-7326-4483-a52b-b0da0be61c01",
                    "image": null,
                    "background_color": "#00E5CB",
                    "is_enabled": false,
                    "cost": 30000,
                    "title": "game analysis 2v2",
                    "prompt": "",
                    "is_user_input_required": false,
                    "max_per_stream_setting": {
                        "is_enabled": true,
                        "max_per_stream": 60
                    },
                    "max_per_user_per_stream_setting": {
                        "is_enabled": false,
                        "max_per_user_per_stream": 0
                    },
                    "global_cooldown_setting": {
                        "is_enabled": false,
                        "global_cooldown_seconds": 0
                    },
                    "is_paused": false,
                    "is_in_stock": false,
                    "default_image": {
                        "url_1x": "https://static-cdn.jtvnw.net/custom-reward-images/default-1.png",
                        "url_2x": "https://static-cdn.jtvnw.net/custom-reward-images/default-2.png",
                        "url_4x": "https://static-cdn.jtvnw.net/custom-reward-images/default-4.png"
                    },
                    "should_redemptions_skip_request_queue": true,
                    "redemptions_redeemed_current_stream": 60,
                    "cooldown_expires_at": null
                }]
            })))
            .mount(&self.server)
            .await;
    }
    pub async fn update_custom_reward2(&self) {
        self.api_mock("PATCH", "/channel_points/custom_rewards")
            .and(query_param("broadcaster_id", "274637212"))
            .and(query_param("id", "92af127c-7326-4483-a52b-b0da0be61c01"))
            .and(header("content-type", "application/json"))
            .and(body_json(json!({
                "title": "game analysis 2v2"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [{
                    "broadcaster_name": "torpedo09",
                    "broadcaster_login": "torpedo09",
                    "broadcaster_id": "274637212",
                    "id": "92af127c-7326-4483-a52b-b0da0be61c01",
                    "image": null,
                    "background_color": "#00E5CB",
                    "is_enabled": false,
                    "cost": 30000,
                    "title": "game analysis 2v2",
                    "prompt": "",
                    "is_user_input_required": false,
                    "max_per_stream_setting": {
                        "is_enabled": true,
                        "max_per_stream": 60
                    },
                    "max_per_user_per_stream_setting": {
                        "is_enabled": false,
                        "max_per_user_per_stream": 0
                    },
                    "global_cooldown_setting": {
                        "is_enabled": false,
                        "global_cooldown_seconds": 0
                    },
                    "is_paused": false,
                    "is_in_stock": false,
                    "default_image": {
                        "url_1x": "https://static-cdn.jtvnw.net/custom-reward-images/default-1.png",
                        "url_2x": "https://static-cdn.jtvnw.net/custom-reward-images/default-2.png",
                        "url_4x": "https://static-cdn.jtvnw.net/custom-reward-images/default-4.png"
                    },
                    "should_redemptions_skip_request_queue": true,
                    "redemptions_redeemed_current_stream": 60,
                    "cooldown_expires_at": null
                }]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn update_redemption_status(&self) {
        self.api_mock("PATCH", "/channel_points/custom_rewards/redemptions")
            .and(query_param("broadcaster_id", "274637212"))
            .and(query_param(
                "reward_id",
                "92af127c-7326-4483-a52b-b0da0be61c01",
            ))
            .and(query_param("id", "17fa2df1-ad76-4804-bfa5-a40ef63efe63"))
            .and(header("content-type", "application/json"))
            .and(body_json(json!({
                "status": "CANCELED"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [{
                    "broadcaster_name": "torpedo09",
                    "broadcaster_login": "torpedo09",
                    "broadcaster_id": "274637212",
                    "id": "17fa2df1-ad76-4804-bfa5-a40ef63efe63",
                    "user_id": "274637212",
                    "user_name": "torpedo09",
                    "user_login": "torpedo09",
                    "user_input": "",
                    "status": "CANCELED",
                    "redeemed_at": "2020-07-01T18:37:32Z",
                    "reward": {
                        "id": "92af127c-7326-4483-a52b-b0da0be61c01",
                        "title": "game analysis",
                        "prompt": "",
                        "cost": 50000
                    }
                }]
            })))
            .mount(&self.server)
            .await;
    }
}

#[cfg(feature = "charity")]
impl TwitchApiTest {
    pub async fn get_charity_campaign(&self) {
        self.api_mock("GET", "/charity/campaigns")
            .and(query_param("broadcaster_id", "123456"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "id": "123-abc-456-def",
                        "broadcaster_id": "123456",
                        "broadcaster_name": "SunnySideUp",
                        "broadcaster_login": "sunnysideup",
                        "charity_name": "Example name",
                        "charity_description": "Example description",
                        "charity_logo": "https://abc.cloudfront.net/ppgf/1000/100.png",
                        "charity_website": "https://www.example.com",
                        "current_amount": {
                            "value": 86000,
                            "decimal_places": 2,
                            "currency": "USD"
                        },
                        "target_amount": {
                            "value": 1500000,
                            "decimal_places": 2,
                            "currency": "USD"
                        }
                    }
                ]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn get_charity_campaign_donations(&self) {
        self.api_mock("GET", "/charity/donations")
            .and(query_param("broadcaster_id", "123456"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "id": "a1b2c3-aabb-4455-d1e2f3",
                        "campaign_id": "123-abc-456-def",
                        "user_id": "5678",
                        "user_login": "cool_user",
                        "user_name": "Cool_User",
                        "amount": {
                            "value": 500,
                            "decimal_places": 2,
                            "currency": "USD"
                        }
                    },
                    {
                        "id": "z1y2x3-ccdd-6677-d1e2f3",
                        "campaign_id": "123-abc-456-def",
                        "user_id": "8765",
                        "user_login": "cool_user2",
                        "user_name": "Cool_User2",
                        "amount": {
                            "value": 10000,
                            "decimal_places": 2,
                            "currency": "USD"
                        }
                    },
                ],
                "pagination" : {
                    "cursor" : "eyJiIjpudWxsLCJhIjp7Ik9mZnNldCI6NX19"
                }
            })))
            .mount(&self.server)
            .await;
    }
}

#[cfg(feature = "chat")]
impl TwitchApiTest {
    pub async fn get_chatters(&self) {
        self.api_mock("GET", "/chat/chatters")
            .and(query_param("broadcaster_id", "123456"))
            .and(query_param("moderator_id", "654321"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "user_id": "128393656",
                        "user_login": "smittysmithers",
                        "user_name": "smittysmithers"
                    },
                ],
                "pagination": {
                    "cursor": "eyJiIjpudWxsLCJhIjp7Ik9mZnNldCI6NX19"
                },
                "total": 8
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn get_channel_emotes(&self) {
        self.api_mock("GET", "/chat/emotes")
            .and(query_param("broadcaster_id", "141981764"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({

                "data": [
                    {
                        "id": "304456832",
                        "name": "twitchdevPitchfork",
                        "images": {
                            "url_1x": "https://static-cdn.jtvnw.net/emoticons/v2/304456832/static/light/1.0",
                            "url_2x": "https://static-cdn.jtvnw.net/emoticons/v2/304456832/static/light/2.0",
                            "url_4x": "https://static-cdn.jtvnw.net/emoticons/v2/304456832/static/light/3.0"
                        },
                        "tier": "1000",
                        "emote_type": "subscriptions",
                        "emote_set_id": "301590448",
                        "format": [
                            "static"
                        ],
                        "scale": [
                            "1.0",
                            "2.0",
                            "3.0"
                        ],
                        "theme_mode": [
                            "light",
                            "dark"
                        ]
                    },
                    {
                        "id": "emotesv2_4c3b4ed516de493bbcd2df2f5d450f49",
                        "name": "twitchdevHyperPitchfork",
                        "images": {
                            "url_1x": "https://static-cdn.jtvnw.net/emoticons/v2/emotesv2_4c3b4ed516de493bbcd2df2f5d450f49/static/light/1.0",
                            "url_2x": "https://static-cdn.jtvnw.net/emoticons/v2/emotesv2_4c3b4ed516de493bbcd2df2f5d450f49/static/light/2.0",
                            "url_4x": "https://static-cdn.jtvnw.net/emoticons/v2/emotesv2_4c3b4ed516de493bbcd2df2f5d450f49/static/light/3.0"
                        },
                        "tier": "1000",
                        "emote_type": "subscriptions",
                        "emote_set_id": "318939165",
                        "format": [
                            "static",
                            "animated"
                        ],
                        "scale": [
                            "1.0",
                            "2.0",
                            "3.0"
                        ],
                        "theme_mode": [
                            "light",
                            "dark"
                        ]
                    },
                ],
                "template": "https://static-cdn.jtvnw.net/emoticons/v2/{{id}}/{{format}}/{{theme_mode}}/{{scale}}"

            })))
            .mount(&self.server)
            .await;
    }

    pub async fn get_global_emotes(&self) {
        self.api_mock("GET", "/chat/emotes/global")
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "id": "196892",
                        "name": "TwitchUnity",
                        "images": {
                            "url_1x": "https://static-cdn.jtvnw.net/emoticons/v2/196892/static/light/1.0",
                            "url_2x": "https://static-cdn.jtvnw.net/emoticons/v2/196892/static/light/2.0",
                            "url_4x": "https://static-cdn.jtvnw.net/emoticons/v2/196892/static/light/3.0"
                        },
                        "format": [
                            "static"
                        ],
                        "scale": [
                            "1.0",
                            "2.0",
                            "3.0"
                        ],
                        "theme_mode": [
                            "light",
                            "dark"
                        ]
                    },
                ],
                "template": "https://static-cdn.jtvnw.net/emoticons/v2/{{id}}/{{format}}/{{theme_mode}}/{{scale}}"
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn get_emote_sets(&self) {
        self.api_mock("GET", "/chat/emotes/set")
            .and(query_param("emote_set_id", "301590448"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "id": "304456832",
                        "name": "twitchdevPitchfork",
                        "images": {
                            "url_1x": "https://static-cdn.jtvnw.net/emoticons/v2/304456832/static/light/1.0",
                            "url_2x": "https://static-cdn.jtvnw.net/emoticons/v2/304456832/static/light/2.0",
                            "url_4x": "https://static-cdn.jtvnw.net/emoticons/v2/304456832/static/light/3.0"
                        },
                        "emote_type": "subscriptions",
                        "emote_set_id": "301590448",
                        "owner_id": "141981764",
                        "format": [
                            "static"
                        ],
                        "scale": [
                            "1.0",
                            "2.0",
                            "3.0"
                        ],
                        "theme_mode": [
                            "light",
                            "dark"
                        ]
                    },
                ],
                "template": "https://static-cdn.jtvnw.net/emoticons/v2/{{id}}/{{format}}/{{theme_mode}}/{{scale}}"
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn get_channel_chat_badges(&self) {
        self.api_mock("GET", "/chat/badges")
            .and(query_param("broadcaster_id", "135093069"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "set_id": "bits",
                        "versions": [
                            {
                                "id": "1",
                                "image_url_1x": "https://static-cdn.jtvnw.net/badges/v1/743a0f3b-84b3-450b-96a0-503d7f4a9764/1",
                                "image_url_2x": "https://static-cdn.jtvnw.net/badges/v1/743a0f3b-84b3-450b-96a0-503d7f4a9764/2",
                                "image_url_4x": "https://static-cdn.jtvnw.net/badges/v1/743a0f3b-84b3-450b-96a0-503d7f4a9764/3",
                                "title": "cheer 1",
                                "description": "cheer 1",
                                "click_action": "visit_url",
                                "click_url": "https://bits.twitch.tv"
                            }
                        ]
                    },
                    {
                        "set_id": "subscriber",
                        "versions": [
                            {
                                "id": "0",
                                "image_url_1x": "https://static-cdn.jtvnw.net/badges/v1/eb4a8a4c-eacd-4f5e-b9f2-394348310442/1",
                                "image_url_2x": "https://static-cdn.jtvnw.net/badges/v1/eb4a8a4c-eacd-4f5e-b9f2-394348310442/2",
                                "image_url_4x": "https://static-cdn.jtvnw.net/badges/v1/eb4a8a4c-eacd-4f5e-b9f2-394348310442/3",
                                "title": "Subscriber",
                                "description": "Subscriber",
                                "click_action": "subscribe_to_channel",
                                "click_url": null
                            },
                        ]
                    }
                ]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn get_global_chat_badges(&self) {
        self.api_mock("GET", "/chat/badges/global")
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "set_id": "vip",
                        "versions": [
                            {
                                "id": "1",
                                "image_url_1x": "https://static-cdn.jtvnw.net/badges/v1/b817aba4-fad8-49e2-b88a-7cc744dfa6ec/1",
                                "image_url_2x": "https://static-cdn.jtvnw.net/badges/v1/b817aba4-fad8-49e2-b88a-7cc744dfa6ec/2",
                                "image_url_4x": "https://static-cdn.jtvnw.net/badges/v1/b817aba4-fad8-49e2-b88a-7cc744dfa6ec/3",
                                "title": "VIP",
                                "description": "VIP",
                                "click_action": "visit_url",
                                "click_url": "https://help.twitch.tv/customer/en/portal/articles/659115-twitch-chat-badges-guide"
                            }
                        ]
                    },
                ]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn get_chat_settings(&self) {
        self.api_mock("GET", "/chat/settings")
            .and(query_param("broadcaster_id", "1234"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "broadcaster_id": "713936733",
                        "slow_mode": false,
                        "slow_mode_wait_time": null,
                        "follower_mode": true,
                        "follower_mode_duration": 0,
                        "subscriber_mode": false,
                        "emote_mode": false,
                        "unique_chat_mode": false,
                        "non_moderator_chat_delay": true,
                        "non_moderator_chat_delay_duration": 4
                    }
                ]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn get_shared_chat_session(&self) {
        self.api_mock("GET", "/shared_chat/session")
            .and(query_param("broadcaster_id", "198704263"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "session_id": "359bce59-fa4e-41a5-bd6f-9bc0c8360485",
                        "host_broadcaster_id": "198704263",
                        "participants": [{
                            "broadcaster_id": "198704263"
                        }, {
                            "broadcaster_id": "487263401"
                        }],
                        "created_at": "2024-09-29T19:45:37Z",
                        "updated_at": "2024-09-29T19:45:37Z"
                    }
                ]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn get_user_emotes(&self) {
        self.api_mock("GET", "/chat/emotes/user")
            .and(query_param("user_id", "123456"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "emote_set_id": "",
                        "emote_type": "hypetrain",
                        "format": [
                            "static"
                        ],
                        "id": "304420818",
                        "name": "HypeLol",
                        "owner_id": "477339272",
                        "scale": [
                            "1.0",
                            "2.0",
                            "3.0"
                        ],
                        "theme_mode": [
                            "light",
                            "dark"
                        ]
                    }
                ],
                "template": "https://static-cdn.jtvnw.net/emoticons/v2/{{id}}/{{format}}/{{theme_mode}}/{{scale}}",
                "pagination": {
                    "cursor": "eyJiIjpudWxsLJxhIjoiIn0gf5"
                }
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn update_chat_settings(&self) {
        self.api_mock("PATCH", "/chat/settings")
            .and(query_param("broadcaster_id", "1234"))
            .and(query_param("moderator_id", "5678"))
            .and(header("content-type", "application/json"))
            .and(body_json(json!({
                "follower_mode": false,
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "broadcaster_id": "1234",
                        "moderator_id": "5678",
                        "slow_mode": true,
                        "slow_mode_wait_time": 10,
                        "follower_mode": false,
                        "follower_mode_duration": null,
                        "subscriber_mode": false,
                        "emote_mode": false,
                        "unique_chat_mode": false,
                        "non_moderator_chat_delay": false,
                        "non_moderator_chat_delay_duration": null
                    }
                ]
            })))
            .mount(&self.server)
            .await;
    }
    pub async fn update_chat_settings2(&self) {
        self.api_mock("PATCH", "/chat/settings")
            .and(query_param("broadcaster_id", "1234"))
            .and(query_param("moderator_id", "5678"))
            .and(header("content-type", "application/json"))
            .and(body_json(json!({
                "slow_mode": true,
                "slow_mode_wait_time": 10
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "broadcaster_id": "1234",
                        "moderator_id": "5678",
                        "slow_mode": true,
                        "slow_mode_wait_time": 10,
                        "follower_mode": false,
                        "follower_mode_duration": null,
                        "subscriber_mode": false,
                        "emote_mode": false,
                        "unique_chat_mode": false,
                        "non_moderator_chat_delay": false,
                        "non_moderator_chat_delay_duration": null
                    }
                ]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn send_chat_announcement(&self) {
        self.api_mock("POST", "/chat/announcements")
            .and(query_param("broadcaster_id", "11111"))
            .and(query_param("moderator_id", "44444"))
            .and(header("content-type", "application/json"))
            .and(body_json(json!({
                "message":"Hello chat!",
                "color":"purple"
            })))
            .respond_with(ResponseTemplate::new(204))
            .mount(&self.server)
            .await;
    }

    pub async fn send_a_shoutout(&self) {
        self.api_mock("POST", "/chat/shoutouts")
            .and(query_param("from_broadcaster_id", "12345"))
            .and(query_param("to_broadcaster_id", "626262"))
            .and(query_param("moderator_id", "98765"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&self.server)
            .await;
    }

    pub async fn send_chat_message(&self) {
        self.api_mock("POST", "/chat/messages")
            .and(header("content-type", "application/json"))
            .and(body_json(json!({
                "broadcaster_id": "12826",
                "sender_id": "141981764",
                "message": "Hello, world! twitchdevHype",
                "for_source_only": true
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "message_id": "abc-123-def",
                        "is_sent": true
                    }
                ]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn get_user_chat_color(&self) {
        self.api_mock("GET", "/chat/color")
            .and(query_param("user_id", "11111"))
            .and(query_param("user_id", "44444"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "user_id": "11111",
                        "user_name": "SpeedySpeedster1",
                        "user_login": "speedyspeedster1",
                        "color": "#9146FF"
                    },
                    {
                        "user_id": "44444",
                        "user_name": "SpeedySpeedster2",
                        "user_login": "speedyspeedster2",
                        "color": ""
                    }
                ]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn update_user_chat_color(&self) {
        self.api_mock("PUT", "/chat/color")
            .and(query_param("user_id", "123"))
            .and(query_param("color", "blue"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&self.server)
            .await;
    }
    // pub async fn update_user_chat_color2(&self) {
    //     self.api_mock("PUT", "/chat/color")
    //         .and(query_param("user_id", "123"))
    //         .and(query_param("color", "%239146FF"))
    //         .respond_with(ResponseTemplate::new(204))
    //         .mount(&self.server)
    //         .await;
    // }
}

#[cfg(feature = "clips")]
impl TwitchApiTest {
    pub async fn create_clip(&self) {
        self.api_mock("POST", "/clips")
            .and(query_param("broadcaster_id", "44322889"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [{
                    "id": "FiveWordsUppercase",
                    "edit_url": "https://clips.twitch.tv/FiveWordsUppercase/edit"
                }]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn get_clips(&self) {
        self.api_mock("GET", "/clips")
            .and(query_param("id", "AwkwardHelplessSalamanderSwiftRage"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "id": "AwkwardHelplessSalamanderSwiftRage",
                        "url": "https://clips.twitch.tv/AwkwardHelplessSalamanderSwiftRage",
                        "embed_url": "https://clips.twitch.tv/embed?clip=AwkwardHelplessSalamanderSwiftRage",
                        "broadcaster_id": "67955580",
                        "broadcaster_name": "ChewieMelodies",
                        "creator_id": "53834192",
                        "creator_name": "BlackNova03",
                        "video_id": "205586603",
                        "game_id": "488191",
                        "language": "en",
                        "title": "babymetal",
                        "view_count": 10,
                        "created_at": "2017-11-30T22:34:18Z",
                        "thumbnail_url": "https://clips-media-assets.twitch.tv/157589949-preview-480x272.jpg",
                        "duration": 60,
                        "vod_offset": 480,
                        "is_featured": false
                    }
                ]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn get_clips2(&self) {
        self.api_mock("GET", "/clips")
            .and(query_param("broadcaster_id", "1234"))
            .and(query_param("first", "5"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "id": "RandomClip1",
                        "url": "https://clips.twitch.tv/AwkwardHelplessSalamanderSwiftRage",
                        "embed_url": "https://clips.twitch.tv/embed?clip=RandomClip1",
                        "broadcaster_id": "1234",
                        "broadcaster_name": "JJ",
                        "creator_id": "123456",
                        "creator_name": "MrMarshall",
                        "video_id": "",
                        "game_id": "33103",
                        "language": "en",
                        "title": "random1",
                        "view_count": 10,
                        "created_at": "2017-11-30T22:34:18Z",
                        "thumbnail_url": "https://clips-media-assets.twitch.tv/157589949-preview-480x272.jpg",
                        "duration": 12.9,
                        "vod_offset": 1957,
                        "is_featured": true
                    },
                ],
                "pagination": {
                    "cursor": "eyJiIjpudWxsLCJhIjoiIn0"
                }
            })))
            .mount(&self.server)
            .await;
    }
}

#[cfg(feature = "ccls")]
impl TwitchApiTest {
    pub async fn get_content_classification_labels(&self) {
        self.api_mock("GET", "/content_classification_labels")
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "id": "DebatedSocialIssuesAndPolitics",
                        "description": "Discussions or debates about politics or sensitive social issues such as elections, civic integrity, military conflict, and civil rights in a polarizing manner.",
                        "name": "Politics and Sensitive Social Issues"
                    },
                    {
                        "id": "DrugsIntoxication",
                        "description": "Excessive tobacco glorification or promotion, any marijuana consumption/use, legal drug and alcohol induced intoxication, discussions of illegal drugs.",
                        "name": "Drugs, Intoxication, or Excessive Tobacco Use"
                    },
                    {
                        "id": "Gambling",
                        "description": "Participating in online or in-person gambling, poker or fantasy sports, that involve the exchange of real money.",
                        "name": "Gambling"
                    },
                    {
                        "id": "MatureGame",
                        "description": "Games that are rated Mature or less suitable for a younger audience.",
                        "name": "Mature-rated game"
                    },
                    {
                        "id": "ProfanityVulgarity",
                        "description": "Prolonged, and repeated use of obscenities, profanities, and vulgarities, especially as a regular part of speech.",
                        "name": "Significant Profanity or Vulgarity"
                    },
                    {
                        "id": "SexualThemes",
                        "description": "Content that focuses on sexualized physical attributes and activities, sexual topics, or experiences.",
                        "name": "Sexual Themes"
                    },
                    {
                        "id": "ViolentGraphic",
                        "description": "Simulations and/or depictions of realistic violence, gore, extreme injury, or death.",
                        "name": "Violent and Graphic Depictions"
                    }
                ]
            })))
            .mount(&self.server)
            .await;
    }
}

#[cfg(feature = "entitlements")]
impl TwitchApiTest {
    pub async fn get_drops_entitlements(&self) {
        self.api_mock("GET", "/entitlements/drops")
            .and(query_param("user_id", "25009227"))
            .and(query_param("game_id", "33214"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "id": "fb78259e-fb81-4d1b-8333-34a06ffc24c0",
                        "benefit_id": "74c52265-e214-48a6-91b9-23b6014e8041",
                        "timestamp": "2019-01-28T04:17:53.325Z",
                        "user_id": "25009227",
                        "game_id": "33214",
                        "fulfillment_status": "CLAIMED",
                        "last_updated": "2019-01-28T04:17:53.325Z"
                    },
                    {
                        "id": "862750a5-265e-4ab6-9f0a-c64df3d54dd0",
                        "benefit_id": "74c52265-e214-48a6-91b9-23b6014e8041",
                        "timestamp": "2019-01-28T04:16:53.325Z",
                        "user_id": "25009227",
                        "game_id": "33214",
                        "fulfillment_status": "CLAIMED",
                        "last_updated": "2021-06-15T04:16:53.325Z"
                    },
                    {
                        "id": "d8879baa-3966-4d10-8856-15fdd62cce02",
                        "benefit_id": "cdfdc5c3-65a2-43bc-8767-fde06eb4ab2c",
                        "timestamp": "2019-01-28T04:15:53.325Z",
                        "user_id": "25009227",
                        "game_id": "33214",
                        "fulfillment_status": "FULFILLED",
                        "last_updated": "2019-01-28T04:17:53.325Z"
                    },
                ],
                "pagination": {
                    "cursor": "eyJiIjpudW..."
                }
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn update_drops_entitlements(&self) {
        self.api_mock("PATCH", "/entitlements/drops")
            .and(header("content-type", "application/json"))
            .and(body_json(json!({
                "fulfillment_status": "FULFILLED",
                "entitlement_ids": [
                    "fb78259e-fb81-4d1b-8333-34a06ffc24c0",
                    "862750a5-265e-4ab6-9f0a-c64df3d54dd0",
                    "d8879baa-3966-4d10-8856-15fdd62cce02",
                    "9a290126-7e3b-4f66-a9ae-551537893b65"
                ]
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "status": "SUCCESS",
                        "ids": [
                            "fb78259e-fb81-4d1b-8333-34a06ffc24c0", "862750a5-265e-4ab6-9f0a-c64df3d54dd0"
                        ]
                    },
                    {
                        "status": "UNAUTHORIZED",
                        "ids": [
                            "d8879baa-3966-4d10-8856-15fdd62cce02"
                        ]
                    },
                    {
                        "status": "UPDATE_FAILED",
                        "ids": [
                            "9a290126-7e3b-4f66-a9ae-551537893b65"
                        ]
                    }
                ]
            })))
            .mount(&self.server)
            .await;
    }
}

#[cfg(feature = "extensions")]
impl TwitchApiTest {
    pub async fn get_extension_configuration_segment(&self) {
        self.basic_mock("GET", "/extensions/configurations")
            .and(query_param(
                "extension_id",
                "uo6dggojyb8d6soh92zknwmi5ej1q2",
            ))
            .and(query_param("segment", "global"))
            .and(header("authorization", "Bearer test_jwt_token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "segment": "global",
                        "content": "hello config!",
                        "version": "0.0.1"
                    }
                ]
            })))
            .mount(&self.server)
            .await;
    }
    pub async fn get_extension_configuration_segment2(&self) {
        self.basic_mock("GET", "/extensions/configurations")
            .and(query_param(
                "extension_id",
                "uo6dggojyb8d6soh92zknwmi5ej1q2",
            ))
            .and(query_param("segment", "global"))
            .and(header("authorization", "Bearer test_jwt_token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "segment": "global",
                        "content": "{\"foo\":\"bar\"}",
                        "version": "0.0.1"
                    }
                ]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn set_extension_configuration_segment(&self) {
        self.api_mock("PUT", "/extensions/configurations")
            .and(header("content-type", "application/json"))
            .and(body_json(json!({
                "extension_id": "uo6dggojyb8d6soh92zknwmi5ej1q2",
                "segment": "global",
                "version": "0.0.1",
                "content": "hello config!"
            })))
            .respond_with(ResponseTemplate::new(204))
            .mount(&self.server)
            .await;
    }

    pub async fn set_extension_required_configuration(&self) {
        self.api_mock("PUT", "/extensions/required_configuration")
            .and(query_param("broadcaster_id", "274637212"))
            .and(header("content-type", "application/json"))
            .and(body_json(json!({
                "required_configuration": "RCS",
                "extension_id": "uo6dggojyb8d6soh92zknwmi5ej1q2",
                "extension_version": "0.0.1"
            })))
            .respond_with(ResponseTemplate::new(204))
            .mount(&self.server)
            .await;
    }

    pub async fn send_extension_pubsub_message(&self) {
        self.api_mock("POST", "/extensions/pubsub")
            .and(header("content-type", "application/json"))
            .and(body_json(json!({
                 "message": "hello world!",
                "broadcaster_id": "141981764",
                "target": ["broadcast"]
            })))
            .respond_with(ResponseTemplate::new(204))
            .mount(&self.server)
            .await;
    }

    pub async fn get_extension_live_channels(&self) {
        self.api_mock("GET", "/extensions/live")
            .and(query_param(
                "extension_id",
                "uo6dggojyb8d6soh92zknwmi5ej1q2",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "broadcaster_id": "252766116",
                        "broadcaster_name": "swoosh_xii",
                        "game_name": "Tom Clancy's Rainbow Six Siege",
                        "game_id": "460630",
                        "title": "[PS4] ITA/ENG UNRANKED CHILLIN' (SUB 1/15) - !instagram !donation !sens !team !youtube"
                    },
                    {
                        "broadcaster_id": "264525686",
                        "broadcaster_name": "gaddem_",
                        "game_name": "For Honor",
                        "game_id": "490382",
                        "title": "any Kätzchen ? - 680 Rep + > Kompetitive Kitten"
                    },
                    {
                        "broadcaster_id": "264787895",
                        "broadcaster_name": "LenhadorGameplay",
                        "game_name": "For Honor",
                        "game_id": "490382",
                        "title": "Vazou o novo personagem! *Triste*"
                    }
                ],
                "pagination": "YVc1emRHRnNiQ015TmpJek5qazVOVHBoYWpKbGRIZDFaR0Z5YjNCMGN6UTJNMkZ1TUdwM2FHWnBZbm8yYW5rNjoy"
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn get_extension_secrets(&self) {
        self.api_mock("GET", "/extensions/jwt/secrets")
            .and(query_param(
                "extension_id",
                "uo6dggojyb8d6soh92zknwmi5ej1q2",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [{
                    "format_version": 1,
                    "secrets": [
                        {
                        "content": "secret",
                        "active_at": "2021-03-29T06:58:40.858343036Z",
                        "expires_at": "2121-03-05T06:58:40.858343036Z"
                        }
                    ]
                }]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn create_extension_secret(&self) {
        self.api_mock("POST", "/extensions/jwt/secrets")
            .and(query_param(
                "extension_id",
                "uo6dggojyb8d6soh92zknwmi5ej1q2",
            ))
            .and(query_param("delay", "600"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [{
                    "format_version": 1,
                        "secrets": [
                            {
                                "content": "old-secret",
                                "active_at": "2021-03-29T06:58:40.858343036Z",
                                "expires_at": "2021-04-22T05:21:54.99261682Z"
                            },
                            {
                                "content": "new-secret",
                                "active_at": "2021-04-22T04:16:54.996365329Z",
                                "expires_at": "2121-03-29T04:16:54.996365329Z"
                            }
                        ]
                }]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn send_extension_chat_message(&self) {
        self.api_mock("POST", "/extensions/chat")
            .and(query_param("broadcaster_id", "237757755"))
            .and(header("content-type", "application/json"))
            .and(body_json(json!({
                "text": "Hello",
                "extension_id": "uo6dggojyb8d6soh92zknwmi5ej1q2",
                "extension_version": "0.0.9"
            })))
            .respond_with(ResponseTemplate::new(204))
            .mount(&self.server)
            .await;
    }

    pub async fn get_extensions(&self) {
        self.api_mock("GET", "/extensions")
            .and(query_param(
                "extension_id",
                "uo6dggojyb8d6soh92zknwmi5ej1q2",
            ))
            .and(query_param("extension_version", "0.0.9"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "author_name": "Twitch Developers",
                        "bits_enabled": true,
                        "can_install": false,
                        "configuration_location": "hosted",
                        "description": "An extension for testing all the features that we add to extensions",
                        "eula_tos_url": "",
                        "has_chat_support": true,
                        "icon_url": "https://extensions-discovery-images.twitch.tv/pgn0bjv51epi7eaekt53tovjnc82qo/0.0.8/logob6c995d8-8b45-48cc-a748-b256e92ac1cd",
                        "icon_urls": {
                            "100x100": "https://extensions-discovery-images.twitch.tv/pgn0bjv51epi7eaekt53tovjnc82qo/0.0.8/logob6c995d8-8b45-48cc-a748-b256e92ac1cd",
                            "24x24": "https://extensions-discovery-images.twitch.tv/pgn0bjv51epi7eaekt53tovjnc82qo/0.0.8/taskbar905b19da-e7e5-4d8f-beb7-f543a861ac1e",
                            "300x200": "https://extensions-discovery-images.twitch.tv/pgn0bjv51epi7eaekt53tovjnc82qo/0.0.8/discoveryd9545b2c-5474-46d7-a523-1c3835d862ce"
                        },
                        "id": "pgn0bjv51epi7eaekt53tovjnc82qo",
                        "name": "Official Developers Demo",
                        "privacy_policy_url": "",
                        "request_identity_link": true,
                        "screenshot_urls": [
                            "https://extensions-discovery-images.twitch.tv/pgn0bjv51epi7eaekt53tovjnc82qo/0.0.8/screenshotbdec475d-3d2f-4378-b334-941dfddc897a"
                        ],
                        "state": "Released",
                        "subscriptions_support_level": "optional",
                        "summary": "Test ALL the extensions features!",
                        "support_email": "dx-extensions-test-dev@justin.tv",
                        "version": "0.0.9",
                        "viewer_summary": "Test ALL the extensions features!",
                        "views": {
                            "mobile": {
                                "viewer_url": "https://pgn0bjv51epi7eaekt53tovjnc82qo.ext-twitch.tv/pgn0bjv51epi7eaekt53tovjnc82qo/0.0.9/f9a0d8aae0f9dd0b2d6ef3416b96bc79/index.html"
                            },
                            "panel": {
                                "viewer_url": "https://pgn0bjv51epi7eaekt53tovjnc82qo.ext-twitch.tv/pgn0bjv51epi7eaekt53tovjnc82qo/0.0.9/f9a0d8aae0f9dd0b2d6ef3416b96bc79/index.html",
                                "height": 300,
                                "can_link_external_content": false
                            },
                            "video_overlay": {
                                "viewer_url": "https://pgn0bjv51epi7eaekt53tovjnc82qo.ext-twitch.tv/pgn0bjv51epi7eaekt53tovjnc82qo/0.0.9/f9a0d8aae0f9dd0b2d6ef3416b96bc79/index.html",
                                "can_link_external_content": false
                            },
                            "component": {
                                "viewer_url": "https://pgn0bjv51epi7eaekt53tovjnc82qo.ext-twitch.tv/pgn0bjv51epi7eaekt53tovjnc82qo/0.0.9/f9a0d8aae0f9dd0b2d6ef3416b96bc79/index.html",
                                "aspect_width": 0,
                                "aspect_height": 0,
                                "aspect_ratio_x": 48000,
                                "aspect_ratio_y": 36000,
                                "autoscale": true,
                                "scale_pixels": 1024,
                                "target_height": 5333,
                                "size": 0,
                                "zoom": false,
                                "zoom_pixels": 0,
                                "can_link_external_content": false
                            }
                        },
                        "allowlisted_config_urls": [],
                        "allowlisted_panel_urls": []
                    }
                ]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn get_released_extensions(&self) {
        self.api_mock("GET", "/extensions/released")
            .and(query_param(
                "extension_id",
                "uo6dggojyb8d6soh92zknwmi5ej1q2",
            ))
            .and(query_param("extension_version", "0.0.9"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "author_name": "Twitch Developer Experience",
                        "bits_enabled": true,
                        "can_install": false,
                        "configuration_location": "hosted",
                        "description": "An extension for testing all the features that we add to extensions",
                        "eula_tos_url": "",
                        "has_chat_support": true,
                        "icon_url": "https://extensions-discovery-images.twitch.tv/pgn0bjv51epi7eaekt53tovjnc82qo/0.0.8/logob6c995d8-8b45-48cc-a748-b256e92ac1cd",
                        "icon_urls": {
                            "100x100": "https://extensions-discovery-images.twitch.tv/pgn0bjv51epi7eaekt53tovjnc82qo/0.0.8/logob6c995d8-8b45-48cc-a748-b256e92ac1cd",
                            "24x24": "https://extensions-discovery-images.twitch.tv/pgn0bjv51epi7eaekt53tovjnc82qo/0.0.8/taskbar905b19da-e7e5-4d8f-beb7-f543a861ac1e",
                            "300x200": "https://extensions-discovery-images.twitch.tv/pgn0bjv51epi7eaekt53tovjnc82qo/0.0.8/discoveryd9545b2c-5474-46d7-a523-1c3835d862ce"
                        },
                        "id": "pgn0bjv51epi7eaekt53tovjnc82qo",
                        "name": "Official Developer Experience Demo",
                        "privacy_policy_url": "",
                        "request_identity_link": true,
                        "screenshot_urls": [
                            "https://extensions-discovery-images.twitch.tv/pgn0bjv51epi7eaekt53tovjnc82qo/0.0.8/screenshotbdec475d-3d2f-4378-b334-941dfddc897a"
                        ],
                        "state": "Released",
                        "subscriptions_support_level": "optional",
                        "summary": "Test ALL the extensions features!",
                        "support_email": "dx-extensions-test-dev@justin.tv",
                        "version": "0.0.9",
                        "viewer_summary": "Test ALL the extensions features!",
                        "views": {
                            "mobile": {
                                "viewer_url": "https://pgn0bjv51epi7eaekt53tovjnc82qo.ext-twitch.tv/pgn0bjv51epi7eaekt53tovjnc82qo/0.0.9/f9a0d8aae0f9dd0b2d6ef3416b96bc79/index.html"
                            },
                            "panel": {
                                "viewer_url": "https://pgn0bjv51epi7eaekt53tovjnc82qo.ext-twitch.tv/pgn0bjv51epi7eaekt53tovjnc82qo/0.0.9/f9a0d8aae0f9dd0b2d6ef3416b96bc79/index.html",
                                "height": 300,
                                "can_link_external_content": false
                            },
                            "video_overlay": {
                                "viewer_url": "https://pgn0bjv51epi7eaekt53tovjnc82qo.ext-twitch.tv/pgn0bjv51epi7eaekt53tovjnc82qo/0.0.9/f9a0d8aae0f9dd0b2d6ef3416b96bc79/index.html",
                                "can_link_external_content": false
                            },
                            "component": {
                                "viewer_url": "https://pgn0bjv51epi7eaekt53tovjnc82qo.ext-twitch.tv/pgn0bjv51epi7eaekt53tovjnc82qo/0.0.9/f9a0d8aae0f9dd0b2d6ef3416b96bc79/index.html",
                                "aspect_width": 0,
                                "aspect_height": 0,
                                "aspect_ratio_x": 48000,
                                "aspect_ratio_y": 36000,
                                "autoscale": true,
                                "scale_pixels": 1024,
                                "target_height": 5333,
                                "size": 0,
                                "zoom": false,
                                "zoom_pixels": 0,
                                "can_link_external_content": false
                            }
                        },
                        "allowlisted_config_urls": [],
                        "allowlisted_panel_urls": []
                    }
                ]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn get_extension_bits_products(&self) {
        self.api_mock("GET", "/bits/extensions")
            .and(query_param("should_include_all", "true"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "sku": "1010",
                        "cost": {
                            "amount": 990,
                            "type": "bits"
                        },
                        "in_development": true,
                        "display_name": "Rusty Crate 2",
                        "expiration": "2021-05-18T09:10:13.397Z",
                        "is_broadcast": false
                    }
                ]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn update_extension_bits_product(&self) {
        self.api_mock("PUT", "/bits/extensions")
            .and(header("content-type", "application/json"))
            .and(body_json(json!({
                "sku": "1010",
                "cost": {
                    "amount": 990,
                    "type": "bits"
                },
                "in_development": true,
                "display_name": "Rusty Crate 2",
                "is_broadcast": true,
                "expiration": "2021-05-18T09:10:13.397Z"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "sku": "1010",
                        "cost": {
                            "amount": 990,
                            "type": "bits"
                        },
                        "in_development": true,
                        "display_name": "Rusty Crate 2",
                        "expiration": "2021-05-18T09:10:13.397Z",
                        "is_broadcast": true
                    }
                ]
            })))
            .mount(&self.server)
            .await;
    }
}
#[cfg(feature = "eventsub")]
impl TwitchApiTest {
    pub async fn create_eventsub(&self) {
        self.api_mock("POST", "/eventsub/subscriptions")
            .and(header("content-type", "application/json"))
            .and(body_json(json!({
                "type": "user.update",
                "version": "1",
                "condition": {
                    "user_id": "1234"
                },
                "transport": {
                    "method": "webhook",
                    "callback": "https://this-is-a-callback.com/",
                    "secret":"s3cre7"
                }
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                 "data": [
                    {
                        "id": "26b1c993-bfcf-44d9-b876-379dacafe75a",
                        "status": "webhook_callback_verification_pending",
                        "type": "user.update",
                        "version": "1",
                        "condition": {
                            "user_id": "1234"
                        },
                        "created_at": "2020-11-10T14:32:18.730260295Z",
                        "transport": {
                            "method": "webhook",
                            "callback": "https://this-is-a-callback.com"
                        },
                        "cost": 1
                    }
                ],
                "total": 1,
                "total_cost": 1,
                "max_total_cost": 10000
            })))
            .mount(&self.server)
            .await
    }
    pub async fn create_eventsub2(&self) {
        self.api_mock("POST", "/eventsub/subscriptions")
            .and(header("content-type", "application/json"))
            .and(body_json(json!({
                "type": "user.update",
                "version": "1",
                "condition": {
                    "user_id": "1234"
                },
                "transport": {
                    "method": "websocket",
                    "session_id": "AQoQexAWVYKSTIu4ec_2VAxyuhAB"
                }
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "id": "26b1c993-bfcf-44d9-b876-379dacafe75a",
                        "status": "webhook_callback_verification_pending",
                        "type": "user.update",
                        "version": "1",
                        "condition": {
                            "user_id": "1234"
                        },
                        "created_at": "2020-11-10T14:32:18.730260295Z",
                        "transport": {
                            "method": "webhook",
                            "callback": "https://this-is-a-callback.com"
                        },
                        "cost": 1
                    }
                ],
                "total": 1,
                "total_cost": 1,
                "max_total_cost": 10000
            })))
            .mount(&self.server)
            .await
    }
    pub async fn create_eventsub3(&self) {
        self.api_mock("POST", "/eventsub/subscriptions")
            .and(header("content-type", "application/json"))
            .and(body_json(json!({
                "type": "user.update",
                "version": "1",
                "condition": {
                    "user_id": "1234"
                },
                "transport": {
                    "method":"conduit",
                    "conduit_id":"bfcfc993-26b1-b876-44d9-afe75a379dac"
                }
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                        {
                            "id": "26b1c993-bfcf-44d9-b876-379dacafe75a",
                            "status": "enabled",
                            "type": "user.update",
                            "version": "1",
                            "condition": {
                                "user_id": "1234"
                            },
                            "created_at": "2020-11-10T14:32:18.730260295Z",
                            "transport": {
                                "method": "conduit",
                                "conduit_id": "bfcfc993-26b1-b876-44d9-afe75a379dac"
                            },
                            "cost": 1
                        }
                ],
                "total": 1,
                "total_cost": 1,
                "max_total_cost": 10000
            })))
            .mount(&self.server)
            .await
    }

    pub async fn delete_eventsub(&self) {
        self.api_mock("DELETE", "/eventsub/subscriptions")
            .and(query_param("id", "26b1c993-bfcf-44d9-b876-379dacafe75a"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&self.server)
            .await
    }

    pub async fn get_eventsub(&self) {
        self.api_mock("GET", "/eventsub/subscriptions")
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "total": 2,
                "data": [
                    {
                        "id": "26b1c993-bfcf-44d9-b876-379dacafe75a",
                        "status": "enabled",
                        "type": "stream.online",
                        "version": "1",
                        "condition": {
                            "broadcaster_user_id": "1234"
                        },
                        "created_at": "2020-11-10T20:08:33.12345678Z",
                        "transport": {
                            "method": "webhook",
                            "callback": "https://this-is-a-callback.com"
                        },
                        "cost": 1
                    },
                    {
                        "id": "35016908-41ff-33ce-7879-61b8dfc2ee16",
                        "status": "webhook_callback_verification_pending",
                        "type": "user.update",
                        "version": "1",
                        "condition": {
                            "user_id": "1234"
                        },
                        "created_at": "2020-11-10T14:32:18.730260295Z",
                        "transport": {
                            "method": "webhook",
                            "callback": "https://this-is-a-callback.com"
                        },
                        "cost": 0
                    }
                ],
                "total_cost": 1,
                "max_total_cost": 10000,
                "pagination": {}
            })))
            .mount(&self.server)
            .await
    }
}

#[cfg(feature = "games")]
impl TwitchApiTest {
    pub async fn get_top_games(&self) {
        self.api_mock("GET", "/games/top")
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "id": "493057",
                        "name": "PUBG: BATTLEGROUNDS",
                        "box_art_url": "https://static-cdn.jtvnw.net/ttv-boxart/493057-{width}x{height}.jpg",
                        "igdb_id": "27789"
                    },
                ],
                "pagination":{"cursor":"eyJiIjpudWxsLCJhIjp7Ik9mZnNldCI6MjB9fQ=="}
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn get_games(&self) {
        self.api_mock("GET", "/games")
            .and(query_param("id", "33214"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "id": "33214",
                        "name": "Fortnite",
                        "box_art_url": "https://static-cdn.jtvnw.net/ttv-boxart/33214-{width}x{height}.jpg",
                        "igdb_id": "1905"
                    }
                ]
            })))
            .mount(&self.server)
            .await;
    }
}

#[cfg(feature = "goals")]
impl TwitchApiTest {
    pub async fn get_creator_goals(&self) {
        self.api_mock("GET", "/goals")
            .and(query_param("broadcaster_id", "141981764"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "id": "1woowvbkiNv8BRxEWSqmQz6Zk92",
                        "broadcaster_id": "141981764",
                        "broadcaster_name": "TwitchDev",
                        "broadcaster_login": "twitchdev",
                        "type": "follower",
                        "description": "Follow goal for Helix testing",
                        "current_amount": 27062,
                        "target_amount": 30000,
                        "created_at": "2021-08-16T17:22:23Z"
                    }
                ]
            })))
            .mount(&self.server)
            .await;
    }
}

#[cfg(feature = "guest-star")]
impl TwitchApiTest {
    pub async fn get_channel_guest_star_settings(&self) {
        self.api_mock("GET", "/guest_star/channel_settings")
            .and(query_param("broadcaster_id", "932104"))
            .and(query_param("moderator_id", "9321049"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "is_moderator_send_live_enabled": true,
                    }
                ]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn update_channel_guest_star_settings(&self) {
        self.api_mock("PUT", "/guest_star/channel_settings")
            .and(query_param("broadcaster_id", "9321049"))
            .and(query_param("group_layout", "TILED_LAYOUT"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&self.server)
            .await;
    }
    pub async fn update_channel_guest_star_settings2(&self) {
        self.api_mock("PUT", "/guest_star/channel_settings")
            .and(query_param("broadcaster_id", "9321049"))
            .and(query_param("is_moderator_send_live_enabled", "false"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&self.server)
            .await;
    }
    pub async fn update_channel_guest_star_settings3(&self) {
        self.api_mock("PUT", "/guest_star/channel_settings")
            .and(query_param("broadcaster_id", "9321049"))
            .and(query_param("slot_count", "6"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&self.server)
            .await;
    }
    pub async fn update_channel_guest_star_settings4(&self) {
        self.api_mock("PUT", "/guest_star/channel_settings")
            .and(query_param("broadcaster_id", "9321049"))
            .and(query_param("regenerate_browser_sources", "true"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&self.server)
            .await;
    }

    pub async fn get_guest_star_session(&self) {
        self.api_mock("GET", "/guest_star/session")
            .and(query_param("broadcaster_id", "9321049"))
            .and(query_param("moderator_id", "9321049"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "id": "2KFRQbFtpmfyD3IevNRnCzOPRJI",
                        "guests": [
                            {
                                "slot_id": "0",
                                "user_id": "9321049",
                                "user_display_name": "Cool_User",
                                "user_login": "cool_user",
                                "is_live": true,
                                "volume": 100,
                                "assigned_at": "2023-01-02T04:16:53.325Z",
                                "audio_settings": {
                                    "is_available": true,
                                    "is_host_enabled": true,
                                    "is_guest_enabled": true,
                                },
                                "video_settings": {
                                    "is_available": true,
                                    "is_host_enabled": true,
                                    "is_guest_enabled": true,
                                }
                            },
                            {
                                "slot_id": "1",
                                "user_id": "144601104",
                                "user_display_name": "Cool_Guest",
                                "user_login": "cool_guest",
                                "is_live": true,
                                "volume": 100,
                                "assigned_at": "2023-01-02T04:20:59.325Z",
                                "audio_settings": {
                                    "is_available": true,
                                    "is_host_enabled": true,
                                    "is_guest_enabled": true,
                                },
                                "video_settings": {
                                    "is_available": true,
                                    "is_host_enabled": true,
                                    "is_guest_enabled": true,
                                }
                            }
                        ]
                    }
                ]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn create_guest_star_session(&self) {
        self.api_mock("POST", "/guest_star/session")
            .and(query_param("broadcaster_id", "9321049"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "id": "2KFRQbFtpmfyD3IevNRnCzOPRJI",
                        "guests": [
                            {
                                "id": "0",
                                "user_id": "9321049",
                                "user_display_name": "Cool_User",
                                "user_login": "cool_user",
                                "is_live": true,
                                "volume": 100,
                                "assigned_at": "2023-01-02T04:16:53.325Z",
                                "audio_settings": {
                                    "is_available": true,
                                    "is_host_enabled": true,
                                    "is_guest_enabled": true,
                                },
                                "video_settings": {
                                    "is_available": true,
                                    "is_host_enabled": true,
                                    "is_guest_enabled": true,
                                }
                            },
                        ]
                    },
                ]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn end_guest_star_session(&self) {
        self.api_mock("DELETE", "/guest_star/session")
            .and(query_param("broadcaster_id", "9321049"))
            .and(query_param("session_id", "2KFRQbFtpmfyD3IevNRnCzOPRJI"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "id": "2KFRQbFtpmfyD3IevNRnCzOPRJI",
                        "guests": [
                            {
                                "id": "0",
                                "user_id": "9321049",
                                "user_display_name": "Cool_User",
                                "user_login": "cool_user",
                                "is_live": true,
                                "volume": 100,
                                "assigned_at": "2023-01-02T04:16:53.325Z",
                                "audio_settings": {
                                    "is_available": true,
                                    "is_host_enabled": true,
                                    "is_guest_enabled": true,
                                },
                                "video_settings": {
                                    "is_available": true,
                                    "is_host_enabled": true,
                                    "is_guest_enabled": true,
                                }
                            },
                        ]
                    }
                ]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn get_guest_star_invites(&self) {
        self.api_mock("GET", "/guest_star/invites")
            .and(query_param("broadcaster_id", "9321049"))
            .and(query_param("moderator_id", "9321049"))
            .and(query_param("session_id", "2KFRQbFtpmfyD3IevNRnCzOPRJI"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "user_id": "144601104",
                        "invited_at": "2023-01-02T04:16:53.325Z",
                        "status": "INVITED",
                        "is_audio_enabled": false,
                        "is_video_enabled": true,
                        "is_audio_available": true,
                        "is_video_available": true
                    }
                ]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn send_guest_star_invite(&self) {
        self.api_mock("POST", "/guest_star/invites")
            .and(query_param("broadcaster_id", "9321049"))
            .and(query_param("moderator_id", "9321049"))
            .and(query_param("session_id", "2KFRQbFtpmfyD3IevNRnCzOPRJI"))
            .and(query_param("guest_id", "144601104"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&self.server)
            .await;
    }

    pub async fn delete_guest_star_invite(&self) {
        self.api_mock("DELETE", "/guest_star/invites")
            .and(query_param("broadcaster_id", "9321049"))
            .and(query_param("moderator_id", "9321049"))
            .and(query_param("session_id", "2KFRQbFtpmfyD3IevNRnCzOPRJI"))
            .and(query_param("guest_id", "144601104"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&self.server)
            .await;
    }

    pub async fn assign_guest_star_slot(&self) {
        self.api_mock("POST", "/guest_star/slot")
            .and(query_param("broadcaster_id", "9321049"))
            .and(query_param("moderator_id", "9321049"))
            .and(query_param("session_id", "2KFRQbFtpmfyD3IevNRnCzOPRJI"))
            .and(query_param("guest_id", "144601104"))
            .and(query_param("slot_id", "1"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&self.server)
            .await;
    }

    pub async fn update_guest_star_slot(&self) {
        self.api_mock("PATCH", "/guest_star/slot")
            .and(query_param("broadcaster_id", "9321049"))
            .and(query_param("moderator_id", "9321049"))
            .and(query_param("session_id", "2KFRQbFtpmfyD3IevNRnCzOPRJI"))
            .and(query_param("source_slot_id", "1"))
            .and(query_param("destination_slot_id", "2"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&self.server)
            .await;
    }

    pub async fn delete_guest_star_slot(&self) {
        self.api_mock("DELETE", "/guest_star/slot")
            .and(query_param("broadcaster_id", "9321049"))
            .and(query_param("moderator_id", "9321049"))
            .and(query_param("session_id", "2KFRQbFtpmfyD3IevNRnCzOPRJI"))
            .and(query_param("guest_id", "144601104"))
            .and(query_param("slot_id", "1"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&self.server)
            .await;
    }

    pub async fn update_guest_star_slot_settings(&self) {
        self.api_mock("PATCH", "/guest_star/slot_settings")
            .and(query_param("broadcaster_id", "9321049"))
            .and(query_param("moderator_id", "9321049"))
            .and(query_param("session_id", "2KFRQbFtpmfyD3IevNRnCzOPRJI"))
            .and(query_param("slot_id", "1"))
            .and(query_param("is_audio_enabled", "false"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&self.server)
            .await;
    }
    pub async fn update_guest_star_slot_settings2(&self) {
        self.api_mock("PATCH", "/guest_star/slot_settings")
            .and(query_param("broadcaster_id", "9321049"))
            .and(query_param("moderator_id", "9321049"))
            .and(query_param("session_id", "2KFRQbFtpmfyD3IevNRnCzOPRJI"))
            .and(query_param("slot_id", "1"))
            .and(query_param("is_live", "true"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&self.server)
            .await;
    }
}

#[cfg(feature = "hype-train")]
impl TwitchApiTest {
    pub async fn get_hype_train_events(&self) {
        self.api_mock("GET", "/hypetrain/events")
            .and(query_param("broadcaster_id", "270954519"))
            .and(query_param("first", "1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "id": "1b0AsbInCHZW2SQFQkCzqN07Ib2",
                        "event_type": "hypetrain.progression",
                        "event_timestamp": "2020-04-24T20:07:24Z",
                        "version": "1.0",
                        "event_data": {
                            "broadcaster_id": "270954519",
                            "cooldown_end_time": "2020-04-24T20:13:21.003802269Z",
                            "expires_at": "2020-04-24T20:12:21.003802269Z",
                            "goal": 1800,
                            "id": "70f0c7d8-ff60-4c50-b138-f3a352833b50",
                            "last_contribution": {
                                "total": 200,
                                "type": "BITS",
                                "user": "134247454"
                            },
                            "level": 2,
                            "started_at": "2020-04-24T20:05:47.30473127Z",
                            "top_contributions": [
                            {
                                "total": 600,
                                "type": "BITS",
                                "user": "134247450"
                            }
                            ],
                            "total": 600
                        }
                    }
                ],
                "pagination": {
                    "cursor": "eyJiIjpudWxsLCJhIjp7IkN1cnNvciI6IjI3MDk1NDUxOToxNTg3NzU4ODQ0OjFiMEFzYkluQ0haVzJTUUZRa0N6cU4wN0liMiJ9fQ"
                }
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn get_hype_train_status(&self) {
        self.api_mock("GET", "/hypetrain/status")
            .and(query_param("broadcaster_id", "123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "current": {
                            "id": "1b0AsbInCHZW2SQFQkCzqN07Ib2",
                            "broadcaster_user_id": "1337",
                            "broadcaster_user_login": "cool_user",
                            "broadcaster_user_name": "Cool_User",
                            "level": 2,
                            "total": 700,
                            "progress": 200,
                            "goal": 1000,
                            "top_contributions": [
                                {
                                    "user_id": "123",
                                    "user_login": "pogchamp",
                                    "user_name": "PogChamp",
                                    "type": "bits",
                                    "total": 50
                                },
                                {
                                    "user_id": "456",
                                    "user_login": "kappa",
                                    "user_name": "Kappa",
                                    "type": "subscription",
                                    "total": 45
                                }
                            ],
                            "shared_train_participants": [
                                {
                                    "broadcaster_user_id": "456",
                                    "broadcaster_user_login": "pogchamp",
                                    "broadcaster_user_name": "PogChamp"
                                },
                                {
                                    "broadcaster_user_id": "321",
                                    "broadcaster_user_login": "pogchamp",
                                    "broadcaster_user_name": "PogChamp"
                                }
                            ],
                            "started_at": "2020-07-15T17:16:03.17106713Z",
                            "expires_at": "2020-07-15T17:16:11.17106713Z",
                            "type": "golden_kappa"
                        },
                        "all_time_high": {
                            "level": 6,
                            "total": 2850,
                            "achieved_at": "2020-04-24T20:12:21.003802269Z"
                        },
                        "shared_all_time_high": {
                            "level": 16,
                            "total": 23850,
                            "achieved_at": "2020-04-27T20:12:21.003802269Z"
                        },
                    }
                ]
            })))
            .mount(&self.server)
            .await
    }
}

#[cfg(feature = "moderation")]
impl TwitchApiTest {
    pub async fn check_automod_status(&self) {
        self.api_mock("POST", "/moderation/enforcements/status")
            .and(query_param("broadcaster_id", "12345"))
            .and(header("content-type", "application/json"))
            .and(body_json(json!({
                  "data": [
                    {
                        "msg_id": "123",
                        "msg_text": "Hello World!"
                    },
                    {
                        "msg_id": "393",
                        "msg_text": "Boooooo!"
                    }
                ]
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [{
                    "msg_id": "123",
                    "is_permitted": true
                }, {
                    "msg_id": "393",
                    "is_permitted": false
                }]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn manage_held_automod_messages(&self) {
        self.api_mock("POST", "/moderation/automod/message")
            .and(header("content-type", "application/json"))
            .and(body_json(json!({
                "user_id": "9327994",
                "msg_id": "836013710",
                "action": "ALLOW"
            })))
            .respond_with(ResponseTemplate::new(204))
            .mount(&self.server)
            .await;
    }

    pub async fn get_automod_settings(&self) {
        self.api_mock("GET", "/moderation/automod/settings")
            .and(query_param("broadcaster_id", "1234"))
            .and(query_param("moderator_id", "5678"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "broadcaster_id": "1234",
                        "moderator_id": "5678",
                        "overall_level": null,
                        "disability": 0,
                        "aggression": 0,
                        "sexuality_sex_or_gender": 0,
                        "misogyny": 0,
                        "bullying": 0,
                        "swearing": 0,
                        "race_ethnicity_or_religion": 0,
                        "sex_based_terms": 0
                    }
                ]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn update_automod_settings(&self) {
        self.api_mock("PUT", "/moderation/automod/settings")
            .and(query_param("broadcaster_id", "1234"))
            .and(query_param("moderator_id", "5678"))
            .and(header("content-type", "application/json"))
            .and(body_json(json!({
                "overall_level": 3
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "broadcaster_id": "1234",
                        "moderator_id": "5678",
                        "overall_level": 3,
                        "disability": 3,
                        "aggression": 3,
                        "sexuality_sex_or_gender": 3,
                        "misogyny": 3,
                        "bullying": 2,
                        "swearing": 0,
                        "race_ethnicity_or_religion": 3,
                        "sex_based_terms": 3
                    }
                ]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn get_banned_users(&self) {
        self.api_mock("GET", "/moderation/banned")
            .and(query_param("broadcaster_id", "198704263"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "user_id": "423374343",
                        "user_login": "glowillig",
                        "user_name": "glowillig",
                        "expires_at": "2022-03-15T02:00:28Z",
                        "created_at": "2022-03-15T01:30:28Z",
                        "reason": "Does not like pineapple on pizza.",
                        "moderator_id": "141981764",
                        "moderator_login": "twitchdev",
                        "moderator_name": "TwitchDev"
                    },
                    {
                        "user_id": "424596340",
                        "user_login": "quotrok",
                        "user_name": "quotrok",
                        "expires_at": "2022-08-07T02:07:55Z",
                        "created_at": "2022-08-07T02:02:55Z",
                        "reason": "Inappropriate words.",
                        "moderator_id": "141981764",
                        "moderator_login": "twitchdev",
                        "moderator_name": "TwitchDev"
                    },
                ],
                "pagination": {
                    "cursor": "eyJiIjpudWxsLCJhIjp7IkN1cnNvciI6IjEwMDQ3MzA2NDo4NjQwNjU3MToxSVZCVDFKMnY5M1BTOXh3d1E0dUdXMkJOMFcifX0"
                }
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn ban_user(&self) {
        self.api_mock("POST", "/moderation/bans")
            .and(query_param("broadcaster_id", "1234"))
            .and(query_param("moderator_id", "5678"))
            .and(header("content-type", "application/json"))
            .and(body_json(json!({
                "data": {
                    "user_id": "9876",
                    "reason": "no reason"
                }
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "broadcaster_id": "1234",
                        "moderator_id": "5678",
                        "user_id": "9876",
                        "created_at": "2021-09-28T18:22:31Z",
                        "end_time": null
                    }
                ]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn ban_user2(&self) {
        self.api_mock("POST", "/moderation/bans")
            .and(query_param("broadcaster_id", "1234"))
            .and(query_param("moderator_id", "5678"))
            .and(header("content-type", "application/json"))
            .and(body_json(json!({
                "data": {
                    "user_id":"9876",
                    "duration":300,
                    "reason":"no reason"
                }
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "broadcaster_id": "1234",
                        "moderator_id": "5678",
                        "user_id": "9876",
                        "created_at": "2021-09-28T19:27:31Z",
                        "end_time": "2021-09-28T19:22:31Z"
                    }
                ]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn ban_user_error(&self) {
        self.api_mock("POST", "/moderation/bans")
            .and(query_param("broadcaster_id", "1234"))
            .and(query_param("moderator_id", "5678"))
            .and(header("content-type", "application/json"))
            .and(body_json(json!({
                "data": {
                    "user_id":"9876",
                    "duration":300,
                    "reason":"no reason"
                }
            })))
            .respond_with(ResponseTemplate::new(400).set_body_json(json!({
                "error": "Bad Request",
                "status": 400,
                "message": "user is already banned"
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn unban_user(&self) {
        self.api_mock("DELETE", "/moderation/bans")
            .and(query_param("broadcaster_id", "1234"))
            .and(query_param("moderator_id", "5678"))
            .and(query_param("user_id", "5432"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&self.server)
            .await;
    }

    pub async fn unban_user_error(&self) {
        self.api_mock("DELETE", "/moderation/bans")
            .and(query_param("broadcaster_id", "1234"))
            .and(query_param("moderator_id", "5678"))
            .and(query_param("user_id", "5432"))
            .respond_with(ResponseTemplate::new(400).set_body_json(json!({
                "error": "Bad Request",
                "status": 400,
                "message": "user is not banned"
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn get_unban_requests(&self) {
        self.api_mock("GET", "/moderation/unban_requests")
            .and(query_param("broadcaster_id", "274637212"))
            .and(query_param("moderator_id", "274637212"))
            .and(query_param("status", "pending"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "id": "92af127c-7326-4483-a52b-b0da0be61c01",
                        "broadcaster_name": "torpedo09",
                        "broadcaster_login": "torpedo09",
                        "broadcaster_id": "274637212",
                        "moderator_id": "141981764",
                        "moderator_login": "twitchdev",
                        "moderator_name": "TwitchDev",
                        "user_id": "424596340",
                        "user_login": "quotrok",
                        "user_name": "quotrok",
                        "text": "Please unban me from the channel?",
                        "status": "pending",
                        "created_at": "2022-08-07T02:07:55Z",
                        "resolved_at": null,
                        "resolution_text": null
                    }
                ],
                "pagination": {
                    "cursor": "eyJiIjpudWxsLCJhIjp7IkN1cnNvciI6IjEwMDQ3MzA2NDo4NjQwNjU3MToxSVZCVDFKMnY5M1BTOXh3d1E0dUdXMkJOMFcifX0"
                }
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn resolve_unban_request(&self) {
        self.api_mock("PATCH", "/moderation/unban_requests")
            .and(query_param("broadcaster_id", "274637212"))
            .and(query_param("moderator_id", "987654321"))
            .and(query_param(
                "unban_request_id",
                "92af127c-7326-4483-a52b-b0daa0be61c01",
            ))
            .and(query_param("status", "approved"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "id": "92af127c-7326-4483-a52b-b0da0be61c01",
                        "broadcaster_name": "torpedo09",
                        "broadcaster_login": "torpedo09",
                        "broadcaster_id": "274637212",
                        "moderator_id": "141981764",
                        "moderator_login": "twitchdev",
                        "moderator_name": "TwitchDev",
                        "user_id": "424596340",
                        "user_login": "quotrok",
                        "user_name": "quotrok",
                        "text": "Please unban me from the channel?",
                        "status": "approved",
                        "created_at": "2022-08-07T02:07:55Z",
                        "resolved_at": "2022-08-09T02:07:55Z",
                        "resolution_text": null
                    }
                ]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn get_blocked_terms(&self) {
        self.api_mock("GET", "/moderation/blocked_terms")
            .and(query_param("broadcaster_id", "1234"))
            .and(query_param("moderator_id", "5678"))
            .and(query_param("first", "10"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "broadcaster_id": "1234",
                        "moderator_id": "5678",
                        "id": "520e4d4e-0cda-49c7-821e-e5ef4f88c2f2",
                        "text": "A phrase I’m not fond of",
                        "created_at": "2021-09-29T19:45:37Z",
                        "updated_at": "2021-09-29T19:45:37Z",
                        "expires_at": null
                    },

                ],
                "pagination": {
                    "cursor": "eyJiIjpudWxsLCJhIjp7IkN1cnNvciI6I..."
                }
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn add_blocked_term(&self) {
        self.api_mock("POST", "/moderation/blocked_terms")
            .and(query_param("broadcaster_id", "1234"))
            .and(query_param("moderator_id", "5678"))
            .and(header("content-type", "application/json"))
            .and(body_json(json!({
                "text":"A phrase I’m not fond of"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "broadcaster_id": "713936733",
                        "moderator_id": "713936733",
                        "id": "3bb6e5d3-afb1-416c-ad4e-21af970ccfe7",
                        "text": "A phrase I’m not fond of",
                        "created_at": "2021-09-29T15:36:45Z",
                        "updated_at": "2021-09-29T15:36:45Z",
                        "expires_at": null
                    }
                ]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn add_blocked_term2(&self) {
        self.api_mock("POST", "/moderation/blocked_terms")
            .and(query_param("broadcaster_id", "1234"))
            .and(query_param("moderator_id", "5678"))
            .and(header("content-type", "application/json"))
            .and(body_json(json!({
                "text":"crac*"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "broadcaster_id": "1234",
                        "moderator_id": "5678",
                        "id": "520e4d4e-0cda-49c7-821e-e5ef4f88c2f2",
                        "text": "crac*",
                        "created_at": "2021-09-29T19:45:37Z",
                        "updated_at": "2021-09-29T19:45:37Z",
                        "expires_at": null
                    }
                ]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn remove_blocked_term(&self) {
        self.api_mock("DELETE", "/moderation/blocked_terms")
            .and(query_param("broadcaster_id", "1234"))
            .and(query_param("moderator_id", "5678"))
            .and(query_param("id", "c9fc79b8-0f63-4ef7-9d38-efd811e74ac2"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&self.server)
            .await;
    }

    pub async fn delete_chat_messages(&self) {
        self.api_mock("DELETE", "/moderation/chat")
            .and(query_param("broadcaster_id", "11111"))
            .and(query_param("moderator_id", "44444"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&self.server)
            .await;
    }

    pub async fn delete_chat_messages2(&self) {
        self.api_mock("DELETE", "/moderation/chat")
            .and(query_param("broadcaster_id", "11111"))
            .and(query_param("moderator_id", "44444"))
            .and(query_param("message_id", "abc-123-def"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&self.server)
            .await;
    }

    pub async fn get_moderated_channels(&self) {
        self.api_mock("GET", "/moderation/channels")
            .and(query_param("user_id", "931931"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "broadcaster_id" : "12345",
                        "broadcaster_login" : "grateful_broadcaster",
                        "broadcaster_name" : "Grateful_Broadcaster"
                    },
                    {
                        "broadcaster_id" : "98765",
                        "broadcaster_login" : "bashfulgamer",
                        "broadcaster_name" : "BashfulGamer"
                    },
                ],
                "pagination" : {
                    "cursor" : "eyJiIjpudWxsLCJhIjp7IkN1cnNvciI6IjEwMDQ3MzA2NDo4NjQwNjU3MToxSVZCVDFKMnY5M1BTOXh3d1E0dUdXMkJOMFcifX0"
                }
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn get_moderators(&self) {
        self.api_mock("GET", "/moderation/moderators")
            .and(query_param("broadcaster_id", "198704263"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "user_id": "424596340",
                        "user_login": "quotrok",
                        "user_name": "quotrok"
                    },
                ],
                "pagination": {
                    "cursor": "eyJiIjpudWxsLCJhIjp7IkN1cnNvciI6IjEwMDQ3MzA2NDo4NjQwNjU3MToxSVZCVDFKMnY5M1BTOXh3d1E0dUdXMkJOMFcifX0"
                }
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn add_channel_moderator(&self) {
        self.api_mock("POST", "/moderation/moderators")
            .and(query_param("broadcaster_id", "11111"))
            .and(query_param("user_id", "44444"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&self.server)
            .await;
    }

    pub async fn remove_channel_moderator(&self) {
        self.api_mock("DELETE", "/moderation/moderators")
            .and(query_param("broadcaster_id", "11111"))
            .and(query_param("user_id", "44444"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&self.server)
            .await;
    }

    pub async fn get_vips(&self) {
        self.api_mock("GET", "/channels/vips")
            .and(query_param("broadcaster_id", "123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "user_id": "11111",
                        "user_name": "UserDisplayName",
                        "user_login": "userloginname"
                    },
                ],
                "pagination": {
                    "cursor": "eyJiIjpudWxsLCJhIjp7Ik9mZnNldCI6NX19"
                }
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn get_vips2(&self) {
        self.api_mock("GET", "/channels/vips")
            .and(query_param("broadcaster_id", "123"))
            .and(query_param("user_id", "456"))
            .and(query_param("user_id", "678"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "user_id": "11111",
                        "user_name": "UserDisplayName",
                        "user_login": "userloginname"
                    },
                ],
                "pagination": {
                    "cursor": "eyJiIjpudWxsLCJhIjp7Ik9mZnNldCI6NX19"
                }
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn add_channel_vip(&self) {
        self.api_mock("POST", "/channels/vips")
            .and(query_param("broadcaster_id", "123"))
            .and(query_param("user_id", "456"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&self.server)
            .await;
    }

    pub async fn remove_channel_vip(&self) {
        self.api_mock("DELETE", "/channels/vips")
            .and(query_param("broadcaster_id", "123"))
            .and(query_param("user_id", "456"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&self.server)
            .await;
    }

    pub async fn update_shield_mode_status(&self) {
        self.api_mock("PUT", "/moderation/shield_mode")
            .and(query_param("broadcaster_id", "12345"))
            .and(query_param("moderator_id", "98765"))
            .and(header("content-type", "application/json"))
            .and(body_json(json!({
                "is_active": false
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "is_active": false,
                        "moderator_id": "98765",
                        "moderator_name": "SimplySimple",
                        "moderator_login": "simplysimple",
                        "last_activated_at": "2022-07-26T17:16:03.123Z"
                    }
                ]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn get_shield_mode_status(&self) {
        self.api_mock("GET", "/moderation/shield_mode")
            .and(query_param("broadcaster_id", "12345"))
            .and(query_param("moderator_id", "98765"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "is_active": true,
                        "moderator_id": "98765",
                        "moderator_name": "SimplySimple",
                        "moderator_login": "simplysimple",
                        "last_activated_at": "2022-07-26T17:16:03.123Z"
                    }
                ]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn warn_chat_user(&self) {
        self.api_mock("POST", "/moderation/warnings")
            .and(query_param("broadcaster_id", "404040"))
            .and(query_param("moderator_id", "404041"))
            .and(header("content-type", "application/json"))
            .and(body_json(json!({
                "data": {
                    "user_id": "9876",
                    "reason": "stop doing that!"
                }
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "broadcaster_id": "404040",
                        "user_id": "9876",
                        "moderator_id": "404041",
                        "reason": "stop doing that!"
                    }
                ]
            })))
            .mount(&self.server)
            .await;
    }
}

#[cfg(feature = "polls")]
impl TwitchApiTest {
    pub async fn get_polls(&self) {
        self.api_mock("GET", "/polls")
            .and(query_param("broadcaster_id", "141981764"))
            .and(query_param("id", "ed961efd-8a3f-4cf5-a9d0-e616c590cd2a"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "id": "ed961efd-8a3f-4cf5-a9d0-e616c590cd2a",
                        "broadcaster_id": "55696719",
                        "broadcaster_name": "TwitchDev",
                        "broadcaster_login": "twitchdev",
                        "title": "Heads or Tails?",
                        "choices": [
                            {
                                "id": "4c123012-1351-4f33-84b7-43856e7a0f47",
                                "title": "Heads",
                                "votes": 0,
                                "channel_points_votes": 0,
                                "bits_votes": 0
                            },
                            {
                                "id": "279087e3-54a7-467e-bcd0-c1393fcea4f0",
                                "title": "Tails",
                                "votes": 0,
                                "channel_points_votes": 0,
                                "bits_votes": 0
                            }
                        ],
                        "bits_voting_enabled": false,
                        "bits_per_vote": 0,
                        "channel_points_voting_enabled": false,
                        "channel_points_per_vote": 0,
                        "status": "ACTIVE",
                        "duration": 1800,
                        "started_at": "2021-03-19T06:08:33.871278372Z"
                    }
                ],
                "pagination": {}
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn create_poll(&self) {
        self.api_mock("POST", "/polls")
            .and(header("content-type", "application/json"))
            .and(body_json(json!({
                "broadcaster_id":"141981764",
                "title":"Heads or Tails?",
                "choices":[{
                        "title":"Heads"
                    },
                    {
                        "title":"Tails"
                }],
                "channel_points_voting_enabled":true,
                "channel_points_per_vote":100,
                "duration":1800,
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "id": "ed961efd-8a3f-4cf5-a9d0-e616c590cd2a",
                        "broadcaster_id": "141981764",
                        "broadcaster_name": "TwitchDev",
                        "broadcaster_login": "twitchdev",
                        "title": "Heads or Tails?",
                        "choices": [
                            {
                                "id": "4c123012-1351-4f33-84b7-43856e7a0f47",
                                "title": "Heads",
                                "votes": 0,
                                "channel_points_votes": 0,
                                "bits_votes": 0
                            },
                            {
                                "id": "279087e3-54a7-467e-bcd0-c1393fcea4f0",
                                "title": "Tails",
                                "votes": 0,
                                "channel_points_votes": 0,
                                "bits_votes": 0
                            }
                        ],
                        "bits_voting_enabled": false,
                        "bits_per_vote": 0,
                        "channel_points_voting_enabled": true,
                        "channel_points_per_vote": 100,
                        "status": "ACTIVE",
                        "duration": 1800,
                        "started_at": "2021-03-19T06:08:33.871278372Z"
                    }
                ]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn end_poll(&self) {
        self.api_mock("PATCH", "/polls")
            .and(header("content-type", "application/json"))
            .and(body_json(json!({
                "broadcaster_id":"141981764",
                "id":"ed961efd-8a3f-4cf5-a9d0-e616c590cd2a",
                "status":"TERMINATED"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "id": "ed961efd-8a3f-4cf5-a9d0-e616c590cd2a",
                        "broadcaster_id": "141981764",
                        "broadcaster_name": "TwitchDev",
                        "broadcaster_login": "twitchdev",
                        "title": "Heads or Tails?",
                        "choices": [
                            {
                                "id": "4c123012-1351-4f33-84b7-43856e7a0f47",
                                "title": "Heads",
                                "votes": 0,
                                "channel_points_votes": 0,
                                "bits_votes": 0
                            },
                            {
                                "id": "279087e3-54a7-467e-bcd0-c1393fcea4f0",
                                "title": "Tails",
                                "votes": 0,
                                "channel_points_votes": 0,
                                "bits_votes": 0
                            }
                        ],
                        "bits_voting_enabled": false,
                        "bits_per_vote": 0,
                        "channel_points_voting_enabled": true,
                        "channel_points_per_vote": 100,
                        "status": "TERMINATED",
                        "duration": 1800,
                        "started_at": "2021-03-19T06:08:33.871278372Z",
                        "ended_at": "2021-03-19T06:11:26.746889614Z"
                    }
                ]
            })))
            .mount(&self.server)
            .await;
    }
}

#[cfg(feature = "predictions")]
impl TwitchApiTest {
    pub async fn get_predictions(&self) {
        self.api_mock("GET", "/predictions")
            .and(query_param("broadcaster_id", "55696719"))
            .and(query_param("id", "d6676d5c-c86e-44d2-bfc4-100fb48f0656"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "id": "d6676d5c-c86e-44d2-bfc4-100fb48f0656",
                        "broadcaster_id": "55696719",
                        "broadcaster_name": "TwitchDev",
                        "broadcaster_login": "twitchdev",
                        "title": "Will there be any leaks today?",
                        "winning_outcome_id": null,
                        "outcomes": [
                            {
                                "id": "021e9234-5893-49b4-982e-cfe9a0aaddd9",
                                "title": "Yes",
                                "users": 0,
                                "channel_points": 0,
                                "top_predictors": null,
                                "color": "BLUE"
                            },
                            {
                                "id": "ded84c26-13cb-4b48-8cb5-5bae3ec3a66e",
                                "title": "No",
                                "users": 0,
                                "channel_points": 0,
                                "top_predictors": null,
                                "color": "PINK"
                            }
                        ],
                        "prediction_window": 600,
                        "status": "ACTIVE",
                        "created_at": "2021-04-28T16:03:06.320848689Z",
                        "ended_at": null,
                        "locked_at": null
                    }
                ],
                "pagination": {}
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn create_prediction(&self) {
        self.api_mock("POST", "/predictions")
            .and(header("content-type", "application/json"))
            .and(body_json(json!({
                "broadcaster_id": "141981764",
                "title": "Any leeks in the stream?",
                "outcomes": [
                    {
                        "title": "Yes, give it time."
                    },
                    {
                        "title": "Yes, give it time."
                    }
                ],
                "prediction_window": 120
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "id": "bc637af0-7766-4525-9308-4112f4cbf178",
                        "broadcaster_id": "141981764",
                        "broadcaster_name": "TwitchDev",
                        "broadcaster_login": "twitchdev",
                        "title": "Any leeks in the stream?",
                        "winning_outcome_id": null,
                        "outcomes": [
                            {
                                "id": "73085848-a94d-4040-9d21-2cb7a89374b7",
                                "title": "Yes, give it time.",
                                "users": 0,
                                "channel_points": 0,
                                "top_predictors": null,
                                "color": "BLUE"
                            },
                            {
                                "id": "906b70ba-1f12-47ea-9e95-e5f93d20e9cc",
                                "title": "Definitely not.",
                                "users": 0,
                                "channel_points": 0,
                                "top_predictors": null,
                                "color": "PINK"
                            }
                        ],
                        "prediction_window": 120,
                        "status": "ACTIVE",
                        "created_at": "2021-04-28T17:11:22.595914172Z",
                        "ended_at": null,
                        "locked_at": null
                    }
                ]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn end_prediction(&self) {
        self.api_mock("PATCH", "/predictions")
            .and(header("content-type", "application/json"))
            .and(body_json(json!({
                "broadcaster_id": "141981764",
                "id": "bc637af0-7766-4525-9308-4112f4cbf178",
                "status": "RESOLVED",
                "winning_outcome_id": "73085848-a94d-4040-9d21-2cb7a89374b7"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "id": "bc637af0-7766-4525-9308-4112f4cbf178",
                        "broadcaster_id": "141981764",
                        "broadcaster_name": "TwitchDev",
                        "broadcaster_login": "twitchdev",
                        "title": "Will we win all the games?",
                        "winning_outcome_id": "73085848-a94d-4040-9d21-2cb7a89374b7",
                        "outcomes": [
                            {
                                "id": "73085848-a94d-4040-9d21-2cb7a89374b7",
                                "title": "yes",
                                "users": 0,
                                "channel_points": 0,
                                "top_predictors": null,
                                "color": "BLUE"
                            },
                            {
                                "id": "86010b2e-9764-4136-9359-fd1c9c5a8033",
                                "title": "no",
                                "users": 0,
                                "channel_points": 0,
                                "top_predictors": null,
                                "color": "PINK"
                            }
                        ],
                        "prediction_window": 120,
                        "status": "RESOLVED",
                        "created_at": "2021-04-28T21:48:19.480371331Z",
                        "ended_at": "2021-04-28T21:54:24.026833954Z",
                        "locked_at": "2021-04-28T21:48:34.636685705Z"
                    }
                ]
            })))
            .mount(&self.server)
            .await;
    }
}

#[cfg(feature = "raid")]
impl TwitchApiTest {
    pub async fn start_raid(&self) {
        self.api_mock("POST", "/raids")
            .and(query_param("from_broadcaster_id", "12345678"))
            .and(query_param("to_broadcaster_id", "12345678"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "created_at": "2022-02-18T07:20:50.52Z",
                        "is_mature": false
                    }
            ]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn cancel_raid(&self) {
        self.api_mock("DELETE", "/raids")
            .and(query_param("broadcaster_id", "12345678"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&self.server)
            .await;
    }
}

#[cfg(feature = "schedule")]
impl TwitchApiTest {
    pub async fn get_channel_stream_schedule(&self) {
        self.api_mock("GET", "/schedule")
            .and(query_param("broadcaster_id", "141981764"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": {
                    "segments": [
                        {
                            "id": "eyJzZWdtZW50SUQiOiJlNGFjYzcyNC0zNzFmLTQwMmMtODFjYS0yM2FkYTc5NzU5ZDQiLCJpc29ZZWFyIjoyMDIxLCJpc29XZWVrIjoyNn0=",
                            "start_time": "2021-07-01T18:00:00Z",
                            "end_time": "2021-07-01T19:00:00Z",
                            "title": "TwitchDev Monthly Update // July 1, 2021",
                            "canceled_until": null,
                            "category": {
                                "id": "509670",
                                "name": "Science & Technology"
                            },
                            "is_recurring": false
                        },
                    ],
                    "broadcaster_id": "141981764",
                    "broadcaster_name": "TwitchDev",
                    "broadcaster_login": "twitchdev",
                    "vacation": null
                },
                "pagination": {} 
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn get_channel_icalendar(&self) {
        self.api_mock("GET", "/schedule/icalendar")
            .and(query_param("broadcaster_id", "141981764"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(
                        "
BEGIN:VCALENDAR
PRODID:-//twitch.tv//StreamSchedule//1.0
VERSION:2.0
CALSCALE:GREGORIAN
REFRESH-INTERVAL;VALUE=DURATION:PT1H
NAME:TwitchDev
BEGIN:VEVENT
UID:e4acc724-371f-402c-81ca-23ada79759d4
DTSTAMP:20210323T040131Z
DTSTART;TZID=/America/New_York:20210701T140000
DTEND;TZID=/America/New_York:20210701T150000
SUMMARY:TwitchDev Monthly Update // July 1, 2021
DESCRIPTION:Science & Technology.
CATEGORIES:Science & Technology
END:VEVENT
END:VCALENDAR%
                ",
                    )
                    .insert_header("content-type", "text/calendar"),
            )
            .mount(&self.server)
            .await;
    }

    pub async fn update_channel_stream_schedule(&self) {
        self.api_mock("PATCH", "/schedule/settings")
            .and(query_param("broadcaster_id", "141981764"))
            .and(query_param("is_vacation_enabled", "true"))
            .and(query_param("vacation_start_time", "2021-05-16T00:00:00Z"))
            .and(query_param("vacation_end_time", "2021-05-23T00:00:00Z"))
            .and(query_param("timezone", "America/New_York"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&self.server)
            .await;
    }

    pub async fn create_channel_stream_schedule_segment(&self) {
        self.api_mock("POST", "/schedule/segment")
            .and(query_param("broadcaster_id", "141981764"))
            .and(header("content-type", "application/json"))
            .and(body_json(json!({
                "start_time": "2021-07-01T18:00:00Z",
                "timezone": "America/New_York",
                "is_recurring": false,
                "duration": "60",
                "category_id": "509670",
                "title": "TwitchDev Monthly Update // July 1, 2021"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": {
                    "segments": [
                        {
                            "id": "eyJzZWdtZW50SUQiOiJlNGFjYzcyNC0zNzFmLTQwMmMtODFjYS0yM2FkYTc5NzU5ZDQiLCJpc29ZZWFyIjoyMDIxLCJpc29XZWVrIjoyNn0=",
                            "start_time": "2021-07-01T18:00:00Z",
                            "end_time": "2021-07-01T19:00:00Z",
                            "title": "TwitchDev Monthly Update // July 1, 2021",
                            "canceled_until": null,
                            "category": {
                                "id": "509670",
                                "name": "Science & Technology"
                            },
                            "is_recurring": false
                        }
                    ],
                    "broadcaster_id": "141981764",
                    "broadcaster_name": "TwitchDev",
                    "broadcaster_login": "twitchdev",
                    "vacation": null
                }
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn update_channel_stream_schedule_segment(&self) {
        self.api_mock("PATCH", "/schedule/segment")
            .and(query_param("broadcaster_id", "141981764"))
            .and(query_param("id", "eyJzZWdtZW50SUQiOiJlNGFjYzcyNC0zNzFmLTQwMmMtODFjYS0yM2FkYTc5NzU5ZDQiLCJpc29ZZWFyIjoyMDIxLCJpc29XZWVrIjoyNn0="))
            .and(header("content-type", "application/json"))
            .and(body_json(json!({
                "duration": "120",
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": {
                    "segments": [
                        {
                            "id": "eyJzZWdtZW50SUQiOiJlNGFjYzcyNC0zNzFmLTQwMmMtODFjYS0yM2FkYTc5NzU5ZDQiLCJpc29ZZWFyIjoyMDIxLCJpc29XZWVrIjoyNn0=",
                            "start_time": "2021-07-01T18:00:00Z",
                            "end_time": "2021-07-01T20:00:00Z",
                            "title": "TwitchDev Monthly Update // July 1, 2021",
                            "canceled_until": null,
                            "category": {
                                "id": "509670",
                                "name": "Science & Technology"
                            },
                            "is_recurring": false
                        }
                    ],
                    "broadcaster_id": "141981764",
                    "broadcaster_name": "TwitchDev",
                    "broadcaster_login": "twitchdev",
                    "vacation": null
                }
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn delete_channel_stream_schedule_segment(&self) {
        self.api_mock("DELETE", "/schedule/segment")
            .and(query_param("broadcaster_id", "141981764"))
            .and(query_param("id", "eyJzZWdtZW50SUQiOiI4Y2EwN2E2NC0xYTZkLTRjYWItYWE5Ni0xNjIyYzNjYWUzZDkiLCJpc29ZZWFyIjoyMDIxLCJpc29XZWVrIjoyMX0="))
            .respond_with(ResponseTemplate::new(204))
            .mount(&self.server)
            .await;
    }
}

#[cfg(feature = "search")]
impl TwitchApiTest {
    pub async fn search_categories(&self) {
        self.api_mock("GET", "/search/categories")
            .and(query_param("query", "fort"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "id": "33214",
                        "name": "Fortnite",
                        "box_art_url": "https://static-cdn.jtvnw.net/ttv-boxart/33214-52x72.jpg"
                    },
                ],
                "pagination": {
                    "cursor": "eyJiIjpudWxsLCJhIjp7IkN"
                }
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn search_channels(&self) {
        self.api_mock("GET", "/search/channels")
            .and(query_param("query", "twitchdev"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "broadcaster_language": "en",
                        "broadcaster_login": "twitchdev",
                        "display_name": "TwitchDev",
                        "game_id": "1469308723",
                        "game_name": "Software and Game Development",
                        "id": "141981764",
                        "is_live": false,
                        "tag_ids": [],
                        "tags": [
                            "WebDevelopment",
                            "GameDevelopment",
                            "SoftwareDevelopment",
                            "English"
                        ],
                        "thumbnail_url": "https://static-cdn.jtvnw.net/jtv_user_pictures/8a6381c7-d0c0-4576-b179-38bd5ce1d6af-profile_image-300x300.png",
                        "title": "Standard Output",
                        "started_at": ""
                    },
                ],
                "pagination": {
                    "cursor": "Mg=="
                }
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn search_channels_2(&self) {
        self.api_mock("GET", "/search/channels")
            .and(query_param("query", "a_seagull"))
            .and(query_param("live_only", "true"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "broadcaster_language": "en",
                        "broadcaster_login": "a_seagull",
                        "display_name": "A_Seagull",
                        "game_id": "506442",
                        "game_name": "DOOM Eternal",
                        "id": "19070311",
                        "is_live": true,
                        "tag_ids": [],
                        "tags": ["English"],
                        "thumbnail_url": "https://static-cdn.jtvnw.net/jtv_user_pictures/a_seagull-profile_image-4d2d235688c7dc66-300x300.png",
                        "title": "a_seagull",
                        "started_at": "2020-03-18T17:56:00Z"
                    }
                ],
                "pagination": {}
            })))
            .mount(&self.server)
            .await;
    }
}

#[cfg(feature = "streams")]
impl TwitchApiTest {
    pub async fn get_stream_key(&self) {
        self.api_mock("GET", "/streams/key")
            .and(query_param("broadcaster_id", "141981764"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "stream_key": "live_44322889_a34ub37c8ajv98a0"
                    },
                ]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn get_streams(&self) {
        self.api_mock("GET", "/streams")
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "id": "123456789",
                        "user_id": "98765",
                        "user_login": "sandysanderman",
                        "user_name": "SandySanderman",
                        "game_id": "494131",
                        "game_name": "Little Nightmares",
                        "type": "live",
                        "title": "hablamos y le damos a Little Nightmares 1",
                        "tags": ["Español"],
                        "viewer_count": 78365,
                        "started_at": "2021-03-10T15:04:21Z",
                        "language": "es",
                        "thumbnail_url": "https://static-cdn.jtvnw.net/previews-ttv/live_user_auronplay-{width}x{height}.jpg",
                        "tag_ids": [],
                        "is_mature": false
                    },
                ],
                "pagination": {
                    "cursor": "eyJiIjp7IkN1cnNvciI6ImV5SnpJam8zT0RNMk5TNDBORFF4TlRjMU1UY3hOU3dpWkNJNlptRnNjMlVzSW5RaU9uUnlkV1Y5In0sImEiOnsiQ3Vyc29yIjoiZXlKeklqb3hOVGs0TkM0MU56RXhNekExTVRZNU1ESXNJbVFpT21aaGJITmxMQ0owSWpwMGNuVmxmUT09In19"
                }
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn get_streams_2(&self) {
        self.api_mock("GET", "/streams")
            .and(query_param("user_login", "cohhcarnage"))
            .and(query_param("user_login", "lana_lux"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "id": "40952121085",
                        "user_id": "101051819",
                        "user_login": "afro",
                        "user_name": "Afro",
                        "game_id": "32982",
                        "game_name": "Grand Theft Auto V",
                        "type": "live",
                        "title": "Jacob: Digital Den Laptops & Routers | NoPixel | !MAINGEAR !FCF",
                        "tags": ["English"],
                        "viewer_count": 1490,
                        "started_at": "2021-03-10T03:18:11Z",
                        "language": "en",
                        "thumbnail_url": "https://static-cdn.jtvnw.net/previews-ttv/live_user_afro-{width}x{height}.jpg",
                        "tag_ids": [],
                        "is_mature": false
                    },
                ],
                "pagination": {}
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn get_followed_streams(&self) {
        self.api_mock("GET", "/streams/followed")
            .and(query_param("user_id", "141981764"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "id": "42170724654",
                        "user_id": "132954738",
                        "user_login": "aws",
                        "user_name": "AWS",
                        "game_id": "417752",
                        "game_name": "Talk Shows & Podcasts",
                        "type": "live",
                        "title": "AWS Howdy Partner! Y'all welcome ExtraHop to the show!",
                        "viewer_count": 20,
                        "started_at": "2021-03-31T20:57:26Z",
                        "language": "en",
                        "thumbnail_url": "https://static-cdn.jtvnw.net/previews-ttv/live_user_aws-{width}x{height}.jpg",
                        "tag_ids": [],
                        "tags": ["English"]
                    },
                ],
                "pagination": {
                    "cursor": "eyJiIjp7IkN1cnNvciI6ImV5SnpJam8zT0RNMk5TNDBORFF4TlRjMU1UY3hOU3dpWkNJNlptRnNjMlVzSW5RaU9uUnlkV1Y5In0sImEiOnsiQ3Vyc29yIjoiZXlKeklqb3hOVGs0TkM0MU56RXhNekExTVRZNU1ESXNJbVFpT21aaGJITmxMQ0owSWpwMGNuVmxmUT09In19"
                }
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn create_stream_marker(&self) {
        self.api_mock("POST", "/streams/markers")
            .and(header("content-type", "application/json"))
            .and(body_json(json!({
                "user_id": "123",
                "description": "hello, this is a marker!"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [{
                    "id": "123",
                    "created_at": "2018-08-20T20:10:03Z",
                    "description": "hello, this is a marker!",
                    "position_seconds": 244
                }]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn get_stream_markers(&self) {
        self.api_mock("GET", "/streams/markers")
            .and(query_param("user_id", "123"))
            .and(query_param("first", "5"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "user_id": "123",
                        "user_name": "TwitchName",
                        "user_login": "twitchname",
                        "videos": [
                            {
                                "video_id": "456",
                                "markers": [
                                    {
                                        "id": "106b8d6243a4f883d25ad75e6cdffdc4",
                                        "created_at": "2018-08-20T20:10:03Z",
                                        "description": "hello, this is a marker!",
                                        "position_seconds": 244,
                                        "URL": "https://twitch.tv/twitchname/manager/highlighter/456?t=0h4m06s"
                                    },
                                ]
                            }
                        ]
                    }
                ],
                "pagination": {
                    "cursor": "eyJiIjpudWxsLCJhIjoiMjk1MjA0Mzk3OjI1Mzpib29rbWFyazoxMDZiOGQ1Y"
                }
            })))
            .mount(&self.server)
            .await;
    }
}

#[cfg(feature = "subscriptions")]
impl TwitchApiTest {
    pub async fn get_broadcaster_subscriptions(&self) {
        self.api_mock("GET", "/subscriptions")
            .and(query_param("broadcaster_id", "141981764"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "broadcaster_id": "141981764",
                        "broadcaster_login": "twitchdev",
                        "broadcaster_name": "TwitchDev",
                        "gifter_id": "12826",
                        "gifter_login": "twitch",
                        "gifter_name": "Twitch",
                        "is_gift": true,
                        "tier": "1000",
                        "plan_name": "Channel Subscription (twitchdev)",
                        "user_id": "527115020",
                        "user_name": "twitchgaming",
                        "user_login": "twitchgaming"
                    },
                ],
                "pagination": {
                    "cursor": "xxxx"
                },
                "total": 13,
                "points": 13
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn check_user_subscription(&self) {
        self.api_mock("GET", "/subscriptions/user")
            .and(query_param("broadcaster_id", "149747285"))
            .and(query_param("user_id", "141981764"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "broadcaster_id": "149747285",
                        "broadcaster_name": "TwitchPresents",
                        "broadcaster_login": "twitchpresents",
                        "is_gift": false,
                        "tier": "1000"
                    }
                ]
            })))
            .mount(&self.server)
            .await;
    }
}

#[cfg(feature = "teams")]
impl TwitchApiTest {
    pub async fn get_channel_teams(&self) {
        self.api_mock("GET", "/teams/channel")
            .and(query_param("broadcaster_id", "96909659"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                 "data": [
                    {
                        "broadcaster_id": "96909659",
                        "broadcaster_name": "CSharpFritz",
                        "broadcaster_login": "csharpfritz",
                        "background_image_url": null,
                        "banner": null,
                        "created_at": "2019-02-11T12:09:22Z",
                        "updated_at": "2020-11-18T15:56:41Z",
                        "info": "<p>An outgoing and enthusiastic group of friendly channels that write code, teach about technology, and promote the technical community.</p>",
                        "thumbnail_url": "https://static-cdn.jtvnw.net/jtv_user_pictures/team-livecoders-team_logo_image-bf1d9a87ca81432687de60e24ad9593d-600x600.png",
                        "team_name": "livecoders",
                        "team_display_name": "Live Coders",
                        "id": "6358"
                    }
                ]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn get_teams(&self) {
        self.api_mock("GET", "/teams")
            .and(query_param("id", "6358"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                    "users": [
                        {
                            "user_id": "278217731",
                            "user_name": "mastermndio",
                            "user_login": "mastermndio"
                        },
                        {
                            "user_id": "41284990",
                            "user_name": "jenninexus",
                            "user_login": "jenninexus"
                        },
                    ],
                        "background_image_url": null,
                        "banner": null,
                        "created_at": "2019-02-11T12:09:22Z",
                        "updated_at": "2020-11-18T15:56:41Z",
                        "info": "<p>An outgoing and enthusiastic group of friendly channels that write code, teach about technology, and promote the technical community.</p>",
                        "thumbnail_url": "https://static-cdn.jtvnw.net/jtv_user_pictures/team-livecoders-team_logo_image-bf1d9a87ca81432687de60e24ad9593d-600x600.png",
                        "team_name": "livecoders",
                        "team_display_name": "Live Coders",
                        "id": "6358"
                    }
                ]
            })))
            .mount(&self.server)
            .await;
    }
}

#[cfg(feature = "users")]
impl TwitchApiTest {
    pub async fn get_users(&self) {
        self.api_mock("GET", "/users")
            .and(query_param("id", "141981764"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [{
                    "id": "141981764",
                    "login": "twitchdev",
                    "display_name": "TwitchDev",
                    "type": "",
                    "broadcaster_type": "partner",
                    "description": "Supporting third-party developers building Twitch integrations from chatbots to game integrations.",
                    "profile_image_url": "https://static-cdn.jtvnw.net/jtv_user_pictures/8a6381c7-d0c0-4576-b179-38bd5ce1d6af-profile_image-300x300.png",
                    "offline_image_url": "https://static-cdn.jtvnw.net/jtv_user_pictures/3f13ab61-ec78-4fe6-8481-8682cb3b0ac2-channel_offline_image-1920x1080.png",
                    "view_count": 5980557,
                    "email": "not-real@email.com",
                    "created_at": "2016-12-14T20:32:28Z"
                }]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn update_user(&self) {
        self.api_mock("PUT", "/users")
            .and(query_param("description", "BaldAngel"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [{
                    "id": "44322889",
                    "login": "dallas",
                    "display_name": "dallas",
                    "type": "staff",
                    "broadcaster_type": "affiliate",
                    "description": "BaldAngel",
                    "profile_image_url": "https://static-cdn.jtvnw.net/jtv_user_pictures/4d1f36cbf1f0072d-profile_image-300x300.png",
                    "offline_image_url": "https://static-cdn.jtvnw.net/jtv_user_pictures/dallas-channel_offline_image-2e82c1df2a464df7-1920x1080.jpeg",
                    "view_count": 6995,
                    "email": "not-real@email.com",
                    "created_at": "2013-06-03T19:12:02.580593Z"
                }]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn get_user_block_list(&self) {
        self.api_mock("GET", "/users/blocks")
            .and(query_param("broadcaster_id", "141981764"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "user_id": "135093069",
                        "user_login": "bluelava",
                        "display_name": "BlueLava"
                    },
                    {
                        "user_id": "27419011",
                        "user_login": "travistyoj",
                        "display_name": "TravistyOJ"
                    }
                ]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn block_user(&self) {
        self.api_mock("PUT", "/users/blocks")
            .and(query_param("target_user_id", "198704263"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&self.server)
            .await;
    }

    pub async fn unblock_user(&self) {
        self.api_mock("DELETE", "/users/blocks")
            .and(query_param("target_user_id", "198704263"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&self.server)
            .await;
    }

    pub async fn get_user_extensions(&self) {
        self.api_mock("GET", "/users/extensions/list")
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "id": "wi08ebtatdc7oj83wtl9uxwz807l8b",
                        "version": "1.1.8",
                        "name": "Streamlabs Leaderboard",
                        "can_activate": true,
                        "type": [
                            "panel"
                        ]
                    },
                    {
                        "id": "d4uvtfdr04uq6raoenvj7m86gdk16v",
                        "version": "2.0.2",
                        "name": "Prime Subscription and Loot Reminder",
                        "can_activate": true,
                        "type": [
                            "overlay"
                        ]
                    },
                    {
                        "id": "rh6jq1q334hqc2rr1qlzqbvwlfl3x0",
                        "version": "1.1.0",
                        "name": "TopClip",
                        "can_activate": true,
                        "type": [
                            "mobile",
                            "panel"
                        ]
                    },
                    {
                        "id": "zfh2irvx2jb4s60f02jq0ajm8vwgka",
                        "version": "1.0.19",
                        "name": "Streamlabs",
                        "can_activate": true,
                        "type": [
                            "mobile",
                            "overlay"
                        ]
                    },
                    {
                        "id": "lqnf3zxk0rv0g7gq92mtmnirjz2cjj",
                        "version": "0.0.1",
                        "name": "Dev Experience Test",
                        "can_activate": true,
                        "type": [
                            "component",
                            "mobile",
                            "panel",
                            "overlay"
                        ]
                    }
                ]
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn get_user_active_extensions(&self) {
        self.api_mock("GET", "/users/extensions")
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": {
                "panel": {
                    "1": {
                        "active": true,
                        "id": "rh6jq1q334hqc2rr1qlzqbvwlfl3x0",
                        "version": "1.1.0",
                        "name": "TopClip"
                    },
                    "2": {
                        "active": true,
                        "id": "wi08ebtatdc7oj83wtl9uxwz807l8b",
                        "version": "1.1.8",
                        "name": "Streamlabs Leaderboard"
                    },
                    "3": {
                        "active": true,
                        "id": "naty2zwfp7vecaivuve8ef1hohh6bo",
                        "version": "1.0.9",
                        "name": "Streamlabs Stream Schedule & Countdown"
                    }
                },
                "overlay": {
                    "1": {
                        "active": true,
                        "id": "zfh2irvx2jb4s60f02jq0ajm8vwgka",
                        "version": "1.0.19",
                        "name": "Streamlabs"
                    }
                },
                "component": {
                    "1": {
                        "active": true,
                        "id": "lqnf3zxk0rv0g7gq92mtmnirjz2cjj",
                        "version": "0.0.1",
                        "name": "Dev Experience Test",
                        "x": 0,
                        "y": 0
                    },
                    "2": {
                        "active": false
                    }
                }
            }
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn update_user_extensions(&self) {
        self.api_mock("PUT", "/users/extensions")
            .and(header("content-type", "application/json"))
            // .and(body_json(json!({
            //     "data": {
            //         "panel": {
            //             "1": {
            //                 "active": true,
            //                 "id": "rh6jq1q334hqc2rr1qlzqbvwlfl3x0",
            //                 "version": "1.1.0"
            //             },
            //             "2": {
            //                 "active": true,
            //                 "id": "wi08ebtatdc7oj83wtl9uxwz807l8b",
            //                 "version": "1.1.8"
            //             },
            //             "3": {
            //                 "active": true,
            //                 "id": "naty2zwfp7vecaivuve8ef1hohh6bo",
            //                 "version": "1.0.9"
            //             }
            //         },
            //         "overlay": {
            //             "1": {
            //                 "active": true,
            //                 "id": "zfh2irvx2jb4s60f02jq0ajm8vwgka",
            //                 "version": "1.0.19"
            //             }
            //         },
            //         "component": {
            //             "1": {
            //                 "active": true,
            //                 "id": "lqnf3zxk0rv0g7gq92mtmnirjz2cjj",
            //                 "version": "0.0.1",
            //                 "x": 0,
            //                 "y": 0
            //             },
            //             "2": {
            //                 "active": false
            //             }
            //         }
            //     }
            // })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": {
                    "panel": {
                        "1": {
                            "active": true,
                            "id": "rh6jq1q334hqc2rr1qlzqbvwlfl3x0",
                            "version": "1.1.0",
                            "name": "TopClip"
                        },
                        "2": {
                            "active": true,
                            "id": "wi08ebtatdc7oj83wtl9uxwz807l8b",
                            "version": "1.1.8",
                            "name": "Streamlabs Leaderboard"
                        },
                        "3": {
                            "active": true,
                            "id": "naty2zwfp7vecaivuve8ef1hohh6bo",
                            "version": "1.0.9",
                            "name": "Streamlabs Stream Schedule & Countdown"
                        }
                    },
                    "overlay": {
                        "1": {
                            "active": true,
                            "id": "zfh2irvx2jb4s60f02jq0ajm8vwgka",
                            "version": "1.0.19",
                            "name": "Streamlabs"
                        }
                    },
                    "component": {
                        "1": {
                            "active": true,
                            "id": "lqnf3zxk0rv0g7gq92mtmnirjz2cjj",
                            "version": "0.0.1",
                            "name": "Dev Experience Test",
                            "x": 0,
                            "y": 0
                        },
                        "2": {
                            "active": false
                        }
                    }
                }
            })))
            .mount(&self.server)
            .await;
    }
}

#[cfg(feature = "videos")]
impl TwitchApiTest {
    pub async fn get_videos(&self) {
        self.api_mock("GET", "/videos")
            .and(query_param("id", "335921245"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [{
                    "id": "335921245",
                    "stream_id": null,
                    "user_id": "141981764",
                    "user_login": "twitchdev",
                    "user_name": "TwitchDev",
                    "title": "Twitch Developers 101",
                    "description": "Welcome to Twitch development! Here is a quick overview of our products and information to help you get started.",
                    "created_at": "2018-11-14T21:30:18Z",
                    "published_at": "2018-11-14T22:04:30Z",
                    "url": "https://www.twitch.tv/videos/335921245",
                    "thumbnail_url": "https://static-cdn.jtvnw.net/cf_vods/d2nvs31859zcd8/twitchdev/335921245/ce0f3a7f-57a3-4152-bc06-0c6610189fb3/thumb/index-0000000000-%{width}x%{height}.jpg",
                    "viewable": "public",
                    "view_count": 1863062,
                    "language": "en",
                    "type": "upload",
                    "duration": "3m21s",
                    "muted_segments": [{
                            "duration": 30,
                            "offset": 120
                    }]
                }],
                "pagination": {}
            })))
            .mount(&self.server)
            .await;
    }

    pub async fn delete_videos(&self) {
        self.api_mock("DELETE", "/videos")
            .and(query_param("id", "1234"))
            .and(query_param("id", "9876"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    "1234",
                    "9876"
                ]
            })))
            .mount(&self.server)
            .await;
    }
}

#[cfg(feature = "whispers")]
impl TwitchApiTest {
    pub async fn send_whisper(&self) {
        self.api_mock("POST", "/whispers")
            .and(query_param("from_user_id", "123"))
            .and(query_param("to_user_id", "456"))
            .and(header("content-type", "application/json"))
            .and(body_json(json!({
                "message": "hello"
            })))
            .respond_with(ResponseTemplate::new(204))
            .mount(&self.server)
            .await;
    }
}
