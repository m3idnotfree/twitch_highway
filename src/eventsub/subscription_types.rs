#![allow(unreachable_patterns)]

use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::Deref,
    str::FromStr,
};

use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

macro_rules! define_subscription_types {
    (
        $(#[$meta:meta])*
        enum SubscriptionType {
            $(
                $name:ident($value:literal, $version:literal),
            )*
        }
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        $(#[$meta])*
        pub enum SubscriptionType {
            $(
                $name,
            )*
        }

        impl SubscriptionType {
            pub fn as_str(&self) -> &str {
                match self {
                    $(
                        Self::$name => $value,
                    )*
                }
            }

            pub fn version(&self) -> &str {
                match self {
                    $(
                        Self::$name => $version,
                    )*
                }
            }
        }

        impl Display for SubscriptionType {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str(self.as_str())
            }
        }

        impl From<SubscriptionType> for String {
            fn from(value: SubscriptionType) -> Self {
                value.to_string()
            }
        }

        impl FromStr for SubscriptionType {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(
                        $value => Ok(Self::$name),
                    )*
                    _ => Err(format!("Unknown subscription type: '{}'", s)),
                }
            }
        }

        impl AsRef<str> for SubscriptionType {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl Deref for SubscriptionType {
            type Target = str;

            fn deref(&self) -> &Self::Target {
                self.as_str()
            }
        }

        impl<'de> Deserialize<'de> for SubscriptionType {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                s.parse().map_err(D::Error::custom)
            }
        }

        impl Serialize for SubscriptionType {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }

    };
}

define_subscription_types! {
    /// <https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/>
    enum SubscriptionType {
        // Automod related
        AutomodMessageHold("automod.message.hold", "1"),
        AutomodMessageHoldV2("automod.message.hold", "2"),
        AutomodMessageUpdate("automod.message.update", "1"),
        AutomodMessageUpdateV2("automod.message.update", "2"),
        AutomodSettingsUpdate("automod.settings.update", "1"),
        AutomodTermsUpdate("automod.terms.update", "1"),

        // Channel related
        ChannelBitsUse("channel.bits.use", "1"),
        ChannelUpdate("channel.update", "2"),
        ChannelFollow("channel.follow", "2"),
        ChannelAdBreakBegin("channel.ad_break.begin", "1"),

        // Chat related
        ChannelChatClear("channel.chat.clear", "1"),
        ChannelChatClearUserMessages("channel.chat.clear_user_messages", "1"),
        ChannelChatMessage("channel.chat.message", "1"),
        ChannelChatMessageDelete("channel.chat.message_delete", "1"),
        ChannelChatNotification("channel.chat.notification", "1"),
        ChannelChatSettingsUpdate("channel.chat_settings.update", "1"),
        ChannelChatUserMessageHold("channel.chat.user_message_hold", "1"),
        ChannelChatUserMessageUpdate("channel.chat.user_message_update", "1"),

        // Shared Chat
        ChannelSharedChatSessionBegin("channel.shared_chat.begin", "1"),
        ChannelSharedChatSessionUpdate("channel.shared_chat.update", "1"),
        ChannelSharedChatSessionEnd("channel.shared_chat.end", "1"),

        // Subscription related
        ChannelSubscribe("channel.subscribe", "1"),
        ChannelSubscriptionEnd("channel.subscription.end", "1"),
        ChannelSubscriptionGift("channel.subscription.gift", "1"),
        ChannelSubscriptionMessage("channel.subscription.message", "1"),

        // Channel interactions
        ChannelCheer("channel.cheer", "1"),
        ChannelRaid("channel.raid", "1"),

        // Moderation
        ChannelBan("channel.ban", "1"),
        ChannelUnban("channel.unban", "1"),
        ChannelUnbanRequestCreate("channel.unban_request.create", "1"),
        ChannelUnbanRequestResolve("channel.unban_request.resolve", "1"),
        ChannelModerate("channel.moderate", "1"),
        ChannelModerateV2("channel.moderate", "2"),
        ChannelModeratorAdd("channel.moderator.add", "1"),
        ChannelModeratorRemove("channel.moderator.remove", "1"),

        // Guest Star (Beta)
        ChannelGuestStarSessionBegin("channel.guest_star_session.begin", "beta"),
        ChannelGuestStarSessionEnd("channel.guest_star_session.end", "beta"),
        ChannelGuestStarGuestUpdate("channel.guest_star_guest.update", "beta"),
        ChannelGuestStarSettingsUpdate("channel.guest_star_settings.update", "beta"),

        // Channel Points
        ChannelPointsAutomaticRewardRedemptionAdd("channel.channel_points_automatic_reward_redemption.add", "1"),
        ChannelPointsAutomaticRewardRedemptionAddV2("channel.channel_points_automatic_reward_redemption.add", "2"),
        ChannelPointsCustomRewardAdd("channel.channel_points_custom_reward.add", "1"),
        ChannelPointsCustomRewardUpdate("channel.channel_points_custom_reward.update", "1"),
        ChannelPointsCustomRewardRemove("channel.channel_points_custom_reward.remove", "1"),
        ChannelPointsCustomRewardRedemptionAdd("channel.channel_points_custom_reward_redemption.add", "1"),
        ChannelPointsCustomRewardRedemptionUpdate("channel.channel_points_custom_reward_redemption.update", "1"),

        // Polls and Predictions
        ChannelPollBegin("channel.poll.begin", "1"),
        ChannelPollProgress("channel.poll.progress", "1"),
        ChannelPollEnd("channel.poll.end", "1"),
        ChannelPredictionBegin("channel.prediction.begin", "1"),
        ChannelPredictionProgress("channel.prediction.progress", "1"),
        ChannelPredictionLock("channel.prediction.lock", "1"),
        ChannelPredictionEnd("channel.prediction.end", "1"),

        // Suspicious Users
        ChannelSuspiciousUserMessage("channel.suspicious_user.message", "1"),
        ChannelSuspiciousUserUpdate("channel.suspicious_user.update", "1"),

        // VIP and Warnings
        ChannelVIPAdd("channel.vip.add", "1"),
        ChannelVIPRemove("channel.vip.remove", "1"),
        ChannelWarningAcknowledgement("channel.warning.acknowledge", "1"),
        ChannelWarningSend("channel.warning.send", "1"),

        // Charity
        CharityDonation("channel.charity_campaign.donate", "1"),
        CharityCampaignStart("channel.charity_campaign.start", "1"),
        CharityCampaignProgress("channel.charity_campaign.progress", "1"),
        CharityCampaignStop("channel.charity_campaign.stop", "1"),

        // Conduit
        ConduitShardDisabled("conduit.shard.disabled", "1"),

        // Drops and Extensions
        DropEntitlementGrant("drop.entitlement.grant", "1"),
        ExtensionBitsTransactionCreate("extension.bits_transaction.create", "1"),

        // Goals and Hype Train
        GoalBegin("channel.goal.begin", "1"),
        GoalProgress("channel.goal.progress", "1"),
        GoalEnd("channel.goal.end", "1"),

        // deprecated
        // HypeTrainBegin("channel.hype_train.begin", "1"),
        HypeTrainBeginV2("channel.hype_train.begin", "2"),
        // deprecated
        // HypeTrainProgress("channel.hype_train.progress", "1"),
        HypeTrainProgressV2("channel.hype_train.progress", "2"),
        // deprecated
        // HypeTrainEnd("channel.hype_train.end", "1"),
        HypeTrainEndV2("channel.hype_train.end", "2"),

        // Shield Mode
        ShieldModeBegin("channel.shield_mode.begin", "1"),
        ShieldModeEnd("channel.shield_mode.end", "1"),

        // Shoutouts
        ShoutoutCreate("channel.shoutout.create", "1"),
        ShoutoutReceived("channel.shoutout.receive", "1"),

        // Stream Status
        StreamOnline("stream.online", "1"),
        StreamOffline("stream.offline", "1"),

        // User Authorization
        UserAuthorizationGrant("user.authorization.grant", "1"),
        UserAuthorizationRevoke("user.authorization.revoke", "1"),
        UserUpdate("user.update", "1"),

        // Whispers
        WhisperReceived("user.whisper.message", "1"),
    }
}
