use serde::Serialize;

use crate::types::ExtensionId;

define_request!(
    #[derive(Debug, Clone, Serialize)]
    RequiredConfiguration<'a> {
        req: {
            extension_id: &'a ExtensionId,
            extension_version: &'a str,
            required_configuration: &'a str
        };
        into_json
    }
);

define_request!(
    #[derive(Debug, Clone, Serialize)]
    ExtensionChatMessageIntoRequestBody<'a> {
        req: {
            text: &'a str,
            extension_id: &'a ExtensionId,
            extension_version: &'a str
        };
        into_json
    }
);
