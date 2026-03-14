use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    future::Future,
    pin::Pin,
    task::{Context, Poll, ready},
    time::{Duration, Instant},
};

use pin_project_lite::pin_project;
use tower::{Layer, Service};
use tracing::{Level, Span, span};

use crate::websocket::{Request, Response};

pub trait MakeSpan {
    fn make_span(&mut self, req: &Request) -> Span;
}

pub trait OnRequest {
    fn on_request(&mut self, req: &Request, span: &Span);
}

pub trait OnResponse {
    fn on_response(&mut self, resp: &Response, latency: Duration, span: &Span);
}

impl<F> MakeSpan for F
where
    F: FnMut(&Request) -> Span,
{
    fn make_span(&mut self, req: &Request) -> Span {
        self(req)
    }
}

impl<F> OnRequest for F
where
    F: FnMut(&Request, &Span),
{
    fn on_request(&mut self, req: &Request, span: &Span) {
        self(req, span)
    }
}

impl<F> OnResponse for F
where
    F: FnMut(&Response, Duration, &Span),
{
    fn on_response(&mut self, resp: &Response, latency: Duration, span: &Span) {
        self(resp, latency, span)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DefaultMakeSpan {
    level: Level,
}

impl DefaultMakeSpan {
    pub fn new() -> Self {
        Self {
            level: Level::DEBUG,
        }
    }

    pub fn level(mut self, level: Level) -> Self {
        self.level = level;
        self
    }
}

impl Default for DefaultMakeSpan {
    fn default() -> Self {
        Self::new()
    }
}

impl MakeSpan for DefaultMakeSpan {
    fn make_span(&mut self, req: &Request) -> Span {
        match self.level {
            Level::ERROR => span!(Level::ERROR, "eventsub", msg = %req.message_type),
            Level::WARN => span!(Level::WARN, "eventsub", msg = %req.message_type),
            Level::INFO => span!(Level::INFO, "eventsub", msg = %req.message_type),
            Level::DEBUG => span!(Level::DEBUG, "eventsub", msg = %req.message_type),
            Level::TRACE => span!(Level::TRACE, "eventsub", msg = %req.message_type),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DefaultOnRequest {
    level: Level,
}

impl DefaultOnRequest {
    pub fn new() -> Self {
        Self {
            level: Level::DEBUG,
        }
    }

    pub fn level(mut self, level: Level) -> Self {
        self.level = level;
        self
    }
}

impl Default for DefaultOnRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl OnRequest for DefaultOnRequest {
    fn on_request(&mut self, _req: &Request, _span: &Span) {
        match self.level {
            Level::ERROR => tracing::event!(Level::ERROR, "started processing request"),
            Level::WARN => tracing::event!(Level::WARN, "started processing request"),
            Level::INFO => tracing::event!(Level::INFO, "started processing request"),
            Level::DEBUG => tracing::event!(Level::DEBUG, "started processing request"),
            Level::TRACE => tracing::event!(Level::TRACE, "started processing request"),
        }
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum LatencyUnit {
    Seconds,
    Millis,
    Micros,
    Nanos,
}

struct Latency {
    unit: LatencyUnit,
    duration: Duration,
}

impl Display for Latency {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.unit {
            LatencyUnit::Seconds => write!(f, "{} s", self.duration.as_secs_f64()),
            LatencyUnit::Millis => write!(f, "{} ms", self.duration.as_millis()),
            LatencyUnit::Micros => write!(f, "{} μs", self.duration.as_micros()),
            LatencyUnit::Nanos => write!(f, "{} ns", self.duration.as_nanos()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DefaultOnResponse {
    level: Level,
    latency_unit: LatencyUnit,
}

impl DefaultOnResponse {
    pub fn new() -> Self {
        Self {
            level: Level::DEBUG,
            latency_unit: LatencyUnit::Millis,
        }
    }

    pub fn level(mut self, level: Level) -> Self {
        self.level = level;
        self
    }

    pub fn latency_unit(mut self, latency_unit: LatencyUnit) -> Self {
        self.latency_unit = latency_unit;
        self
    }
}

impl OnResponse for DefaultOnResponse {
    fn on_response(&mut self, resp: &Response, latency: Duration, _span: &Span) {
        let latency = Latency {
            unit: self.latency_unit,
            duration: latency,
        };

        match self.level {
            Level::ERROR => {
                tracing::event!(Level::ERROR, %latency, status = %resp.status, "finished processing request")
            }
            Level::WARN => {
                tracing::event!(Level::WARN, %latency, status = %resp.status, "finished processing request")
            }
            Level::INFO => {
                tracing::event!(Level::INFO, %latency, status = %resp.status, "finished processing request")
            }
            Level::DEBUG => {
                tracing::event!(Level::DEBUG, %latency, status = %resp.status, "finished processing request")
            }
            Level::TRACE => {
                tracing::event!(Level::TRACE, %latency, status = %resp.status, "finished processing request")
            }
        }
    }
}

impl Default for DefaultOnResponse {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TraceLayer<
    MakeSpan = DefaultMakeSpan,
    OnRequest = DefaultOnRequest,
    OnResponse = DefaultOnResponse,
> {
    make_span: MakeSpan,
    on_request: OnRequest,
    on_response: OnResponse,
}

impl TraceLayer {
    pub fn new() -> Self {
        Self {
            make_span: DefaultMakeSpan::default(),
            on_request: DefaultOnRequest::default(),
            on_response: DefaultOnResponse::default(),
        }
    }
}

impl Default for TraceLayer {
    fn default() -> Self {
        Self::new()
    }
}

impl<MakeSpanT, OnRequestT, OnResponseT> TraceLayer<MakeSpanT, OnRequestT, OnResponseT> {
    pub fn make_span_with<NewMakeSpan>(
        self,
        make_span: NewMakeSpan,
    ) -> TraceLayer<NewMakeSpan, OnRequestT, OnResponseT> {
        TraceLayer {
            make_span,
            on_request: self.on_request,
            on_response: self.on_response,
        }
    }

    pub fn on_request<NewOnRequest>(
        self,
        on_request: NewOnRequest,
    ) -> TraceLayer<MakeSpanT, NewOnRequest, OnResponseT> {
        TraceLayer {
            make_span: self.make_span,
            on_request,
            on_response: self.on_response,
        }
    }

    pub fn on_response<NewOnResponse>(
        self,
        on_response: NewOnResponse,
    ) -> TraceLayer<MakeSpanT, OnRequestT, NewOnResponse> {
        TraceLayer {
            make_span: self.make_span,
            on_request: self.on_request,
            on_response,
        }
    }
}

impl<S, MakeSpanT, OnRequestT, OnResponseT> Layer<S>
    for TraceLayer<MakeSpanT, OnRequestT, OnResponseT>
where
    MakeSpanT: Clone,
    OnRequestT: Clone,
    OnResponseT: Clone,
{
    type Service = TraceService<S, MakeSpanT, OnRequestT, OnResponseT>;

    fn layer(&self, inner: S) -> Self::Service {
        TraceService {
            inner,
            make_span: self.make_span.clone(),
            on_request: self.on_request.clone(),
            on_response: self.on_response.clone(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TraceService<S, MakeSpan, OnRequest, OnResponse> {
    inner: S,
    make_span: MakeSpan,
    on_request: OnRequest,
    on_response: OnResponse,
}

impl<S, MakeSpanT, OnRequestT, OnResponseT> Service<Request>
    for TraceService<S, MakeSpanT, OnRequestT, OnResponseT>
where
    S: Service<Request, Response = Response>,
    MakeSpanT: MakeSpan,
    OnRequestT: OnRequest,
    OnResponseT: OnResponse + Clone,
{
    type Response = Response;
    type Error = S::Error;
    type Future = TraceFuture<S::Future, OnResponseT>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let start = Instant::now();
        let span = self.make_span.make_span(&req);

        let future = {
            let _guard = span.enter();
            self.on_request.on_request(&req, &span);
            self.inner.call(req)
        };

        TraceFuture {
            future,
            span,
            on_response: Some(self.on_response.clone()),
            start,
        }
    }
}

pin_project! {
    pub struct TraceFuture<F,OnResponse> {
        #[pin]
        future: F,
        span: Span,
        on_response: Option<OnResponse>,
        start: Instant,
    }
}

impl<F, E, OnResponseT> Future for TraceFuture<F, OnResponseT>
where
    F: Future<Output = Result<Response, E>>,
    OnResponseT: OnResponse,
{
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let _guard = this.span.enter();
        let result = ready!(this.future.poll(cx));
        let latency = this.start.elapsed();

        if let Ok(resp) = &result {
            this.on_response
                .take()
                .unwrap()
                .on_response(resp, latency, this.span);
        }

        Poll::Ready(result)
    }
}
