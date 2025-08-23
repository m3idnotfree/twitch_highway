pub mod request;
pub mod response;
pub mod types;

use asknothingx2_util::api::Method;
use request::{GustStarSettingRequest, UpdateSlotSettingsRequest};
use response::{GuestStarSettingsResponse, GustStarInvitesResponse, GustStarSessionResponse};

use crate::{
    request::{EndpointType, NoContent},
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
            endpoint_type: EndpointType::GetChannelGuestStarSettings,
            method: Method::GET,
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
            endpoint_type: EndpointType::UpdateChannelGuestStarSettings,
            method: Method::PUT,
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
            endpoint_type: EndpointType::GetGuestStarSession,
            method: Method::GET,
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
            endpoint_type: EndpointType::CreateGuestStarSession,
            method: Method::POST,
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
            endpoint_type: EndpointType::EndGuestStarSession,
            method: Method::DELETE,
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
            endpoint_type: EndpointType::GetGuestStarInvites,
            method: Method::GET,
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
            endpoint_type: EndpointType::SendGuestStarInvite,
            method: Method::POST,
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
            endpoint_type: EndpointType::DeleteGuestStarInvite,
            method: Method::DELETE,
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
            endpoint_type: EndpointType::AssignGuestStarSlot,
            method: Method::POST,
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
            endpoint_type: EndpointType::UpdateGuestStarSlot,
            method: Method::PATCH,
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
            endpoint_type: EndpointType::DeleteGuestStarSlot,
            method: Method::DELETE,
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
            endpoint_type: EndpointType::UpdateGuestStarSlotSettings,
            method: Method::PATCH,
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

#[cfg(test)]
mod tests {
    use crate::{
        guest_star::{
            request::{GustStarSettingRequest, UpdateSlotSettingsRequest},
            types::GroupLayout,
            GuestStarAPI,
        },
        types::{BroadcasterId, ModeratorId},
    };

    api_test!(
        get_channel_guest_star_settings,
        [
            &BroadcasterId::from("932104"),
            &ModeratorId::from("9321049")
        ]
    );
    api_test!(
        update_channel_guest_star_settings,
        [
            &BroadcasterId::from("9321049"),
            GustStarSettingRequest::new().group_layout(GroupLayout::TILED_LAYOUT)
        ]
    );
    api_test!(
        get_guest_star_session,
        [
            &BroadcasterId::from("9321049"),
            &ModeratorId::from("9321049")
        ]
    );
    api_test!(create_guest_star_session, [&BroadcasterId::from("9321049")]);
    api_test!(
        end_guest_star_session,
        [
            &BroadcasterId::from("9321049"),
            "2KFRQbFtpmfyD3IevNRnCzOPRJI"
        ]
    );
    api_test!(
        get_guest_star_invites,
        [
            &BroadcasterId::from("9321049"),
            &ModeratorId::from("9321049"),
            "2KFRQbFtpmfyD3IevNRnCzOPRJI"
        ]
    );
    api_test!(
        send_guest_star_invite,
        [
            &BroadcasterId::from("9321049"),
            &ModeratorId::from("9321049"),
            "2KFRQbFtpmfyD3IevNRnCzOPRJI",
            "144601104"
        ]
    );
    api_test!(
        delete_guest_star_invite,
        [
            &BroadcasterId::from("9321049"),
            &ModeratorId::from("9321049"),
            "2KFRQbFtpmfyD3IevNRnCzOPRJI",
            "144601104"
        ]
    );
    api_test!(
        assign_guest_star_slot,
        [
            &BroadcasterId::from("9321049"),
            &ModeratorId::from("9321049"),
            "2KFRQbFtpmfyD3IevNRnCzOPRJI",
            "144601104",
            "1"
        ]
    );
    api_test!(
        update_guest_star_slot,
        [
            &BroadcasterId::from("9321049"),
            &ModeratorId::from("9321049"),
            "2KFRQbFtpmfyD3IevNRnCzOPRJI",
            "1",
            Some("2")
        ]
    );
    api_test!(
        delete_guest_star_slot,
        [
            &BroadcasterId::from("9321049"),
            &ModeratorId::from("9321049"),
            "2KFRQbFtpmfyD3IevNRnCzOPRJI",
            "144601104",
            "1",
            None
        ]
    );
    api_test!(
        update_guest_star_slot_settings,
        [
            &BroadcasterId::from("9321049"),
            &ModeratorId::from("9321049"),
            "2KFRQbFtpmfyD3IevNRnCzOPRJI",
            "1",
            UpdateSlotSettingsRequest::new().is_audio_enabled(false)
        ]
    );

    api_test!(extra
        update_channel_guest_star_settings,
        update_channel_guest_star_settings2,
        [
            &BroadcasterId::from("9321049"),
            GustStarSettingRequest::new().is_moderator_send_live_enabled(false)
        ]
    );
    api_test!(extra
        update_channel_guest_star_settings,
        update_channel_guest_star_settings3,
        [
            &BroadcasterId::from("9321049"),
            GustStarSettingRequest::new().slot_count(6)
        ]
    );
    api_test!(extra
        update_channel_guest_star_settings,
        update_channel_guest_star_settings4,
        [
            &BroadcasterId::from("9321049"),
            GustStarSettingRequest::new().regenerate_browser_sources(true)
        ]
    );

    api_test!(extra
        update_guest_star_slot_settings,
        update_guest_star_slot_settings2,
        [
            &BroadcasterId::from("9321049"),
            &ModeratorId::from("9321049"),
            "2KFRQbFtpmfyD3IevNRnCzOPRJI",
            "1",
            UpdateSlotSettingsRequest::new().is_live(true)
        ]
    );
}
