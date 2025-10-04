mod request;
mod response;
mod types;

pub use request::{GustStarSettingRequest, UpdateSlotSettingsRequest};
pub use response::{GuestStarSettingsResponse, GustStarInvitesResponse, GustStarSessionResponse};
pub use types::{
    GroupLayout, Guest, GuestSetting, GuestStarInvite, GuestStarStatus, GustStarSession,
    GustStarSetting,
};

use crate::{
    request::NoContent,
    types::{
        constants::{BROADCASTER_ID, MODERATOR_ID},
        BroadcasterId, ModeratorId,
    },
};

const GUEST_STAR: &str = "guest_star";

endpoints! {
    GuestStarAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#get-channel-guest-star-settings>
        fn get_channel_guest_star_settings(
            &self,
            broadcaster_id: &BroadcasterId,
            moderator_id: &ModeratorId,
        ) -> GuestStarSettingsResponse {
            endpoint_type: GetChannelGuestStarSettings,
            method: GET,
            path: [GUEST_STAR, "channel_settings"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(MODERATOR_ID, moderator_id)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#update-channel-guest-star-settings>
        fn update_channel_guest_star_settings(
            &self,
            broadcaster_id: &BroadcasterId,
            request: GustStarSettingRequest,
        ) -> NoContent {
            endpoint_type: UpdateChannelGuestStarSettings,
            method: PUT,
            path: [GUEST_STAR, "channel_settings"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                into_query(request)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#get-guest-star-session>
        fn get_guest_star_session(
            &self,
            broadcaster_id: &BroadcasterId,
            moderator_id: &ModeratorId,
        ) -> GustStarSessionResponse {
            endpoint_type: GetGuestStarSession,
            method: GET,
            path: [GUEST_STAR, "session"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(MODERATOR_ID, moderator_id)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#create-guest-star-session>
        fn create_guest_star_session(
            &self,
            broadcaster_id: &BroadcasterId,
        ) -> GustStarSessionResponse {
            endpoint_type: CreateGuestStarSession,
            method: POST,
            path: [GUEST_STAR, "session"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#end-guest-star-session>
        fn end_guest_star_session(
            &self,
            broadcaster_id: &BroadcasterId,
            session_id: &str,
        ) -> GustStarSessionResponse {
            endpoint_type: EndGuestStarSession,
            method: DELETE,
            path: [GUEST_STAR, "session"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query("session_id", session_id)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#get-guest-star-invites>
        fn get_guest_star_invites(
            &self,
            broadcaster_id: &BroadcasterId,
            moderator_id: &ModeratorId,
            session_id: &str,
        ) -> GustStarInvitesResponse {
            endpoint_type: GetGuestStarInvites,
            method: GET,
            path: [GUEST_STAR, "invites"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(MODERATOR_ID, moderator_id),
                query("session_id", session_id)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#send-guest-star-invite>
        fn send_guest_star_invite(
            &self,
            broadcaster_id: &BroadcasterId,
            moderator_id: &ModeratorId,
            session_id: &str,
            guest_id: &str,
        ) -> NoContent {
            endpoint_type: SendGuestStarInvite,
            method: POST,
            path: [GUEST_STAR, "invites"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(MODERATOR_ID, moderator_id),
                query("session_id", session_id),
                query("guest_id", guest_id)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#delete-guest-star-invite>
        fn delete_guest_star_invite(
            &self,
            broadcaster_id: &BroadcasterId,
            moderator_id: &ModeratorId,
            session_id: &str,
            guest_id: &str,
        ) -> NoContent {
            endpoint_type: DeleteGuestStarInvite,
            method: DELETE,
            path: [GUEST_STAR, "invites"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(MODERATOR_ID, moderator_id),
                query("session_id", session_id),
                query("guest_id", guest_id)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#assign-guest-star-slot>
        fn assign_guest_star_slot(
            &self,
            broadcaster_id: &BroadcasterId,
            moderator_id: &ModeratorId,
            session_id: &str,
            guest_id: &str,
            slot_id: &str,
        ) -> NoContent {
            endpoint_type: AssignGuestStarSlot,
            method: POST,
            path: [GUEST_STAR, "slot"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(MODERATOR_ID, moderator_id),
                query("session_id", session_id),
                query("guest_id", guest_id),
                query("slot_id", slot_id)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#update-guest-star-slot>
        fn update_guest_star_slot(
            &self,
            broadcaster_id: &BroadcasterId,
            moderator_id: &ModeratorId,
            session_id: &str,
            source_slot_id: &str,
            destination_slot_id: Option<&str>,
        ) -> NoContent {
            endpoint_type: UpdateGuestStarSlot,
            method: PATCH,
            path: [GUEST_STAR, "slot"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(MODERATOR_ID, moderator_id),
                query("session_id", session_id),
                query("source_slot_id", source_slot_id),
                opt("destination_slot_id", destination_slot_id)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#delete-guest-star-slot>
        fn delete_guest_star_slot(
            &self,
            broadcaster_id: &BroadcasterId,
            moderator_id: &ModeratorId,
            session_id: &str,
            guest_id: &str,
            slot_id: &str,
            should_reinvite_guest: Option<&str>,
        ) -> NoContent {
            endpoint_type: DeleteGuestStarSlot,
            method: DELETE,
            path: [GUEST_STAR, "slot"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(MODERATOR_ID, moderator_id),
                query("session_id", session_id),
                query("guest_id", guest_id),
                query("slot_id", slot_id),
                opt("should_reinvite_guest", should_reinvite_guest)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#update-guest-star-slot-settings>
        fn update_guest_star_slot_settings(
            &self,
            broadcaster_id: &BroadcasterId,
            moderator_id: &ModeratorId,
            session_id: &str,
            slot_id: &str,
            opts: UpdateSlotSettingsRequest,
        ) -> NoContent {
            endpoint_type: UpdateGuestStarSlotSettings,
            method: PATCH,
            path: [GUEST_STAR, "slot_settings"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(MODERATOR_ID, moderator_id),
                query("session_id", session_id),
                query("slot_id", slot_id),
                into_query(opts)
            }
        }
    }
}
