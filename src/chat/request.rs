use serde::Serialize;

use crate::{types::BroadcasterId, RequestBody};

request_struct!(
    #[derive(Debug, Serialize)]
    SendChatMessageRequest {
        required {
            broadcaster_id: BroadcasterId,
            sender_id: String,
            message: String,
        }
    }
);

impl RequestBody for SendChatMessageRequest {
    fn as_body(&self) -> Option<String> {
        Some(serde_json::to_string(self).unwrap())
    }
}

request_struct!(
    /// https://dev.twitch.tv/docs/api/reference/#update-chat-settings
    #[derive(Debug, Default, Serialize)]
    UpdateChatSettingsRequest {
        #[serde(skip_serializing_if = "Option::is_none")]
        emote_mode: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        follower_mode: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        follower_mode_duration: u64,
        #[serde(skip_serializing_if = "Option::is_none")]
        non_moderator_chat_delay: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        non_moderator_chat_delay_duration: u64,
        #[serde(skip_serializing_if = "Option::is_none")]
        slow_mode: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        slow_mode_wait_time: u64,
        #[serde(skip_serializing_if = "Option::is_none")]
        subscriber_mode: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        unique_chat_mode: bool,
    }
);

impl RequestBody for UpdateChatSettingsRequest {
    fn as_body(&self) -> Option<String> {
        Some(serde_json::to_string(self).unwrap())
    }
}

#[derive(Debug, Serialize)]
pub enum ChatColor {
    Blue,
    BlueViolet,
    CadetBlue,
    Chocolate,
    Coral,
    DodgerBlue,
    Firebrick,
    GoldenRod,
    Green,
    HotPink,
    OrangeRed,
    Red,
    SeaGreen,
    SpringGreen,
    YellowGreen,
}

impl ChatColor {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Blue => "blue",
            Self::BlueViolet => "blue_violet",
            Self::CadetBlue => "cadet_blue",
            Self::Chocolate => "chocolate",
            Self::Coral => "coral",
            Self::DodgerBlue => "dodger_blue",
            Self::Firebrick => "firebrick",
            Self::GoldenRod => "golden_rod",
            Self::Green => "green",
            Self::HotPink => "hot_pink",
            Self::OrangeRed => "orange_red",
            Self::Red => "red",
            Self::SeaGreen => "sea_green",
            Self::SpringGreen => "spring_green",
            Self::YellowGreen => "yellow_green",
        }
    }
}
