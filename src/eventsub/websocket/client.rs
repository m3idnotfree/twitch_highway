use std::{
    convert::Infallible,
    future::{Future, IntoFuture},
    marker::PhantomData,
    pin::{pin, Pin},
    str::FromStr,
    time::Duration,
};

use futures_util::{
    stream::{SplitSink, SplitStream},
    FutureExt, SinkExt, StreamExt,
};
use tokio::{net::TcpStream, sync::watch, time};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{Message, Utf8Bytes},
    MaybeTlsStream, WebSocketStream,
};
use tower::{util::ServiceExt, Service};
use tracing::{error, info, trace, warn};

use crate::eventsub::websocket::{scanner, Request, Response};

type WsSink = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;
type WsStream = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

type WsMessage = Option<Result<Message, tokio_tungstenite::tungstenite::Error>>;

pub fn client<M, S>(url: impl Into<String>, make_service: M) -> Client<M, S>
where
    M: Service<(), Error = Infallible, Response = S>,
    <M as Service<()>>::Future: Send,
    S: Service<Request, Response = Response, Error = Infallible> + Clone + Send + 'static,
    S::Future: Send,
{
    Client {
        url: url.into(),
        make_service,
        config: Config::default(),
        _marker: PhantomData,
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub max_reconnect_attempts: usize,
    pub initial_reconnect_delay: Duration,
    pub max_reconnect_delay: Duration,
    pub reconnect_grace_period: Duration,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_reconnect_attempts: 5,
            initial_reconnect_delay: Duration::from_secs(1),
            max_reconnect_delay: Duration::from_secs(30),
            reconnect_grace_period: Duration::from_secs(1),
        }
    }
}

pub struct Client<M, S> {
    url: String,
    make_service: M,
    config: Config,
    _marker: PhantomData<S>,
}

impl<M, S> Client<M, S>
where
    M: Service<(), Error = Infallible, Response = S>,
    <M as Service<()>>::Future: Send,
    S: Service<Request, Response = Response, Error = Infallible> + Clone + Send + 'static,
    S::Future: Send,
{
    pub fn with_config(mut self, config: Config) -> Self {
        self.config = config;
        self
    }

    pub fn with_graceful_shutdown<F>(self, signal: F) -> WithGracefulShutdown<M, S, F>
    where
        F: Future<Output = ()> + Send + 'static,
    {
        WithGracefulShutdown {
            url: self.url,
            make_service: self.make_service,
            config: self.config,
            signal,
            _marker: PhantomData,
        }
    }

    async fn run(self) -> Result<(), Error> {
        let Self {
            url,
            mut make_service,
            config,
            _marker,
        } = self;

        if !url.starts_with("ws://") && !url.starts_with("wss://") {
            return Err(Error::InvalidUrl(url));
        }

        let mut current_url = url;

        loop {
            let (mut write, read) = try_accept(&current_url, &config).await?;

            trace!("websocket connection established to twitch eventsub");

            let mut svc = make_service
                .call(())
                .await
                .expect("make_service error is Infallible");

            let recv_task = handle_connection(&mut write, read, &mut svc);

            match recv_task.await? {
                ConnectionResult::Url(url) => {
                    trace!("reconnect requested, switching to new url");
                    current_url = url;
                    time::sleep(config.reconnect_grace_period).await;
                }
                ConnectionResult::Closed => {
                    trace!("connection closed");
                    return Ok(());
                }
            }
        }
    }
}

impl<M, S> IntoFuture for Client<M, S>
where
    M: Service<(), Error = Infallible, Response = S> + Clone + Send + 'static,
    <M as Service<()>>::Future: Send,
    S: Service<Request, Response = Response, Error = Infallible> + Clone + Send + 'static,
    S::Future: Send,
{
    type Output = Result<(), Error>;
    type IntoFuture = Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.run().await })
    }
}

pub struct WithGracefulShutdown<M, S, F> {
    url: String,
    make_service: M,
    config: Config,
    signal: F,
    _marker: PhantomData<S>,
}

impl<M, S, F> WithGracefulShutdown<M, S, F>
where
    M: Service<(), Error = Infallible, Response = S>,
    <M as Service<()>>::Future: Send,
    S: Service<Request, Response = Response, Error = Infallible> + Clone + Send + 'static,
    S::Future: Send,
    F: Future<Output = ()> + Send + 'static,
{
    async fn run(self) -> Result<(), Error> {
        let Self {
            url,
            mut make_service,
            config,
            signal,
            _marker,
        } = self;

        if !url.starts_with("ws://") && !url.starts_with("wss://") {
            return Err(Error::InvalidUrl(url));
        }

        let (shutdown_tx, signal_rx) = watch::channel(());
        tokio::spawn(async move {
            signal.await;
            trace!("received shutdown signal, initiating graceful shutdown");
            drop(signal_rx);
        });

        let mut current_url = url;

        loop {
            let (mut write, read) = tokio::select! {
                conn = try_accept(&current_url, &config) => conn?,
                _ = shutdown_tx.closed() => {
                    trace!("shutdown signal received, stopping connection");
                    return Ok(());
                }
            };

            trace!("websocket connection established to twitch eventsub");

            let mut svc = make_service
                .call(())
                .await
                .expect("make_service error is Infallible");

            let recv_task = handle_connection(&mut write, read, &mut svc);

            let mut signal_closed = pin!(shutdown_tx.closed().fuse());
            tokio::select! {
                result = recv_task => {
                    match result? {
                        ConnectionResult::Url(url) => {
                            trace!("reconnect requested, switching to new url");
                            current_url = url;
                            time::sleep(config.reconnect_grace_period).await;
                        },
                        ConnectionResult::Closed => {
                            trace!("connection closed");
                            return Ok(());

                        },
                    }
                }
                _ = &mut signal_closed => {
                        trace!("shutdown signal received");
                        return Ok(())
                }
            }
        }
    }
}

impl<M, S, F> IntoFuture for WithGracefulShutdown<M, S, F>
where
    M: Service<(), Error = Infallible, Response = S> + Clone + Send + 'static,
    <M as Service<()>>::Future: Send,
    S: Service<Request, Response = Response, Error = Infallible> + Clone + Send + 'static,
    S::Future: Send,
    F: Future<Output = ()> + Send + 'static,
{
    type Output = Result<(), Error>;
    type IntoFuture = Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.run().await })
    }
}

async fn try_accept(url: &str, config: &Config) -> Result<(WsSink, WsStream), Error> {
    let mut attempts = 0;

    loop {
        attempts += 1;

        match connect_async(url).await {
            Ok((ws_stream, _)) => {
                info!("successfully conncted to websocket");
                return Ok(ws_stream.split());
            }
            Err(e) => {
                if attempts >= config.max_reconnect_attempts {
                    error!("failed to connect after {} attempts", attempts);
                    return Err(Error::FailedConnect { attempts });
                }

                let delay = calculate_backoff_delay(attempts, config);

                warn!(
                    "connection attempt {}/{} failed: {}, retrying in {:?}",
                    attempts, config.max_reconnect_attempts, e, delay
                );

                time::sleep(delay).await;
            }
        }
    }
}

enum ConnectionResult {
    Url(String),
    Closed,
}

async fn handle_connection<S>(
    write: &mut WsSink,
    mut read: WsStream,
    svc: &mut S,
) -> Result<ConnectionResult, Error>
where
    S: Service<Request, Response = Response, Error = Infallible> + Clone + Send + 'static,
    S::Future: Send,
{
    loop {
        let msg = read.next().await;
        if let Some(result) = handle_messages(write, svc, msg).await? {
            return Ok(result);
        }
    }
}

async fn handle_messages<S>(
    write: &mut WsSink,
    svc: &mut S,
    msg: WsMessage,
) -> Result<Option<ConnectionResult>, Error>
where
    S: Service<Request, Response = Response, Error = Infallible>,
{
    match msg {
        Some(Ok(Message::Text(text))) => match handle_text_message(write, svc, text).await {
            Ok(Some(url)) => {
                trace!("received reconnect request, closing current connection");
                let _ = write.close().await;
                Ok(Some(ConnectionResult::Url(url)))
            }
            Ok(None) => Ok(None),
            Err(e) => {
                warn!("error handling text message: {}, closing connection", e);
                let _ = write.close().await;
                Err(e)
            }
        },
        Some(Ok(Message::Ping(ping))) => {
            trace!("received ping, sending pong");
            write.send(Message::Pong(ping)).await?;
            Ok(None)
        }
        Some(Ok(Message::Close(frame))) => {
            trace!("received close frame: {:?}", frame);
            let _ = write.close().await;
            Ok(Some(ConnectionResult::Closed))
        }
        Some(Err(e)) => {
            error!("websocket error: {}", e);
            let _ = write.close().await;
            Err(Error::WebSocket(e))
        }
        Some(Ok(Message::Pong(_) | Message::Binary(_) | Message::Frame(_))) => {
            trace!("ignoring non-text message");
            Ok(None)
        }
        None => {
            trace!("websocket stream ended");
            Ok(Some(ConnectionResult::Closed))
        }
    }
}

async fn handle_text_message<S>(
    write: &mut WsSink,
    svc: &mut S,
    text: Utf8Bytes,
) -> Result<Option<String>, Error>
where
    S: Service<Request, Response = Response, Error = Infallible>,
{
    let req = Request::from_str(&text)?;

    if req.is_keepalive() {
        trace!("received keepalive, sending pong");
        write.send(Message::Pong("".into())).await?;

        return Ok(None);
    }

    if req.is_reconnect() {
        info!("server requested reconnect to: {}", req.get_reconnect_url());
        return Ok(Some(req.get_reconnect_url().to_string()));
    }
    svc.ready().await.expect("service error is Infallible");

    let resp = svc.call(req).await.expect("service error is Infallible");

    if resp.is_reconnect() {
        trace!("handler requested reconnect");
        return Ok(resp.url);
    }

    Ok(None)
}

fn calculate_backoff_delay(attempts: usize, config: &Config) -> Duration {
    let backoff_multiplier = 2u32.saturating_pow(attempts.saturating_sub(1).min(5) as u32);
    config
        .initial_reconnect_delay
        .saturating_mul(backoff_multiplier)
        .min(config.max_reconnect_delay)
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("WebSocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("Connection failed after {attempts} attempts")]
    FailedConnect { attempts: usize },
    #[error("Failed to parse message: {0}")]
    ParseError(#[from] scanner::ScanError),
    #[error("Invalid Websocket URL: {0}")]
    InvalidUrl(String),
}
