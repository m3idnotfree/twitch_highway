#![allow(dead_code)]

use std::time::Duration;

use tokio::{
    net::TcpStream,
    process::{Child, Command},
    time,
};

#[derive(Default, Debug, Clone)]
pub struct CliConfig {
    pub command: String,
    pub args: Vec<String>,
    pub kill_on_drop: bool,
    pub url: String,
    pub delay: Duration,
}

impl CliConfig {
    pub fn new() -> Self {
        Self {
            kill_on_drop: true,
            ..Self::default()
        }
    }

    pub fn mock_api() -> Self {
        Self {
            command: "twitch".to_string(),
            args: vec!["mock-api".to_string(), "start".to_string()],
            kill_on_drop: true,
            url: "127.0.0.1:8080".to_string(),
            delay: Duration::from_secs(5),
        }
    }

    pub fn websocket_server() -> Self {
        Self {
            command: "twitch".to_string(),
            args: vec![
                "event".to_string(),
                "websocket".to_string(),
                "start-server".to_string(),
            ],
            kill_on_drop: true,
            url: "ws://127.0.0.1:8080/ws".to_string(),
            delay: Duration::from_secs(5),
        }
    }

    pub fn webhook_trigger() -> Self {
        Self {
            command: "twitch".to_string(),
            args: vec![],
            kill_on_drop: true,
            url: "".to_string(),
            delay: Duration::from_millis(0),
        }
    }

    pub fn disable_kill_on_drop(mut self) -> Self {
        self.kill_on_drop = false;
        self
    }

    pub fn set_delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }

    pub fn set_url(mut self, url: impl Into<String>) -> Self {
        self.url = url.into();
        self
    }

    pub fn add_arg(&mut self, arg: impl Into<String>) -> &mut Self {
        self.args.push(arg.into());
        self
    }

    pub fn add_args<I, S>(&mut self, args: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.args.extend(args.into_iter().map(Into::into));
        self
    }

    pub fn command(self) -> Command {
        let mut command = Command::new(self.command);
        command.args(self.args).kill_on_drop(self.kill_on_drop);
        command
    }

    pub fn spawn(self) -> Result<Child, tokio::io::Error> {
        self.command().spawn()
    }

    pub async fn spawn_wait_server(self) -> Result<Child, CliError> {
        let url = self.url.clone();
        let delay = self.delay;

        let command = self.spawn()?;
        let _ = time::timeout(delay, TcpStream::connect(url)).await?;

        Ok(command)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error("timed out waiting for server to start")]
    Timeout(#[from] tokio::time::error::Elapsed),
    #[error("failed to execute CLI command: {0}")]
    Io(#[from] std::io::Error),
}
