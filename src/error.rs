#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("URL parse error {0}")]
    UrlParseError(#[from] url::ParseError),
    #[error("Missing Test URL")]
    MissingTestUrl,
}
