use serde::Serialize;

#[derive(Serialize)]
pub struct WhisperBody<'a> {
    pub message: &'a str,
}
