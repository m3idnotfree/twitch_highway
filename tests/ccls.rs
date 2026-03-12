#[macro_use]
mod common;

use common::HttpMock;
use twitch_highway::ccls::CclsAPI;

#[tokio::test]
async fn get_content_classification_labels() {
    let suite = HttpMock::new().await;
    suite.get_content_classification_labels().await;

    let result = suite.api().get_content_classification_labels(None).await;

    assert!(result.is_ok());
}
