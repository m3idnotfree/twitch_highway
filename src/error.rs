// use twitch_oauth_token::types::ErrorResponse;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("request error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("not implement : {0}")]
    NotImplementError(String),
    #[error("request body error: {0}")]
    RequestBodyError(String),
    #[error("GetUserError")]
    UserError(#[from] GetUsersError),
    // #[error("error response: {0}")]
    // ResponseError(ErrorResponse),
}
#[derive(Debug, thiserror::Error)]
pub enum GetUsersError {
    #[error("request specify under 100  user info: {0}")]
    AddIdError(String),
    #[error("request specify under 100  user info: {0}")]
    AddLoginError(String),
}
