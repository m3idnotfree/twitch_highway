use serde::{Deserialize, Serialize};

use crate::types::{BroadcasterId, ExtensionId};

define_request!(
    #[derive(Serialize, Deserialize)]
    SetConfigurationSegment {
        opts: {
            #[serde(skip_serializing_if = "Option::is_none")]
            broadcaster_id: BroadcasterId,
            #[serde(skip_serializing_if = "Option::is_none")]
            content: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            version: String
        };
        into_request_body
    }
);

define_request!(
    #[derive(Serialize, Deserialize)]
    RequiredConfiguration {
        req: {
            extension_id: ExtensionId,
            extension_version: String,
            required_configuration: String
        };
        into_request_body
    }
);

define_request!(
    #[derive(Serialize, Deserialize)]
    ExtensionChatMessageIntoRequestBody {
        req: {
            text: String,
            extension_id: ExtensionId,
            extension_version: String
        };
        into_request_body
    }
);

define_request!(
    #[derive(Serialize, Deserialize)]
    UpdateExtensoinBitsProductsRequest {
        opts: {
            #[serde(skip_serializing_if = "Option::is_none")]
            expiration: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            in_development: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_broadcast: bool,
        }
    }
);
