use twitch_oauth_token::scope::Scope;

#[derive(Debug, PartialEq, Eq)]
pub enum EndpointType {
    // Ads
    StartCommercial,
    GetAdSchedule,
    SnoozeNextAd,
    // Analytics
    GetExtensionAnalytics,
    GetGameAnalytics,
    // Bits
    GetBitsLeaderboard,
    GetCheermotes,
    GetExtensionTransactions,
    // Channels
    GetChanelInformation,
    ModifyChannelInformation,
    GetChannelEditors,
    GetFollowedChannels,
    GetChannelFollowers,
    // Channel Points
    CreateCustomRewards,
    DeleteCustomReward,
    GetCustomReward,
    GetCustomRewardRedemption,
    UpdateCustomReward,
    UpdateRedemptionStatus,
    // Charity
    GetCharityCampaign,
    GetCharityCampaignDonations,
    // Chat
    GetChatters,
    GetChannelEmotes,
    GetGlobalEmotes,
    GetEmoteSets,
    GetChannelChatBadges,
    GetGlobalChatBadges,
    GetChatSettings,
    GetShardChatSession,
    GetUserEmotes,
    UpdateChatSettings,
    SendChatAnnouncement,
    SendAShoutout,
    SendChatMessage,
    GetUserChatColor,
    UpdateUserChatColor,
    // Clips
    CreateClip,
    GetClips,
    // Conduits
    CreateConduits,
    UpdateConduits,
    DeleteConduit,
    GetConduitShards,
    UpdateConduitShards,
    // CCLs
    GetContentClassificationLabels,
    // Entitlements
    GetDropsEntitlements,
    UpdateDropsEntitlements,
    // Extensions
    GetExtensionConfigurationSegment,
    SetExtensionConfigurationSegment,
    SetExtensionRequiredConfiguration,
    SendExtensionPubSubMessage,
    GetExtensionLiveChannels,
    GetExtensionSecrets,
    CreateExtensionSecret,
    SendExtensionChatMessage,
    GetExtensions,
    GetReleasedExtensions,
    GetExtensionBitsProducts,
    UpdateExtensionBitsProduct,
    // EventSub
    DeleteEventSub,
    CreateEventSub,
    GetEventSub,
    ChannelRaid,
    // Games
    GetTopGames,
    GetGames,
    GetCreatorGoals,
    // Guest Star
    GetChannelGuestStarSettings,
    UpdateChannelGuestStarSettings,
    GetGuestStarSession,
    CreateGuestStarSession,
    EndGuestStarSession,
    GetGuestStarInvites,
    SendGuestStarInvite,
    DeleteGuestStarInvite,
    AssignGuestStarSlot,
    UpdateGuestStarSlot,
    DeleteGuestStarSlot,
    UpdateGuestStarSlotSettings,
    // Hype Train
    GetHypeTrainEvents,

    // Moderation
    CheckAutoModStatus,
    ManageHeldAutoModMessages,
    GetAutoModSettings,
    UpdateAutoModSettings,
    GetBannedUsers,
    BanUsers,
    UnbanUser,
    GetUnbanRequests,
    ResolveUnbanRequests,
    GetBlockedTerms,
    AddBlockedTerm,
    RemoveBlockedTerm,
    DeleteChatMessages,
    GetModeratedChannels,
    GetModerators,
    AddChannelModerator,
    RemoveChannelModerator,
    GetVIPs,
    AddChannelVIP,
    RemoveChannelVIP,
    UpdateShieldModeStatus,
    GetShieldModeStatus,
    WarnChatUser,

    // Polls
    GetPolls,
    CreatePoll,
    EndPoll,
    // Predictions
    GetPredictions,
    CreatePrediction,
    EndPrediction,
    // Raids
    Startraid,
    Cancelraid,
    // Schedule
    GetChannelStreamSchedule,
    GetChanneliCalendar,
    UpdateChannelStreamSchedule,
    CreateChannelStreamScheduleSegment,
    UpdateChannelStreamScheduleSegment,
    DeleteChannelStreamScheduleSegment,
    // Search
    SearchCategories,
    SearchChannels,
    // Streams
    GetStreamKey,
    GetStreams,
    GetFollowedStreams,
    CreateStreamMarker,
    GetStreamMarkers,
    // Subscriptions
    GetBroadcasterSubscriptions,
    CheckUserSubscriptions,
    // Tags
    // deprecated
    // GetAllStreamTags,
    // GetStreamTags,
    // Teams
    GetChannelTeams,
    GetTeams,
    // Users
    GetUsers,
    UpdateUser,
    GetUserBlockList,
    BlockUser,
    UnblockUser,
    GetUserExtensions,
    GetUserActiveExtensions,
    UpdateUserExtensions,
    // Videos
    GetVideos,
    DeleteVideos,
    // Whispers
    SendWhisper,
}

impl EndpointType {
    pub fn required_scopes(&self) -> Option<Vec<Scope>> {
        match self {
            // Ads
            Self::StartCommercial => Some(vec![Scope::ChannelEditCommercial]),
            Self::GetAdSchedule => Some(vec![Scope::ChannelReadAds]),
            Self::SnoozeNextAd => Some(vec![Scope::ChannelManageAds]),
            // Analytics
            Self::GetExtensionAnalytics => Some(vec![Scope::AnalyticsReadExtensions]),
            Self::GetGameAnalytics => Some(vec![Scope::AnalyticsReadGames]),
            // Bits
            Self::GetBitsLeaderboard => Some(vec![Scope::BitsRead]),
            Self::GetCheermotes => None,
            Self::GetExtensionTransactions => None,
            // Channels
            Self::GetChanelInformation => None,
            Self::ModifyChannelInformation => Some(vec![Scope::ChannelManageBroadcast]),
            Self::GetChannelEditors => Some(vec![Scope::ChannelReadEditors]),
            Self::GetFollowedChannels => Some(vec![Scope::UserReadFollows]),
            Self::GetChannelFollowers => Some(vec![Scope::ModeratorReadFollowers]),
            // Channel Points
            Self::CreateCustomRewards => Some(vec![Scope::ChannelManageRedemptions]),
            Self::DeleteCustomReward => Some(vec![Scope::ChannelManageRedemptions]),
            Self::GetCustomReward => Some(vec![Scope::ChannelReadRedemptions]),
            Self::GetCustomRewardRedemption => Some(vec![Scope::ChannelReadRedemptions]),
            Self::UpdateCustomReward => Some(vec![Scope::ChannelManageRedemptions]),
            Self::UpdateRedemptionStatus => Some(vec![Scope::ChannelManageRedemptions]),
            // Charity
            Self::GetCharityCampaign => Some(vec![Scope::ChannelReadCharity]),
            Self::GetCharityCampaignDonations => Some(vec![Scope::ChannelReadCharity]),
            // Chat
            Self::GetChatters => Some(vec![Scope::ModeratorReadChatters]),
            Self::GetChannelEmotes => None,
            Self::GetGlobalEmotes => None,
            Self::GetEmoteSets => None,
            Self::GetChannelChatBadges => None,
            Self::GetGlobalChatBadges => None,
            Self::GetChatSettings => None,
            Self::GetShardChatSession => None,
            Self::GetUserEmotes => Some(vec![Scope::UserReadEmotes]),
            Self::UpdateChatSettings => Some(vec![Scope::ModeratorManageChatSettings]),
            Self::SendChatAnnouncement => Some(vec![Scope::ModeratorManageAnnouncements]),
            Self::SendAShoutout => Some(vec![Scope::ModeratorManageShoutouts]),
            Self::SendChatMessage => Some(vec![
                Scope::UserWriteChat,
                Scope::UserBot,
                Scope::ChannelBot,
            ]),
            Self::GetUserChatColor => None,
            Self::UpdateUserChatColor => Some(vec![Scope::UserManageChatColor]),
            // Clips
            Self::CreateClip => Some(vec![Scope::ClipsEdit]),
            Self::GetClips => None,
            // Conduits
            Self::CreateConduits => None,
            Self::UpdateConduits => None,
            Self::DeleteConduit => None,
            Self::GetConduitShards => None,
            Self::UpdateConduitShards => None,
            // CCLs
            Self::GetContentClassificationLabels => None,
            // Entitlements
            Self::GetDropsEntitlements => None,
            Self::UpdateDropsEntitlements => None,
            // Extensions
            Self::GetExtensionConfigurationSegment => None,
            Self::SetExtensionConfigurationSegment => None,
            Self::SetExtensionRequiredConfiguration => None,
            Self::SendExtensionPubSubMessage => None,
            Self::GetExtensionLiveChannels => None,
            Self::GetExtensionSecrets => None,
            Self::CreateExtensionSecret => None,
            Self::SendExtensionChatMessage => None,
            Self::GetExtensions => None,
            Self::GetReleasedExtensions => None,
            Self::GetExtensionBitsProducts => None,
            Self::UpdateExtensionBitsProduct => None,

            // EventSub
            Self::DeleteEventSub => None,
            Self::CreateEventSub => None,
            Self::GetEventSub => None,
            Self::ChannelRaid => None,
            // Games
            Self::GetTopGames => None,
            Self::GetGames => None,
            // Goals
            Self::GetCreatorGoals => Some(vec![Scope::ChannelReadGoals]),
            // Guest Star
            Self::GetChannelGuestStarSettings => Some(vec![
                Scope::ChannelReadGuestStar,
                Scope::ChannelManageGuestStar,
                Scope::ModeratorReadGuestStar,
            ]),
            Self::UpdateChannelGuestStarSettings => Some(vec![Scope::ChannelManageGuestStar]),
            Self::GetGuestStarSession => Some(vec![
                Scope::ChannelReadGuestStar,
                Scope::ChannelManageGuestStar,
                Scope::ModeratorReadGuestStar,
            ]),
            Self::CreateGuestStarSession => Some(vec![Scope::ChannelManageGuestStar]),
            Self::EndGuestStarSession => Some(vec![Scope::ChannelManageGuestStar]),
            Self::GetGuestStarInvites => Some(vec![
                Scope::ChannelReadGuestStar,
                Scope::ChannelManageGuestStar,
                Scope::ModeratorReadGuestStar,
            ]),
            // or moderator:manage:guest_star
            Self::SendGuestStarInvite => Some(vec![Scope::ChannelManageGuestStar]),
            // or moderator:manage:guest_star
            Self::DeleteGuestStarInvite => Some(vec![Scope::ChannelManageGuestStar]),
            // or moderator:manage:guest_star
            Self::AssignGuestStarSlot => Some(vec![Scope::ChannelManageGuestStar]),
            // or moderator:manage:guest_star
            Self::UpdateGuestStarSlot => Some(vec![Scope::ChannelManageGuestStar]),
            // or moderator:manage:guest_star
            Self::DeleteGuestStarSlot => Some(vec![Scope::ChannelManageGuestStar]),
            // or moderator:manage:guest_star
            Self::UpdateGuestStarSlotSettings => Some(vec![Scope::ChannelManageGuestStar]),
            // Hype Train
            Self::GetHypeTrainEvents => Some(vec![Scope::ChannelReadHypeTrain]),

            // Moderation
            Self::CheckAutoModStatus => Some(vec![Scope::ModerationRead]),
            Self::ManageHeldAutoModMessages => Some(vec![Scope::ModeratorManageAutomod]),
            Self::GetAutoModSettings => Some(vec![Scope::ModeratorReadAutomodSettings]),
            Self::UpdateAutoModSettings => Some(vec![Scope::ModeratorManageAutomodSettings]),
            Self::GetBannedUsers => Some(vec![Scope::ModerationRead]),
            Self::BanUsers => Some(vec![Scope::ModeratorManageBannedUsers]),
            Self::UnbanUser => Some(vec![Scope::ModeratorManageBannedUsers]),
            Self::GetUnbanRequests => Some(vec![Scope::ModeratorReadUnbanRequests]),
            Self::ResolveUnbanRequests => Some(vec![Scope::ModeratorManageUnbanRequests]),
            Self::GetBlockedTerms => Some(vec![Scope::ModeratorReadBlockedTerms]),
            Self::AddBlockedTerm => Some(vec![Scope::ModeratorManageBlockedTerms]),
            Self::RemoveBlockedTerm => Some(vec![Scope::ModeratorManageBlockedTerms]),
            Self::DeleteChatMessages => Some(vec![Scope::ModeratorManageChatMessages]),
            Self::GetModeratedChannels => Some(vec![Scope::UserReadModeratedChannels]),
            Self::GetModerators => Some(vec![Scope::ModerationRead]),
            Self::AddChannelModerator => Some(vec![Scope::ChannelManageModerators]),
            Self::RemoveChannelModerator => Some(vec![Scope::ChannelManageModerators]),
            Self::GetVIPs => Some(vec![Scope::ChannelReadVips]),
            Self::AddChannelVIP => Some(vec![Scope::ChannelManageVips]),
            Self::RemoveChannelVIP => Some(vec![Scope::ChannelManageVips]),
            Self::UpdateShieldModeStatus => Some(vec![Scope::ModeratorManageShieldMode]),
            Self::GetShieldModeStatus => Some(vec![Scope::ModeratorReadShieldMode]),
            Self::WarnChatUser => Some(vec![Scope::ModeratorManageWarnings]),

            // Polls
            Self::GetPolls => Some(vec![Scope::ChannelReadPolls]),
            Self::CreatePoll => Some(vec![Scope::ChannelManagePolls]),
            Self::EndPoll => Some(vec![Scope::ChannelManagePolls]),
            // Predictions
            Self::GetPredictions => Some(vec![Scope::ChannelReadPredictions]),
            Self::CreatePrediction => Some(vec![Scope::ChannelManagePredictions]),
            Self::EndPrediction => Some(vec![Scope::ChannelManagePredictions]),
            // Raids
            Self::Startraid => Some(vec![Scope::ChannelManageRaids]),
            Self::Cancelraid => Some(vec![Scope::ChannelManageRaids]),
            // Schedule
            Self::GetChannelStreamSchedule => None,
            Self::GetChanneliCalendar => None,
            Self::UpdateChannelStreamSchedule => Some(vec![Scope::ChannelManageSchedule]),
            Self::CreateChannelStreamScheduleSegment => Some(vec![Scope::ChannelManageSchedule]),
            Self::UpdateChannelStreamScheduleSegment => Some(vec![Scope::ChannelManageSchedule]),
            Self::DeleteChannelStreamScheduleSegment => Some(vec![Scope::ChannelManageSchedule]),
            // Search
            Self::SearchCategories => None,
            Self::SearchChannels => None,
            // Streams
            Self::GetStreamKey => Some(vec![Scope::ChannelReadStreamKey]),
            Self::GetStreams => None,
            Self::GetFollowedStreams => Some(vec![Scope::UserReadFollows]),
            Self::CreateStreamMarker => Some(vec![Scope::ChannelManageBroadcast]),
            // or ChannelManageBroadcast
            Self::GetStreamMarkers => Some(vec![Scope::UserReadBroadcast]),
            // Subscriptions
            // A Twitch extensions may use an app access token if the broadcaster has granted the channel:read:subscriptions scope from within the Twitch Extensions manager.
            Self::GetBroadcasterSubscriptions => Some(vec![Scope::ChannelReadSubscriptions]),
            Self::CheckUserSubscriptions => Some(vec![Scope::UserReadSubscriptions]),
            // Tags
            // deprecated
            //Self::GetAllStreamTags => None,
            //Self::GetStreamTags => Some(vec![Scope::ChannelManagePolls]),
            // Teams
            Self::GetChannelTeams => None,
            Self::GetTeams => None,
            // Users
            Self::GetUsers => Some(vec![Scope::UserReadEmail]),
            Self::UpdateUser => Some(vec![Scope::UserReadEmail, Scope::UserEdit]),
            Self::GetUserBlockList => Some(vec![Scope::UserReadBlockedUsers]),
            Self::BlockUser => Some(vec![Scope::UserManageBlockedUsers]),
            Self::UnblockUser => Some(vec![Scope::UserManageBlockedUsers]),
            Self::GetUserExtensions => {
                Some(vec![Scope::UserReadBroadcast, Scope::UserEditBroadcast])
            }
            Self::GetUserActiveExtensions => {
                Some(vec![Scope::UserReadBroadcast, Scope::UserEditBroadcast])
            }
            Self::UpdateUserExtensions => Some(vec![Scope::UserEditBroadcast]),
            // Videos
            Self::GetVideos => None,
            Self::DeleteVideos => Some(vec![Scope::ChannelManageVideos]),
            // Whispers
            Self::SendWhisper => Some(vec![Scope::UserManageWhispers]),
        }
    }

    pub fn token_type(&self) -> TokenType {
        match self {
            // Ads
            Self::StartCommercial => TokenType::User,
            Self::GetAdSchedule => TokenType::User,
            Self::SnoozeNextAd => TokenType::User,
            // Analytics
            Self::GetExtensionAnalytics => TokenType::User,
            Self::GetGameAnalytics => TokenType::User,
            // Bits
            Self::GetBitsLeaderboard => TokenType::User,
            Self::GetCheermotes => TokenType::Any,
            Self::GetExtensionTransactions => TokenType::App,
            // Channels
            Self::GetChanelInformation => TokenType::Any,
            Self::ModifyChannelInformation => TokenType::User,
            Self::GetChannelEditors => TokenType::User,
            Self::GetFollowedChannels => TokenType::User,
            Self::GetChannelFollowers => TokenType::User,
            // Channel Points
            Self::CreateCustomRewards => TokenType::User,
            Self::DeleteCustomReward => TokenType::User,
            Self::GetCustomReward => TokenType::User,
            Self::GetCustomRewardRedemption => TokenType::User,
            Self::UpdateCustomReward => TokenType::User,
            Self::UpdateRedemptionStatus => TokenType::User,
            // Charity
            Self::GetCharityCampaign => TokenType::User,
            Self::GetCharityCampaignDonations => TokenType::User,
            // Chat
            Self::GetChatters => TokenType::User,
            Self::GetChannelEmotes => TokenType::Any,
            Self::GetGlobalEmotes => TokenType::Any,
            Self::GetEmoteSets => TokenType::Any,
            Self::GetChannelChatBadges => TokenType::Any,
            Self::GetGlobalChatBadges => TokenType::Any,
            Self::GetChatSettings => TokenType::Any,
            Self::GetShardChatSession => TokenType::Any,
            Self::GetUserEmotes => TokenType::User,
            Self::UpdateChatSettings => TokenType::User,
            Self::SendChatAnnouncement => TokenType::User,
            Self::SendAShoutout => TokenType::User,
            Self::SendChatMessage => TokenType::Any,
            Self::GetUserChatColor => TokenType::Any,
            Self::UpdateUserChatColor => TokenType::User,
            // Clips
            Self::CreateClip => TokenType::User,
            Self::GetClips => TokenType::Any,
            // Conduits
            Self::CreateConduits => TokenType::App,
            Self::UpdateConduits => TokenType::App,
            Self::DeleteConduit => TokenType::App,
            Self::GetConduitShards => TokenType::App,
            Self::UpdateConduitShards => TokenType::App,
            // CCLs
            Self::GetContentClassificationLabels => TokenType::Any,
            // Entitlements
            Self::GetDropsEntitlements => TokenType::Any,
            Self::UpdateDropsEntitlements => TokenType::Any,
            // Extensions
            Self::GetExtensionConfigurationSegment => TokenType::Any,
            Self::SetExtensionConfigurationSegment => TokenType::Any,
            Self::SetExtensionRequiredConfiguration => TokenType::Any,
            Self::SendExtensionPubSubMessage => TokenType::Any,
            Self::GetExtensionLiveChannels => TokenType::Any,
            Self::GetExtensionSecrets => TokenType::Any,
            Self::CreateExtensionSecret => TokenType::Any,
            Self::SendExtensionChatMessage => TokenType::Any,
            Self::GetExtensions => TokenType::Any,
            Self::GetReleasedExtensions => TokenType::Any,
            Self::GetExtensionBitsProducts => TokenType::Any,
            Self::UpdateExtensionBitsProduct => TokenType::Any,

            // EventSub
            Self::CreateEventSub => TokenType::App,
            Self::DeleteEventSub => TokenType::App,
            Self::GetEventSub => TokenType::App,
            Self::ChannelRaid => TokenType::App,

            // Games
            Self::GetTopGames => TokenType::Any,
            Self::GetGames => TokenType::Any,
            // Goals
            Self::GetCreatorGoals => TokenType::User,
            // Guest Star
            Self::GetChannelGuestStarSettings => TokenType::User,
            Self::UpdateChannelGuestStarSettings => TokenType::User,
            Self::GetGuestStarSession => TokenType::User,
            Self::CreateGuestStarSession => TokenType::User,
            Self::EndGuestStarSession => TokenType::User,
            Self::GetGuestStarInvites => TokenType::User,
            Self::SendGuestStarInvite => TokenType::User,
            Self::DeleteGuestStarInvite => TokenType::User,
            Self::AssignGuestStarSlot => TokenType::User,
            Self::UpdateGuestStarSlot => TokenType::User,
            Self::DeleteGuestStarSlot => TokenType::User,
            Self::UpdateGuestStarSlotSettings => TokenType::User,
            // Hype Train
            Self::GetHypeTrainEvents => TokenType::User,

            // Moderation
            Self::CheckAutoModStatus => TokenType::User,
            Self::ManageHeldAutoModMessages => TokenType::User,
            Self::GetAutoModSettings => TokenType::User,
            Self::UpdateAutoModSettings => TokenType::User,
            Self::GetBannedUsers => TokenType::User,
            Self::BanUsers => TokenType::User,
            Self::UnbanUser => TokenType::User,
            Self::GetUnbanRequests => TokenType::User,
            Self::ResolveUnbanRequests => TokenType::User,
            Self::GetBlockedTerms => TokenType::User,
            Self::AddBlockedTerm => TokenType::User,
            Self::RemoveBlockedTerm => TokenType::User,
            Self::DeleteChatMessages => TokenType::User,
            Self::GetModeratedChannels => TokenType::User,
            Self::GetModerators => TokenType::User,
            Self::AddChannelModerator => TokenType::User,
            Self::RemoveChannelModerator => TokenType::User,
            Self::GetVIPs => TokenType::User,
            Self::AddChannelVIP => TokenType::User,
            Self::RemoveChannelVIP => TokenType::User,
            Self::UpdateShieldModeStatus => TokenType::User,
            Self::GetShieldModeStatus => TokenType::User,
            Self::WarnChatUser => TokenType::User,

            // Polls
            Self::GetPolls => TokenType::User,
            Self::CreatePoll => TokenType::User,
            Self::EndPoll => TokenType::User,
            // Predictions
            Self::GetPredictions => TokenType::User,
            Self::CreatePrediction => TokenType::User,
            Self::EndPrediction => TokenType::User,
            // Raids
            Self::Startraid => TokenType::User,
            Self::Cancelraid => TokenType::User,
            // Schedule
            Self::GetChannelStreamSchedule => TokenType::Any,
            Self::GetChanneliCalendar => TokenType::Any,
            Self::UpdateChannelStreamSchedule => TokenType::User,
            Self::CreateChannelStreamScheduleSegment => TokenType::User,
            Self::UpdateChannelStreamScheduleSegment => TokenType::User,
            Self::DeleteChannelStreamScheduleSegment => TokenType::User,
            // Search
            Self::SearchCategories => TokenType::Any,
            Self::SearchChannels => TokenType::Any,
            // Streams
            Self::GetStreamKey => TokenType::User,
            Self::GetStreams => TokenType::Any,
            Self::GetFollowedStreams => TokenType::User,
            Self::CreateStreamMarker => TokenType::User,
            Self::GetStreamMarkers => TokenType::User,
            // Subscriptions
            Self::GetBroadcasterSubscriptions => TokenType::User,
            Self::CheckUserSubscriptions => TokenType::User,
            // Tags
            //Self::GetAllStreamTags => TokenType::User,
            //Self::GetStreamTags => TokenType::User,
            // Teams
            Self::GetChannelTeams => TokenType::Any,
            Self::GetTeams => TokenType::Any,

            // Users
            Self::GetUsers => TokenType::Any,
            Self::UpdateUser => TokenType::User,
            Self::GetUserBlockList => TokenType::User,
            Self::BlockUser => TokenType::User,
            Self::UnblockUser => TokenType::User,
            Self::GetUserExtensions => TokenType::User,
            Self::GetUserActiveExtensions => TokenType::Any,
            Self::UpdateUserExtensions => TokenType::User,
            // Videos
            Self::GetVideos => TokenType::Any,
            Self::DeleteVideos => TokenType::User,
            // Whispers
            Self::SendWhisper => TokenType::User,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    User,
    App,
    Any,
}
