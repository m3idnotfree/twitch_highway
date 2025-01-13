use serde::{Deserialize, Serialize};

use crate::{
    types::{BroadcasterId, ExtensionId},
    RequestBody,
};

use super::types::Segment;

request_struct!(
#[derive(Debug, Serialize, Deserialize)]
SetConfigurationSegment {
    required {
        extension_id: ExtensionId,
        segment: Segment
    },
    optional {
        #[serde(skip_serializing_if = "Option::is_none")]
        broadcaster_id: BroadcasterId,
        #[serde(skip_serializing_if = "Option::is_none")]
        content: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        version: String
    }
}
);

impl RequestBody for SetConfigurationSegment {
    fn as_body(&self) -> Option<String> {
        Some(serde_json::to_string(&self).unwrap())
    }
}

request_struct!(
#[derive(Debug, Serialize, Deserialize)]
RequiredConfiguration {
    required {
        extension_id: ExtensionId,
        extension_version: String,
        required_configuration: String
    }
}
);

impl RequestBody for RequiredConfiguration {
    fn as_body(&self) -> Option<String> {
        Some(serde_json::to_string(&self).unwrap())
    }
}

request_struct!(
#[derive(Debug, Serialize, Deserialize)]
PubSubMessageRequest {
    required {
        target: Vec<String>,
        message: String,
        broadcaster_id: BroadcasterId
    },
    optional {
        #[serde(skip_serializing_if = "Option::is_none")]
        is_global_broadcast: bool,

    }
}
);

impl RequestBody for PubSubMessageRequest {
    fn as_body(&self) -> Option<String> {
        Some(serde_json::to_string(&self).unwrap())
    }
}

request_struct!(
#[derive(Debug, Serialize, Deserialize)]
ExtensionChatMessageRequestBody {
    required {
        text: String,
        extension_id: String,
        extension_version: String
    }
}
);
impl RequestBody for ExtensionChatMessageRequestBody {
    fn as_body(&self) -> Option<String> {
        Some(serde_json::to_string(&self).unwrap())
    }
}
