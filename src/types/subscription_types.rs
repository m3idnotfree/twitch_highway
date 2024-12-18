#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubscriptionTypes {
    // Automod related
    AutomodMessageHold,
    AutomodMessageHoldV2,
    AutomodMessageUpdate,
    AutomodMessageUpdateV2,
    AutomodSettingsUpdate,
    AutomodTermsUpdate,

    // Channel related
    ChannelUpdate,
    ChannelFollow,
    ChannelAdBreakBegin,

    // Chat related
    ChannelChatClear,
    ChannelChatClearUserMessages,
    ChannelChatMessage,
    ChannelChatMessageDelete,
    ChannelChatNotification,
    ChannelChatSettingsUpdate,
    ChannelChatUserMessageHold,
    ChannelChatUserMessageUpdate,

    // Shared Chat
    ChannelSharedChatSessionBegin,
    ChannelSharedChatSessionUpdate,
    ChannelSharedChatSessionEnd,

    // Subscription related
    ChannelSubscribe,
    ChannelSubscriptionEnd,
    ChannelSubscriptionGift,
    ChannelSubscriptionMessage,

    // Channel interactions
    ChannelCheer,
    ChannelRaid,

    // Moderation
    ChannelBan,
    ChannelUnban,
    ChannelUnbanRequestCreate,
    ChannelUnbanRequestResolve,
    ChannelModerate,
    ChannelModerateV2,
    ChannelModeratorAdd,
    ChannelModeratorRemove,

    // Guest Star (Beta)
    ChannelGuestStarSessionBegin,
    ChannelGuestStarSessionEnd,
    ChannelGuestStarGuestUpdate,
    ChannelGuestStarSettingsUpdate,

    // Channel Points
    ChannelPointsAutomaticRewardRedemption,
    ChannelPointsCustomRewardAdd,
    ChannelPointsCustomRewardUpdate,
    ChannelPointsCustomRewardRemove,
    ChannelPointsCustomRewardRedemptionAdd,
    ChannelPointsCustomRewardRedemptionUpdate,

    // Polls and Predictions
    ChannelPollBegin,
    ChannelPollProgress,
    ChannelPollEnd,
    ChannelPredictionBegin,
    ChannelPredictionProgress,
    ChannelPredictionLock,
    ChannelPredictionEnd,

    // Suspicious Users
    ChannelSuspiciousUserMessage,
    ChannelSuspiciousUserUpdate,

    // VIP and Warnings
    ChannelVIPAdd,
    ChannelVIPRemove,
    ChannelWarningAcknowledgement,
    ChannelWarningSend,

    // Charity
    CharityDonation,
    CharityCampaignStart,
    CharityCampaignProgress,
    CharityCampaignStop,

    // Conduit
    ConduitShardDisabled,

    // Drops and Extensions
    DropEntitlementGrant,
    ExtensionBitsTransactionCreate,

    // Goals and Hype Train
    GoalBegin,
    GoalProgress,
    GoalEnd,
    HypeTrainBegin,
    HypeTrainProgress,
    HypeTrainEnd,

    // Shield Mode
    ShieldModeBegin,
    ShieldModeEnd,

    // Shoutouts
    ShoutoutCreate,
    ShoutoutReceived,

    // Stream Status
    StreamOnline,
    StreamOffline,

    // User Authorization
    UserAuthorizationGrant,
    UserAuthorizationRevoke,
    UserUpdate,

    // Whispers
    WhisperReceived,
}

impl SubscriptionTypes {
    pub fn as_str(&self) -> &str {
        match self {
            Self::AutomodMessageHold => "automod.message.hold",
            Self::AutomodMessageHoldV2 => "automod.message.hold",
            Self::AutomodMessageUpdate => "automod.message.update",
            Self::AutomodMessageUpdateV2 => "automod.message.update",
            Self::AutomodSettingsUpdate => "automod.settings.update",
            Self::AutomodTermsUpdate => "automod.terms.update",
            Self::ChannelUpdate => "channel.update",
            Self::ChannelFollow => "channel.follow",
            Self::ChannelAdBreakBegin => "channel.ad_break.begin",
            Self::ChannelChatClear => "channel.chat.clear",
            Self::ChannelChatClearUserMessages => "channel.chat.clear_user_messages",
            Self::ChannelChatMessage => "channel.chat.message",
            Self::ChannelChatMessageDelete => "channel.chat.message_delete",
            Self::ChannelChatNotification => "channel.chat.notification",
            Self::ChannelChatSettingsUpdate => "channel.chat_settings.update",
            Self::ChannelChatUserMessageHold => "channel.chat.user_message_hold",
            Self::ChannelChatUserMessageUpdate => "channel.chat.user_message_update",
            Self::ChannelSharedChatSessionBegin => "channel.shared_chat.begin",
            Self::ChannelSharedChatSessionUpdate => "channel.shared_chat.update",
            Self::ChannelSharedChatSessionEnd => "channel.shared_chat.end",
            Self::ChannelSubscribe => "channel.subscribe",
            Self::ChannelSubscriptionEnd => "channel.subscription.end",
            Self::ChannelSubscriptionGift => "channel.subscription.gift",
            Self::ChannelSubscriptionMessage => "channel.subscription.message",
            Self::ChannelCheer => "channel.cheer",
            Self::ChannelRaid => "channel.raid",
            Self::ChannelBan => "channel.ban",
            Self::ChannelUnban => "channel.unban",
            Self::ChannelUnbanRequestCreate => "channel.unban_request.create",
            Self::ChannelUnbanRequestResolve => "channel.unban_request.resolve",
            Self::ChannelModerate => "channel.moderate",
            Self::ChannelModerateV2 => "channel.moderate",
            Self::ChannelModeratorAdd => "channel.moderator.add",
            Self::ChannelModeratorRemove => "channel.moderator.remove",
            Self::ChannelGuestStarSessionBegin => "channel.guest_star_session.begin",
            Self::ChannelGuestStarSessionEnd => "channel.guest_star_session.end",
            Self::ChannelGuestStarGuestUpdate => "channel.guest_star_guest.update",
            Self::ChannelGuestStarSettingsUpdate => "channel.guest_star_settings.update",
            Self::ChannelPointsAutomaticRewardRedemption => {
                "channel.channel_points_automatic_reward_redemption.add"
            }
            Self::ChannelPointsCustomRewardAdd => "channel.channel_points_custom_reward.add",
            Self::ChannelPointsCustomRewardUpdate => "channel.channel_points_custom_reward.update",
            Self::ChannelPointsCustomRewardRemove => "channel.channel_points_custom_reward.remove",
            Self::ChannelPointsCustomRewardRedemptionAdd => {
                "channel.channel_points_custom_reward_redemption.add"
            }
            Self::ChannelPointsCustomRewardRedemptionUpdate => {
                "channel.channel_points_custom_reward_redemption.update"
            }
            Self::ChannelPollBegin => "channel.poll.begin",
            Self::ChannelPollProgress => "channel.poll.progress",
            Self::ChannelPollEnd => "channel.poll.end",
            Self::ChannelPredictionBegin => "channel.prediction.begin",
            Self::ChannelPredictionProgress => "channel.prediction.progress",
            Self::ChannelPredictionLock => "channel.prediction.lock",
            Self::ChannelPredictionEnd => "channel.prediction.end",
            Self::ChannelSuspiciousUserMessage => "channel.suspicious_user.message",
            Self::ChannelSuspiciousUserUpdate => "channel.suspicious_user.update",
            Self::ChannelVIPAdd => "channel.vip.add",
            Self::ChannelVIPRemove => "channel.vip.remove",
            Self::ChannelWarningAcknowledgement => "channel.warning.acknowledge",
            Self::ChannelWarningSend => "channel.warning.send",
            Self::CharityDonation => "channel.charity_campaign.donate",
            Self::CharityCampaignStart => "channel.charity_campaign.start",
            Self::CharityCampaignProgress => "channel.charity_campaign.progress",
            Self::CharityCampaignStop => "channel.charity_campaign.stop",
            Self::ConduitShardDisabled => "conduit.shard.disabled",
            Self::DropEntitlementGrant => "drop.entitlement.grant",
            Self::ExtensionBitsTransactionCreate => "extension.bits_transaction.create",
            Self::GoalBegin => "channel.goal.begin",
            Self::GoalProgress => "channel.goal.progress",
            Self::GoalEnd => "channel.goal.end",
            Self::HypeTrainBegin => "channel.hype_train.begin",
            Self::HypeTrainProgress => "channel.hype_train.progress",
            Self::HypeTrainEnd => "channel.hype_train.end",
            Self::ShieldModeBegin => "channel.shield_mode.begin",
            Self::ShieldModeEnd => "channel.shield_mode.end",
            Self::ShoutoutCreate => "channel.shoutout.create",
            Self::ShoutoutReceived => "channel.shoutout.receive",
            Self::StreamOnline => "stream.online",
            Self::StreamOffline => "stream.offline",
            Self::UserAuthorizationGrant => "user.authorization.grant",
            Self::UserAuthorizationRevoke => "user.authorization.revoke",
            Self::UserUpdate => "user.update",
            Self::WhisperReceived => "user.whisper.message",
        }
    }

    pub fn version(&self) -> &str {
        match self {
            Self::AutomodMessageHoldV2 | Self::AutomodMessageUpdateV2 => "2",
            Self::ChannelUpdate | Self::ChannelFollow => "2",
            Self::ChannelModerateV2 => "2",
            Self::ChannelGuestStarSessionBegin
            | Self::ChannelGuestStarSessionEnd
            | Self::ChannelGuestStarGuestUpdate
            | Self::ChannelGuestStarSettingsUpdate => "beta",
            _ => "1",
        }
    }
}
