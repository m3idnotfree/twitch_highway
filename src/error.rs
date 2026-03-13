use std::{
    error::Error as StdError,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
};

type BoxError = Box<dyn StdError + Send + Sync>;

pub struct Error {
    inner: Box<Inner>,
}

#[derive(Debug)]
struct Inner {
    kind: Kind,
    message: Option<String>,
    source: Option<BoxError>,
}

#[derive(Debug)]
pub(crate) enum Kind {
    Request,
    Api,
    Parse,
}

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Kind::Request => f.write_str("request failed"),
            Kind::Api => f.write_str("api request failed"),
            Kind::Parse => f.write_str("response deserialization failed"),
        }
    }
}

impl Error {
    pub(crate) fn with_source(kind: Kind, source: impl Into<BoxError>) -> Self {
        Self {
            inner: Box::new(Inner {
                kind,
                message: None,
                source: Some(source.into()),
            }),
        }
    }

    pub(crate) fn with_message(kind: Kind, message: impl Into<String>) -> Self {
        Self {
            inner: Box::new(Inner {
                kind,
                message: Some(message.into()),
                source: None,
            }),
        }
    }

    pub fn message(&self) -> Option<&str> {
        self.inner.message.as_deref()
    }

    pub fn is_request(&self) -> bool {
        matches!(self.inner.kind, Kind::Request)
    }

    pub fn is_api(&self) -> bool {
        matches!(self.inner.kind, Kind::Api)
    }

    pub fn is_parse(&self) -> bool {
        matches!(self.inner.kind, Kind::Parse)
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut builder = f.debug_struct("twitch_highway::Error");

        builder.field("kind", &self.inner.kind);

        if let Some(message) = &self.inner.message {
            builder.field("message", message);
        }

        if let Some(source) = &self.inner.source {
            builder.field("source", source);
        }

        builder.finish()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.inner.kind)?;
        if let Some(msg) = &self.inner.message {
            write!(f, ": {msg}")?;
        }
        if let Some(src) = &self.inner.source {
            write!(f, ": {src}")?;
        }
        Ok(())
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.inner.source.as_ref().map(|e| &**e as _)
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::with_source(Kind::Request, value)
    }
}

pub(crate) fn api_error(message: impl Into<String>) -> Error {
    Error::with_message(Kind::Api, message.into())
}

pub(crate) fn parse_error<E: Into<BoxError>>(source: E) -> Error {
    Error::with_source(Kind::Parse, source)
}
