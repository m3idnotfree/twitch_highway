use crate::{
    guest_star::GroupLayout,
    request::NoContent,
    types::{
        constants::{
            BROADCASTER_ID, CHANNEL_SETTINGS, DESTINATION_SLOT_ID, GROUP_LAYOUT, GUEST_ID,
            GUEST_STAR, IS_AUDIO_ENABLED, IS_BROWSER_SOURCE_AUDIO_ENABLED, IS_LIVE,
            IS_MODERATOR_SEND_LIVE_ENABLED, IS_VIDEO_ENABLED, MODERATOR_ID,
            REGENERATE_BROWSER_SOURCES, SESSION_ID, SHOULD_REINVITE_GUEST, SLOT, SLOT_COUNT,
            SLOT_ID, SLOT_SETTINGS, SOURCE_SLOT_ID, VOLUME,
        },
        BroadcasterId, ModeratorId, SessionId, UserId,
    },
};

define_request_builder! {
    #[derive(Debug)]
    UpdateChannelGuestStarSettingsBuilder<'a> {
        req: {broadcaster_id: &'a BroadcasterId [key = BROADCASTER_ID]},
        opts: {
            is_moderator_send_live_enabled: bool [key = IS_MODERATOR_SEND_LIVE_ENABLED, convert = to_string],
            slot_count: u64 [key = SLOT_COUNT, convert = to_string],
            is_browser_source_audio_enabled: bool [key = IS_BROWSER_SOURCE_AUDIO_ENABLED, convert = to_string],
            group_layout: GroupLayout [key = GROUP_LAYOUT, convert = as_ref],
            regenerate_browser_sources: bool [key = REGENERATE_BROWSER_SOURCES, convert = to_string],
        }
    } -> NoContent;
    endpoint_type: UpdateChannelGuestStarSettings,
    method: PUT,
    path: [GUEST_STAR, CHANNEL_SETTINGS],
}

define_request_builder! {
    #[derive(Debug)]
    UpdateGuestStarSlotBuilder<'a> {
        req: {
            broadcaster_id: &'a BroadcasterId [key = BROADCASTER_ID],
            moderator_id: &'a ModeratorId [key = MODERATOR_ID],
            session_id: &'a SessionId [key = SESSION_ID],
            source_slot_id: &'a str [key = SOURCE_SLOT_ID],
        },
        opts: {
            destination_slot_id: &'a str [key = DESTINATION_SLOT_ID]
        }
    } -> NoContent;
    endpoint_type: UpdateGuestStarSlot,
    method: PATCH,
    path: [GUEST_STAR, SLOT],
}

define_request_builder! {
    #[derive(Debug)]
    DeleteGuestStarSlotBuilder<'a> {
        req: {
            broadcaster_id: &'a BroadcasterId [key = BROADCASTER_ID],
            moderator_id: &'a ModeratorId [key = MODERATOR_ID],
            session_id: &'a SessionId [key = SESSION_ID],
            guest_id: &'a UserId [key = GUEST_ID],
            slot_id: &'a str [key = SLOT_ID],
        },
        opts: {should_reinvite_guest: &'a str [key = SHOULD_REINVITE_GUEST]}
    } -> NoContent;
    endpoint_type: DeleteGuestStarSlot,
    method: DELETE,
    path: [GUEST_STAR, SLOT],
}

define_request_builder! {
    #[derive(Debug)]
    UpdateGuestStarSlotSettingsBuilder<'a> {
        req: {
            broadcaster_id: &'a BroadcasterId [key = BROADCASTER_ID],
            moderator_id: &'a ModeratorId [key = MODERATOR_ID],
            session_id: &'a SessionId [key = SESSION_ID],
            slot_id: &'a str [key = SLOT_ID],
        }
        opts: {
            is_audio_enabled: bool [key = IS_AUDIO_ENABLED, convert = to_string],
            is_video_enabled: bool [key = IS_VIDEO_ENABLED, convert = to_string],
            is_live: bool [key = IS_LIVE, convert = to_string],
            volume: u64 [key = VOLUME, convert = to_string]
        }
    } -> NoContent;
    endpoint_type: UpdateGuestStarSlotSettings,
    method: PATCH,
    path: [GUEST_STAR, SLOT_SETTINGS],
}
