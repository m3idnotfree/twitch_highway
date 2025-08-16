use serde::{Deserialize, Serialize};

use crate::types::{BroadcasterId, ExtensionId};

define_request!(
    #[derive(Serialize, Deserialize)]
    SetConfigurationSegment<'a> {
        opts: {
            #[serde(skip_serializing_if = "Option::is_none")]
            broadcaster_id: BroadcasterId,
            #[serde(skip_serializing_if = "Option::is_none")]
            content: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            version: &'a str
        };
        into_json
    }
);

define_request!(
    #[derive(Serialize, Deserialize)]
    RequiredConfiguration<'a> {
        req: {
            extension_id: ExtensionId,
            extension_version: &'a str,
            required_configuration: &'a str
        };
        into_json
    }
);

define_request!(
    #[derive(Serialize, Deserialize)]
    ExtensionChatMessageIntoRequestBody<'a> {
        req: {
            text: &'a str,
            extension_id: ExtensionId,
            extension_version: &'a str
        };
        into_json
    }
);

define_request!(
    #[derive(Serialize, Deserialize)]
    UpdateExtensoinBitsProductsRequest<'a> {
        opts: {
            #[serde(skip_serializing_if = "Option::is_none")]
            expiration: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            in_development: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_broadcast: bool,
        }
    }
);
