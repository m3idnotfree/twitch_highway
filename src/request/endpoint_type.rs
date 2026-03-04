#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    CreateClipFromVod,
    GetClips,
    GetClipsDownload,
    // Conduits
    GetConduits,
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
    GetHypeTrainStatus,

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
    GetAuthorizationByUser,
    // Videos
    GetVideos,
    DeleteVideos,
    // Whispers
    SendWhisper,
}

impl EndpointType {
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
            Self::CreateClipFromVod => TokenType::Any,
            Self::GetClips => TokenType::Any,
            Self::GetClipsDownload => TokenType::Any,
            // Conduits
            Self::GetConduits => TokenType::App,
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
            Self::GetHypeTrainStatus => TokenType::User,

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
            Self::GetAuthorizationByUser => TokenType::App,
            // Videos
            Self::GetVideos => TokenType::Any,
            Self::DeleteVideos => TokenType::User,
            // Whispers
            Self::SendWhisper => TokenType::User,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
    User,
    App,
    Any,
}
