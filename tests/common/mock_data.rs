use asknothingx2_util::api::preset;
use serde::{Deserialize, Serialize};

use twitch_highway::streams::Stream;

#[derive(Debug, Deserialize, Serialize)]
pub struct MockStreams {
    pub cursor: String,
    pub total: usize,
    pub data: Vec<Stream>,
}

pub async fn get_stream_data() -> Result<MockStreams, MockDataError> {
    let client = preset::testing("twitch-hyghway-test/1.0").build().unwrap();
    client
        .get("http://localhost:8080/units/streams")
        .send()
        .await?
        .json::<MockStreams>()
        .await
        .map_err(MockDataError::from)
}

#[derive(Debug, thiserror::Error)]
pub enum MockDataError {
    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),
}
