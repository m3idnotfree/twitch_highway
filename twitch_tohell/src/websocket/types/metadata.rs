use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    str,
};

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize, de::Error as DeError};

use crate::{MessageId, SubscriptionType, websocket::MessageType};

/// <https://dev.twitch.tv/docs/eventsub/websocket-reference>
#[derive(Debug, Clone, Serialize)]
pub struct MetaData {
    pub message_id: MessageId,
    pub message_type: MessageType,
    pub message_timestamp: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_type: Option<SubscriptionType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_version: Option<String>,
}

impl Display for MetaData {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "[{}] {} at {}",
            self.message_id,
            self.message_type,
            self.message_timestamp.format("%Y-%m-%d %H:%M:%S")
        )?;

        if let Some(sub_type) = &self.subscription_type {
            write!(f, "({sub_type})")?;
        }

        if let Some(version) = &self.subscription_version {
            write!(f, " v{version}")?;
        }

        Ok(())
    }
}

impl<'de> Deserialize<'de> for MetaData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            message_id: MessageId,
            message_type: MessageType,
            message_timestamp: DateTime<FixedOffset>,
            subscription_type: Option<String>,
            subscription_version: Option<String>,
        }

        let helper = Helper::deserialize(deserializer)?;

        let subscription_type = helper
            .subscription_type
            .map(|kind| {
                SubscriptionType::from_type_and_version(
                    &kind,
                    helper.subscription_version.as_deref(),
                )
                .map_err(DeError::custom)
            })
            .transpose()?;

        Ok(MetaData {
            message_id: helper.message_id,
            message_type: helper.message_type,
            message_timestamp: helper.message_timestamp,
            subscription_type,
            subscription_version: helper.subscription_version,
        })
    }
}
