use asknothingx2_util::api::APIRequest;
use serde::{Deserialize, Serialize};
use url::Url;

crate::impl_endpoint!(
/// https://dev.twitch.tv/docs/api/reference/#send-chat-message
/// Requires an app access token or user access token that includes the user:write:chat scope.
/// If app access token used,
/// then additionally requires user:bot scope from chatting user,
/// and either channel:bot scope from broadcaster or moderator status.
    SendChatMessage {
    broadcaster_id: String,
    sender_id: String,
     /// The message is limited to a maximum of 500 characters.
     /// Chat messages can also include emoticons.
     /// To include emoticons,
     /// use the name of the emote.
     /// The names are case sensitive.
     /// Don’t include colons around the name (e.g., :bleedPurple:).
     /// If Twitch recognizes the name,
     /// Twitch converts the name to the emote before writing the chat message to the chat room
    message: String,
    };
    new = {
        params = {
            broadcaster_id: impl Into<String>,
            sender_id: impl Into<String>
        },
        init = {
            broadcaster_id: broadcaster_id.into(),
            sender_id: sender_id.into(),
            message: "".to_string()
        }
    },
    url = ["chat","messages"]
);

impl SendChatMessage {
    pub fn set_message<T: Into<String>>(&mut self, message: T) {
        self.message = message.into();
    }

    pub fn set_broadcaster_id<T: Into<String>>(&mut self, broadcaster_id: T) {
        self.broadcaster_id = broadcaster_id.into();
    }

    pub fn set_sender_id<T: Into<String>>(&mut self, sender_id: T) {
        self.sender_id = sender_id.into();
    }
}

#[derive(Debug, Serialize)]
pub struct SendChatMessageRequest {
    broadcaster_id: String,
    sender_id: String,
    message: String,
}

impl SendChatMessageRequest {
    pub fn new(broadcaster_id: String, sender_id: String, message: String) -> Self {
        Self {
            broadcaster_id,
            sender_id,
            message,
        }
    }
}

impl APIRequest for SendChatMessage {
    crate::impl_api_request_method!(POST);
    crate::impl_api_request_header!(json);

    fn json(&self) -> Option<String> {
        Some(
            serde_json::to_string(&SendChatMessageRequest::new(
                self.broadcaster_id.clone(),
                self.sender_id.clone(),
                self.message.clone(),
            ))
            .unwrap(),
        )
    }

    fn url(&self) -> Url {
        self.get_url()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropReason {
    code: String,
    message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageResponse {
    message_id: String,
    is_sent: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    drop_reason: Option<DropReason>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendChatMessageResponse {
    data: Vec<MessageResponse>,
}

#[cfg(test)]
mod tests {
    use crate::{
        api_general, expect_APIRequest, expect_headers, expect_response_json,
        send_chat_message::SendChatMessageResponse,
    };

    use super::SendChatMessage;

    #[test]
    fn send_chat_message() {
        let mut send_chat_message = api_general!(SendChatMessage, "12826", "141981764");
        send_chat_message.set_message("Hello, world! twitchdevHype");

        expect_APIRequest!(
            POST,
            expect_headers!(json),
            "https://api.twitch.tv/helix/chat/messages",
            json = Some("{\"broadcaster_id\":\"12826\",\"sender_id\":\"141981764\",\"message\":\"Hello, world! twitchdevHype\"}".to_string()),
            text = None,
            urlencoded= None,
            send_chat_message
        );
    }

    #[test]
    fn send_chat_message_response() {
        expect_response_json!("{\n  \"data\": [\n    {\n      \"message_id\": \"abc-123-def\",\n      \"is_sent\": true\n    }\n  ]\n}",
        SendChatMessageResponse);
    }
}
