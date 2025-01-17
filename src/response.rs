use std::{fmt, marker::PhantomData};

use asknothingx2_util::api::{APIResponse, JsonError, StatusCode};
use serde::de::DeserializeOwned;

pub struct Response<T> {
    res: APIResponse,
    _marker: PhantomData<T>,
}

impl<T> Response<T>
where
    T: DeserializeOwned,
{
    pub fn new(res: APIResponse) -> Self {
        Self {
            res,
            _marker: PhantomData,
        }
    }
    pub fn status(&self) -> StatusCode {
        self.res.status()
    }

    pub fn is_success(&self) -> bool {
        self.res.is_success()
    }

    pub fn parse_response(self) -> Result<T, JsonError> {
        if self.res.raw_body().is_empty() {
            serde_json::from_str("\"\"").map_err(JsonError::DeserializationError)
        } else {
            self.res.into_json()
        }
    }

    pub fn text(&self) -> String {
        String::from_utf8_lossy(self.res.raw_body()).to_string()
    }
}

impl<T> fmt::Debug for Response<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Response")
            .field("status", &self.res.status())
            .field("body", &self.res.raw_body())
            .finish()
    }
}

impl<T> fmt::Display for Response<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Response {{ status: {}, body: {:?} }}",
            self.res.status(),
            self.res.raw_body()
        )
    }
}
