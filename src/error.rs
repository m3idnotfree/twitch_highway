use std::fmt;

type BoxError = Box<dyn std::error::Error + Send + Sync>;

pub struct Error {
    inner: Box<Inner>,
}

#[derive(Debug)]
struct Inner {
    kind: Kind,
    message: Option<String>,
    source: Option<BoxError>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Kind {
    Request,
    Api,
    Decode,
}

impl Kind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Kind::Request => "error sending request",
            Kind::Api => "error twitch api",
            Kind::Decode => "error decoding response body",
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
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

    // pub(crate) fn with_message_and_source(
    //     kind: Kind,
    //     message: impl Into<String>,
    //     source: impl Into<BoxError>,
    // ) -> Self {
    //     Self {
    //         inner: Box::new(Inner {
    //             kind,
    //             message: Some(message.into()),
    //             source: Some(source.into()),
    //         }),
    //     }
    // }

    pub fn message(&self) -> Option<&str> {
        self.inner.message.as_deref()
    }

    pub fn is_request(&self) -> bool {
        matches!(self.inner.kind, Kind::Request)
    }

    pub fn is_api(&self) -> bool {
        matches!(self.inner.kind, Kind::Api)
    }

    pub fn is_decode(&self) -> bool {
        matches!(self.inner.kind, Kind::Decode)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut builder = f.debug_struct("twitch_highway::Error");

        builder.field("kind", &self.inner.kind);

        if let Some(ref message) = self.inner.message {
            builder.field("message", message);
        }

        if let Some(ref source) = self.inner.source {
            builder.field("source", source);
        }

        builder.finish()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref message) = self.inner.message {
            write!(f, "{message}")
        } else {
            write!(f, "{}", self.inner.kind)
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
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

pub(crate) fn decode_error<E: Into<BoxError>>(source: E) -> Error {
    Error::with_source(Kind::Decode, source)
}
