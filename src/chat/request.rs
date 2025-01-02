use serde::Serialize;

use crate::AsBody;

#[derive(Debug, Serialize)]
pub struct SendChatMessageRequest {
    broadcaster_id: String,
    sender_id: String,
    message: String,
}

impl SendChatMessageRequest {
    pub fn new<T: Into<String>>(broadcaster_id: T, sender_id: T, message: T) -> Self {
        Self {
            broadcaster_id: broadcaster_id.into(),
            sender_id: sender_id.into(),
            message: message.into(),
        }
    }
}

impl AsBody for SendChatMessageRequest {
    fn as_body(&self) -> Option<String> {
        Some(serde_json::to_string(self).unwrap())
    }
}

/// https://dev.twitch.tv/docs/api/reference/#update-chat-settings
#[derive(Debug, Default, Serialize)]
pub struct UpdateChatSettingsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    emote_mode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    follower_mode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    follower_mode_duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    non_moderator_chat_delay: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    non_moderator_chat_delay_duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slow_mode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slow_mode_wait_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscriber_mode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unique_chat_mode: Option<bool>,
}

impl UpdateChatSettingsRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_emote_mode(mut self, mode: bool) -> Self {
        self.emote_mode = Some(mode);
        self
    }
    pub fn set_follower_mode(mut self, mode: bool) -> Self {
        self.follower_mode = Some(mode);
        self
    }
    pub fn set_follower_mode_duration(mut self, duration: u64) -> Self {
        self.follower_mode_duration = Some(duration);
        self
    }
    pub fn set_non_moderator_chat_delay(mut self, mode: bool) -> Self {
        self.non_moderator_chat_delay = Some(mode);
        self
    }
    pub fn set_non_moderator_chat_delay_duration(mut self, duration: u64) -> Self {
        self.non_moderator_chat_delay_duration = Some(duration);
        self
    }
    pub fn set_slow_mode(mut self, mode: bool) -> Self {
        self.slow_mode = Some(mode);
        self
    }
    pub fn set_slow_mode_wait_time(mut self, wait_time: u64) -> Self {
        self.slow_mode_wait_time = Some(wait_time);
        self
    }
    pub fn set_subscriber_mode(mut self, mode: bool) -> Self {
        self.subscriber_mode = Some(mode);
        self
    }
    pub fn set_unique_chat_mode(mut self, mode: bool) -> Self {
        self.unique_chat_mode = Some(mode);
        self
    }
}

impl AsBody for UpdateChatSettingsRequest {
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
