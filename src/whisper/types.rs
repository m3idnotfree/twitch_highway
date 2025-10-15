use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct WhisperBody<'a> {
    pub message: &'a str,
}
