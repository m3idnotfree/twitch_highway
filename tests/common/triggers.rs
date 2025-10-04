#![allow(dead_code)]

use std::{
    future::{Future, IntoFuture},
    pin::Pin,
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
    item_id: Option<String>,
    // websocket
    transport: Option<String>,
    session: Option<String>,
}

impl Trigger {
    pub fn new(config: CliConfig) -> Self {
        Trigger {
            config,
            forward_address: None,
            secret: None,
            item_id: None,
            transport: None,
            session: None,
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

    pub fn event(&self, event: impl Into<String>) -> TriggerExecution {
        TriggerExecution {
            trigger: self.clone(),
            event: event.into(),
        }
    }
}

pub struct TriggerExecution {
    trigger: Trigger,
    event: String,
}

impl IntoFuture for TriggerExecution {
    type Output = Result<(), CliError>;
    type IntoFuture = TriggerFuture;

    fn into_future(self) -> Self::IntoFuture {
        TriggerFuture {
            future: Box::pin(async move {
                let mut child = self.trigger.config;
                child.add_args(["event", "trigger", &self.event]);

                if let Some(forward_address) = &self.trigger.forward_address {
                    child.add_args(["-F".to_string(), forward_address.clone()]);
                }

                if let Some(secret) = &self.trigger.secret {
                    child.add_args(["-s".to_string(), secret.clone()]);
                }

                if let Some(transport) = &self.trigger.transport {
                    let f = format!("--transport={}", transport);
                    child.add_arg(f);
                }

                if let Some(session) = &self.trigger.session {
                    child.add_args(["-s".to_string(), session.clone()]);
                }

                let mut child = child.spawn()?;

                child.wait().await?;
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

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        self.project().future.poll(cx)
    }
}
