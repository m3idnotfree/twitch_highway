use serde::Serialize;

use crate::types::BroadcasterId;

request_struct!(
    #[derive(Serialize)]
    SendChatMessageRequest {
        required {
            broadcaster_id: BroadcasterId,
            sender_id: String,
            message: String
        }
    };
    impl_body: true
);

request_struct!(
    /// <https://dev.twitch.tv/docs/api/reference/#update-chat-settings>
    #[derive(Serialize)]
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
    };
    impl_body: true
);

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

request_struct!(
    #[derive(Serialize)]
    ChatAnnouncementBody {
        required {
            message: String,
            color: Option<AnnouncementColor>
        }
    };
    impl_body: true
);

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AnnouncementColor {
    Blue,
    Green,
    Orange,
    Purple,
    Primary,
}

impl AnnouncementColor {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Blue => "blue",
            Self::Green => "green",
            Self::Orange => "orange",
            Self::Purple => "purple",
            Self::Primary => "primary",
        }
    }
}
