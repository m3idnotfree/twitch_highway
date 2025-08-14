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
