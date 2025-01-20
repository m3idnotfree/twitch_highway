use serde::{Deserialize, Serialize};
use url::Url;

use crate::types::Category;

pub trait TestUrl {
    fn with_url(self, port: Option<u16>, endpoint: Option<String>) -> Self;
}

#[derive(Default, Debug)]
pub struct TestUrlHold {
    endpoint: Option<String>,
    port: Option<u16>,
}

impl TestUrlHold {
    /// defualt 'mock'
    pub fn with_endpoint(&mut self, endpoint: Option<String>) -> &mut Self {
        if let Some(endpoint) = endpoint {
            self.endpoint = Some(endpoint);
        } else {
            self.endpoint = Some("mock".to_string());
        }
        self
    }

    /// default '8080'
    pub fn with_port(&mut self, port: Option<u16>) -> &mut Self {
        if let Some(port) = port {
            self.port = Some(port);
        } else {
            self.port = Some(8080);
        }
        self
    }

    pub fn from_url(&self, url: &Url) -> Result<Url, crate::Error> {
        if self.endpoint.is_none() && self.port.is_none() {
            Err(crate::Error::MissingTestUrl)
        } else {
            let mut test_url = Url::parse("http://localhost:8080").unwrap();

            test_url.set_port(self.port).unwrap();
            let paths = url
                .path_segments()
                .unwrap()
                .skip(1)
                .fold(self.endpoint.clone().unwrap(), |acc, x| {
                    format!("{acc}/{x}")
                });
            test_url.set_path(&paths);
            test_url.set_query(url.query());

            Ok(test_url)
        }
    }
}

#[cfg(feature = "subscriptions")]
use crate::subscriptions::types::Subscription;
pub async fn mock_categories(
    port: Option<u16>,
) -> Result<MockData<Category>, asknothingx2_util::api::ReqwestError> {
    let mut url = Url::parse("http://localhost:8080/units/categories").unwrap();
    if port.is_some() {
        url.set_port(port).unwrap();
    }

    asknothingx2_util::api::get(url)
        .await?
        .json::<MockData<Category>>()
        .await
}

#[cfg(feature = "streams")]
use crate::streams::types::Stream;
#[cfg(feature = "streams")]
pub async fn mock_streams(
    port: Option<u16>,
) -> Result<MockData<Stream>, asknothingx2_util::api::ReqwestError> {
    let mut url = Url::parse("http://localhost:8080/units/streams").unwrap();
    if port.is_some() {
        url.set_port(port).unwrap();
    }

    asknothingx2_util::api::get(url)
        .await?
        .json::<MockData<Stream>>()
        .await
}

#[cfg(feature = "subscriptions")]
pub async fn mock_subscriptions(
    port: Option<u16>,
) -> Result<MockData<Subscription>, asknothingx2_util::api::ReqwestError> {
    let mut url = Url::parse("http://localhost:8080/units/subscriptions").unwrap();
    if port.is_some() {
        url.set_port(port).unwrap();
    }

    asknothingx2_util::api::get(url)
        .await?
        .json::<MockData<Subscription>>()
        .await
}

#[cfg(feature = "videos")]
use crate::videos::types::Video;
#[cfg(feature = "videos")]
pub async fn mock_videos(
    port: Option<u16>,
) -> Result<MockData<Video>, asknothingx2_util::api::ReqwestError> {
    let mut url = Url::parse("http://localhost:8080/units/videos").unwrap();
    if port.is_some() {
        url.set_port(port).unwrap();
    }

    asknothingx2_util::api::get(url)
        .await?
        .json::<MockData<Video>>()
        .await
}

#[cfg(feature = "teams")]
use crate::teams::types::Team;
#[cfg(feature = "teams")]
pub async fn mock_teams(
    port: Option<u16>,
) -> Result<MockData<Team>, asknothingx2_util::api::ReqwestError> {
    let mut url = Url::parse("http://localhost:8080/units/teams").unwrap();
    if port.is_some() {
        url.set_port(port).unwrap();
    }

    asknothingx2_util::api::get(url)
        .await?
        .json::<MockData<Team>>()
        .await
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MockData<T> {
    pub cursor: String,
    pub total: u64,
    pub data: Vec<T>,
}
