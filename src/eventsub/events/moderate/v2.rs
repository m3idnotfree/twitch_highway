use serde::{Deserialize, Serialize};

use crate::types::{BroadcasterId, ModeratorId, UserId};

use crate::eventsub::events::moderate::{
    AutomodTermsModeration, BanModeration, DeleteModeration, FollowersModeration, ModModeration,
    RaidModeration, SlowModeration, TimeoutModeration, UnbanModeration, UnbanRequestModeration,
    UnmodModeration, UnraidModeration, UntimeoutModeration, UnvipModeration, VipModeration,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelModerate {
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub source_broadcaster_user_id: BroadcasterId,
    pub source_broadcaster_user_login: String,
    pub soruce_broadcaster_user_name: String,
    pub moderator_user_id: ModeratorId,
    pub moderator_user_login: String,
    pub moderator_user_name: String,
    pub action: Action,
    pub followers: Option<FollowersModeration>,
    pub slow: Option<SlowModeration>,
    pub vip: Option<VipModeration>,
    pub unvip: Option<UnvipModeration>,
    #[serde(rename = "mod")]
    pub mod_: Option<ModModeration>,
    pub unmod: Option<UnmodModeration>,
    pub ban: Option<BanModeration>,
    pub unban: Option<UnbanModeration>,
    pub timeout: Option<TimeoutModeration>,
    pub untimeout: Option<UntimeoutModeration>,
    pub raid: Option<RaidModeration>,
    pub unraid: Option<UnraidModeration>,
    pub delete: Option<DeleteModeration>,
    pub automod_terms: Option<AutomodTermsModeration>,
    pub unban_request: Option<UnbanRequestModeration>,
    pub warn: Option<WarnModeration>,

    pub shared_chat_ban: Option<String>,
    pub shared_chat_unban: Option<String>,
    pub shared_chat_timeout: Option<String>,
    pub shared_chat_untimeout: Option<String>,
    pub shared_chat_delete: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarnModeration {
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    pub reason: Option<String>,
    pub chat_rules_cited: Option<Vec<String>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    Ban,
    Timeout,
    Unban,
    Untimeout,
    Clear,
    Emoteonly,
    EmoteonlyOff,
    Followers,
    FollowersOff,
    UniqueChat,
    UniquechatOff,
    Slow,
    SlowOff,
    Subscribers,
    SubscribersOff,
    Unraid,
    Delete,
    Unvip,
    Vip,
    Raid,
    #[serde(rename = "add_blocked_term")]
    AddBlockedTerm,
    #[serde(rename = "add_permitted_term")]
    AddPermittedTerm,
    #[serde(rename = "remove_blocked_term")]
    RemoveBlockedTerm,
    #[serde(rename = "remove_permitted_term")]
    RemovePermittedTerm,
    Mod,
    Unmod,
    #[serde(rename = "approve_unban_request")]
    ApproveUnbanRequest,
    #[serde(rename = "deny_unban_request")]
    DenyUnbanRequest,
    Warn,
    #[serde(rename = "shared_chat_ban")]
    SharedChatBan,
    #[serde(rename = "shared_chat_timeout")]
    SharedChatTimeout,
    #[serde(rename = "shared_chat_untimeout")]
    SharedChatUntimeout,
    #[serde(rename = "shared_chat_unban")]
    SharedChatUnban,
    #[serde(rename = "shared_chat_delete")]
    SharedChatDelete,
}
