use serde::{Deserialize, Serialize};

use crate::types::{BroadcasterId, UserId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelChatClear {
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelChatClearUserMessages {
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub target_user_id: UserId,
    pub target_user_login: String,
    pub target_user_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelChatMessage {
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_name: String,
    pub broadcaster_user_login: String,
    pub chatter_user_id: UserId,
    pub chatter_user_name: String,
    pub chatter_user_login: String,
    pub message_id: String,
    pub message: Message,
    pub message_type: String,
    pub badges: Vec<Badge>,
    pub cheer: Option<Cheer>,
    pub color: String,
    pub reply: Option<Reply>,
    pub channel_points_custom_reward_id: Option<String>,

    pub source_broadcaster_user_id: Option<BroadcasterId>,
    pub source_broadcaster_user_name: Option<String>,
    pub source_broadcaster_user_login: Option<String>,
    pub source_message_id: Option<String>,
    pub source_badges: Option<Badge>,
    pub is_source_only: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub text: String,
    pub fragments: Vec<Fragment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fragment {
    #[serde(rename = "type")]
    pub kind: FragmentType,
    pub text: String,
    pub cheermote: Option<Cheermote>,
    pub emote: Option<Emote>,
    pub mention: Option<Mention>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FragmentType {
    Text,
    Cheermote,
    Emote,
    Mention,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cheermote {
    pub prefix: String,
    pub bits: u64,
    pub tier: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Emote {
    pub id: String,
    pub emote_set_id: String,
    pub owner_id: UserId,
    pub format: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mention {
    pub user_id: UserId,
    pub user_name: String,
    pub user_login: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Badge {
    pub set_id: String,
    pub id: String,
    pub info: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cheer {
    pub bits: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reply {
    pub parent_message_id: String,
    pub parent_message_body: String,
    pub parent_user_id: UserId,
    pub parent_user_name: String,
    pub parent_user_login: String,
    pub thread_message_id: String,
    pub thread_user_id: UserId,
    pub thread_user_name: String,
    pub thread_user_login: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelChatMessageDelete {
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_name: String,
    pub broadcaster_user_login: String,
    pub target_user_id: UserId,
    pub target_user_name: String,
    pub target_user_login: String,
    pub message_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelChatNotification {
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_name: String,
    pub broadcaster_user_login: String,
    pub chatter_user_id: UserId,
    pub chatter_user_login: String,
    pub chatter_user_name: String,
    pub chatter_is_anonymous: bool,
    pub color: String,
    pub badges: Vec<Badge>,
    pub system_message: String,
    pub message_id: String,
    pub message: Message,
    pub notice_type: NoticeType,
    pub sub: Option<SubNotification>,
    pub resub: Option<ResubNotification>,
    pub sub_gift: Option<SubGiftNotification>,
    pub community_sub_gift: Option<CommunitySubGiftNotification>,
    pub gift_paid_upgrade: Option<GiftPaidUpgradeNotification>,
    pub prime_paid_upgrade: Option<PrimePaidUpgradeNotification>,
    pub pay_it_forward: Option<PayItForwardNotification>,
    pub raid: Option<RaidNotification>,
    pub unraid: Option<UnraidNotification>,
    pub announcement: Option<AnnouncementNotification>,
    pub bits_badge_tier: Option<BitsBadgeTierNotification>,
    pub charity_donation: Option<CharityDonationNotification>,

    pub source_broadcaster_user_id: Option<BroadcasterId>,
    pub source_broadcaster_user_name: Option<String>,
    pub source_broadcaster_user_login: Option<String>,
    pub source_message_id: Option<String>,
    pub source_badges: Option<Badge>,

    pub shared_chat_sub: Option<SubGiftNotification>,
    pub shared_chat_resub: Option<ResubNotification>,
    pub shared_chat_sub_gift: Option<SubGiftNotification>,
    pub shared_chat_community_sub_gift: Option<CommunitySubGiftNotification>,
    pub shared_chat_gift_paid_upgrade: Option<GiftPaidUpgradeNotification>,
    pub shared_chat_prime_paid_upgrade: Option<PrimePaidUpgradeNotification>,
    pub shared_chat_pay_it_forward: Option<PayItForwardNotification>,
    pub shared_chat_raid: Option<RaidNotification>,
    pub shared_chat_announcement: Option<AnnouncementNotification>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NoticeType {
    Sub,
    Resub,
    SubGift,
    CommunitySubGift,
    GiftPaidUpgrade,
    PrimePaidUpgrade,
    Raid,
    Unraid,
    PayItForward,
    Announcement,
    BitsBadgeTier,
    CharityDonation,
    SharedChatSub,
    SharedChatResub,
    SharedChatSubGift,
    SharedChatCommunitySubGift,
    SharedChatGiftPaidUpgrade,
    SharedChatPrimePaidUpgrade,
    SharedChatRaid,
    SharedChatPayItForward,
    SharedChatAnnouncement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubNotification {
    pub sub_tier: String,
    pub is_prime: bool,
    pub duration_months: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResubNotification {
    pub cumulative_months: u32,
    pub duration_months: u32,
    pub streak_months: Option<u32>,
    pub sub_tier: String,
    pub is_prime: Option<bool>,
    pub is_gift: bool,
    pub gifter_is_anonymous: Option<bool>,
    pub gifter_user_id: Option<UserId>,
    pub gifter_user_name: Option<String>,
    pub gifter_user_login: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubGiftNotification {
    pub duration_months: u32,
    pub cumulative_total: Option<u32>,
    pub recipient_user_id: UserId,
    pub recipient_user_name: String,
    pub recipient_user_login: String,
    pub sub_tier: String,
    pub community_gift_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunitySubGiftNotification {
    pub id: String,
    pub total: u32,
    pub sub_tier: String,
    pub cumulative_total: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiftPaidUpgradeNotification {
    pub gifter_is_anonymous: bool,
    pub gifter_user_id: Option<UserId>,
    pub gifter_user_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimePaidUpgradeNotification {
    pub sub_tier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaidNotification {
    pub user_id: UserId,
    pub user_name: String,
    pub user_login: String,
    pub viewer_count: u32,
    pub profile_image_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnraidNotification {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayItForwardNotification {
    pub gifter_is_anonymous: bool,
    pub gifter_user_id: Option<UserId>,
    pub gifter_user_name: Option<String>,
    pub gifter_user_login: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnouncementNotification {
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitsBadgeTierNotification {
    pub tier: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharityDonationNotification {
    pub charity_name: String,
    pub amount: Amount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Amount {
    pub value: u64,
    pub decimal_places: u32,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelChatSettingsUpdate {
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub emote_mode: bool,
    pub follower_mode: bool,
    pub follower_mode_duration_minutes: Option<u32>,
    pub slow_mode: bool,
    pub slow_mode_wait_time_seconds: Option<u32>,
    pub subscriber_mode: bool,
    pub unique_chat_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelChatUserMessageHold {
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    pub message_id: String,
    pub message: Message,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageHoldMessage {
    pub text: String,
    pub fragments: Vec<MessageHoldFragement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageHoldFragement {
    pub text: String,
    pub emote: Option<MessageHoldEmote>,
    pub cheermote: Option<Cheermote>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageHoldEmote {
    pub id: String,
    pub emote_set_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelChatUserMessageUpdate {
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    pub message_id: String,
    pub status: UserMessageUpdateStatus,
    pub message: MessageHoldMessage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UserMessageUpdateStatus {
    Approved,
    Denied,
    Invalid,
}
