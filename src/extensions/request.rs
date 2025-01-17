use serde::{Deserialize, Serialize};

use crate::{
    types::{BroadcasterId, ExtensionId},
    IntoRequestBody,
};

new_request_struct!(
    #[derive(Serialize, Deserialize)]
    SetConfigurationSegment {
        string: {
            #[serde(skip_serializing_if = "Option::is_none")]
            content: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            version: String
        },
        any: {
            #[serde(skip_serializing_if = "Option::is_none")]
            broadcaster_id: BroadcasterId,

        }
    }
);

impl IntoRequestBody for SetConfigurationSegment {
    fn as_body(&self) -> Option<String> {
        Some(serde_json::to_string(&self).unwrap())
    }
}

request_struct!(
    #[derive(Serialize, Deserialize)]
    RequiredConfiguration {
        required {
            extension_id: ExtensionId,
            extension_version: String,
            required_configuration: String
        }
    }
);

impl IntoRequestBody for RequiredConfiguration {
    fn as_body(&self) -> Option<String> {
        Some(serde_json::to_string(&self).unwrap())
    }
}

request_struct!(
#[derive(Serialize, Deserialize)]
ExtensionChatMessageIntoRequestBody {
    required {
        text: String,
        extension_id: ExtensionId,
        extension_version: String
    }
}
);
impl IntoRequestBody for ExtensionChatMessageIntoRequestBody {
    fn as_body(&self) -> Option<String> {
        Some(serde_json::to_string(&self).unwrap())
    }
}

new_request_struct!(
    #[derive(Serialize, Deserialize)]
    UpdateExtensoinBitsProductsRequest {
        string: {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub expiration: String,
        },
        any: {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub in_development: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub is_broadcast: bool,
        }
    }
);
