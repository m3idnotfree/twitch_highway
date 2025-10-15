#![allow(dead_code)]

use std::{
    fs::OpenOptions,
    future::{Future, IntoFuture},
    pin::Pin,
    task::{Context, Poll},
};

use pin_project_lite::pin_project;

use crate::common::{CliConfig, CliError};

pub fn trigger_webhook_event(forward_address: &str, secret: &str) -> Trigger {
    Trigger::new(CliConfig::webhook_trigger())
        .with_forward_address(forward_address)
        .with_secret(secret)
}

pub fn trigger_websocket_event(session: &str) -> Trigger {
    Trigger::new(CliConfig::webhook_trigger())
        .with_transport("websocket")
        .with_session(session)
}

#[derive(Debug, Clone)]
pub struct Trigger {
    config: CliConfig,
    // webhook
    forward_address: Option<String>,
    secret: Option<String>,
    verify: bool,
    item_id: Option<String>,
    // websocket
    transport: Option<String>,
    session: Option<String>,

    // log
    output_file: Option<String>,
    append_output: bool,
    include_stderr: bool,
}

impl Trigger {
    pub fn new(config: CliConfig) -> Self {
        Trigger {
            config,
            forward_address: None,
            secret: None,
            verify: false,
            item_id: None,
            transport: None,
            session: None,
            output_file: None,
            append_output: true,
            include_stderr: false,
        }
    }

    pub fn with_forward_address(mut self, forward_address: impl Into<String>) -> Self {
        self.forward_address = Some(forward_address.into());
        self
    }

    pub fn with_secret(mut self, secret: impl Into<String>) -> Self {
        self.secret = Some(secret.into());
        self
    }

    pub fn with_transport(mut self, transport: impl Into<String>) -> Self {
        self.transport = Some(transport.into());
        self
    }

    pub fn with_session(mut self, session: impl Into<String>) -> Self {
        self.session = Some(session.into());
        self
    }

    pub fn with_verify(mut self) -> Self {
        self.verify = true;
        self
    }

    pub fn with_output_file(mut self, path: impl Into<String>) -> Self {
        self.output_file = Some(path.into());
        self
    }

    pub fn append_output(mut self, append: bool) -> Self {
        self.append_output = append;
        self
    }

    pub fn include_stderr(mut self, include: bool) -> Self {
        self.include_stderr = include;
        self
    }

    pub fn event(&self, event: impl Into<String>) -> TriggerExecution {
        TriggerExecution {
            trigger: self.clone(),
            event: event.into(),
            version: None,
        }
    }
}

pub struct TriggerExecution {
    trigger: Trigger,
    event: String,
    version: Option<u8>,
}

impl TriggerExecution {
    pub fn version(mut self, version: u8) -> Self {
        self.version = Some(version);
        self
    }
}

impl IntoFuture for TriggerExecution {
    type Output = Result<(), CliError>;
    type IntoFuture = TriggerFuture;

    fn into_future(self) -> Self::IntoFuture {
        TriggerFuture {
            future: Box::pin(async move {
                let Self {
                    trigger,
                    event,
                    version,
                } = self;

                let Trigger {
                    config: mut child,
                    forward_address,
                    secret,
                    verify,
                    item_id: _,
                    transport,
                    session,
                    output_file,
                    append_output,
                    include_stderr,
                } = trigger;

                child.add_arg("event");

                if verify {
                    child.add_args(["verify-subscription", &event]);
                } else {
                    child.add_args(["trigger", &event]);
                }

                if let Some(forward_address) = &forward_address {
                    child.add_args(["-F", forward_address]);
                }

                if let Some(secret) = &secret {
                    child.add_args(["-s", secret]);
                }

                if let Some(transport) = &transport {
                    child.add_arg(format!("--transport={}", transport));
                }

                if let Some(session) = &session {
                    child.add_args(["-s", session]);
                }

                if let Some(version) = version {
                    child.add_args(["-v", &version.to_string()]);
                }

                let mut process = child.command();
                let output = process.output().await?;

                if let Some(output_path) = output_file {
                    write_output(&output, &output_path, &event, include_stderr, append_output)?;
                }

                Ok(())
            }),
        }
    }
}

pin_project! {
    pub struct TriggerFuture {
        #[pin]
        future: Pin<Box<dyn Future<Output = Result<(), CliError>>>>
    }
}

impl Future for TriggerFuture {
    type Output = Result<(), CliError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.project().future.poll(cx)
    }
}

fn write_output(
    output: &std::process::Output,
    path: &str,
    event: &str,
    include_stderr: bool,
    append_output: bool,
) -> std::io::Result<()> {
    use std::io::Write;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let mut content = format!(
        "=== Event: {} ===\nExit Code: {:?}\n\n--- STDOUT ---\n{}",
        event,
        output.status.code(),
        stdout
    );

    if include_stderr && !stderr.is_empty() {
        content.push_str(&format!("\n--- STDERR ---\n{}", stderr));
    }

    content.push_str(&format!("\n{}\n\n", "=".repeat(80)));

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(append_output)
        .truncate(!append_output)
        .open(path)?;

    file.write_all(content.as_bytes())
}
