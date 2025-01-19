use serde::{Deserialize, Serialize};

use crate::types::{BroadcasterId, ExtensionId};

request_struct!(
    #[derive(Serialize, Deserialize)]
    SetConfigurationSegment {
        #[serde(skip_serializing_if = "Option::is_none")]
        broadcaster_id: BroadcasterId,
        #[serde(skip_serializing_if = "Option::is_none")]
        content: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        version: String
    };
    impl_body: true
);

request_struct!(
    #[derive(Serialize, Deserialize)]
    RequiredConfiguration {
        required {
            extension_id: ExtensionId,
            extension_version: String,
            required_configuration: String
        }
    };
    impl_body: true
);

request_struct!(
    #[derive(Serialize, Deserialize)]
    ExtensionChatMessageIntoRequestBody {
        required {
            text: String,
            extension_id: ExtensionId,
            extension_version: String
        }
    };
    impl_body: true
);

request_struct!(
    #[derive(Serialize, Deserialize)]
    UpdateExtensoinBitsProductsRequest {
        string {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub expiration: String,
        },
        any {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub in_development: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub is_broadcast: bool,
        }
    }
);
