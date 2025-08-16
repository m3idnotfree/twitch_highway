use asknothingx2_util::api::Method;
use request::{GustStarSettingRequest, UpdateSlotSettingsRequest};
use response::{GuestStarSettingsResponse, GustStarInvitesResponse, GustStarSessionResponse};

use crate::{
    request::{EndpointType, NoContent, TwitchAPIRequest},
    types::{
        constants::{BROADCASTER_ID, MODERATOR_ID},
        BroadcasterId, ModeratorId,
    },
    TwitchAPI,
};

pub mod request;
pub mod response;
pub mod types;

const GUEST_STAR: &str = "guest_star";

twitch_api_trait! {
    #[cfg_attr(docsrs, doc(cfg(feature = "guest-star")))]
    trait GuestStarAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#get-channel-guest-star-settings>
        fn get_channel_guest_star_settings(
            &self,
            broadcaster_id: BroadcasterId,
            moderator_id: ModeratorId,
        ) -> GuestStarSettingsResponse;
        /// <https://dev.twitch.tv/docs/api/reference/#update-channel-guest-star-settings>
        fn update_channel_guest_star_settings(
            &self,
            broadcaster_id: BroadcasterId,
            request: GustStarSettingRequest,
        ) -> NoContent;
        /// <https://dev.twitch.tv/docs/api/reference/#get-guest-star-session>
        fn get_guest_star_session(
            &self,
            broadcaster_id: BroadcasterId,
            moderator_id: ModeratorId,
        ) -> GustStarSessionResponse;
        /// <https://dev.twitch.tv/docs/api/reference/#create-guest-star-session>
        fn create_guest_star_session(
            &self,
            broadcaster_id: BroadcasterId,
        ) -> GustStarSessionResponse;
        /// <https://dev.twitch.tv/docs/api/reference/#end-guest-star-session>
        fn end_guest_star_session(
            &self,
            broadcaster_id: BroadcasterId,
            session_id: &str,
        ) -> GustStarSessionResponse;
        /// <https://dev.twitch.tv/docs/api/reference/#get-guest-star-invites>
        fn get_guest_star_invites(
            &self,
            broadcaster_id: BroadcasterId,
            moderator_id: ModeratorId,
            session_id: &str,
        ) -> GustStarInvitesResponse;
        /// <https://dev.twitch.tv/docs/api/reference/#send-guest-star-invite>
        fn send_guest_star_invites(
            &self,
            broadcaster_id: BroadcasterId,
            moderator_id: ModeratorId,
            session_id: &str,
            guest_id: &str,
        ) -> NoContent;
        /// <https://dev.twitch.tv/docs/api/reference/#delete-guest-star-invite>
        fn delete_guest_star_invites(
            &self,
            broadcaster_id: BroadcasterId,
            moderator_id: ModeratorId,
            session_id: &str,
            guest_id: &str,
        ) -> NoContent;
        /// <https://dev.twitch.tv/docs/api/reference/#assign-guest-star-slot>
        fn assign_guest_star_slot(
            &self,
            broadcaster_id: BroadcasterId,
            moderator_id: ModeratorId,
            session_id: &str,
            guest_id: &str,
            slot_id: &str,
        ) -> NoContent;
        /// <https://dev.twitch.tv/docs/api/reference/#update-guest-star-slot>
        fn update_guest_star_slot(
            &self,
            broadcaster_id: BroadcasterId,
            moderator_id: ModeratorId,
            session_id: &str,
            source_slot_id: &str,
            destination_slot_id: Option<&str>,
        ) -> NoContent;
        /// <https://dev.twitch.tv/docs/api/reference/#delete-guest-star-slot>
        fn delete_guest_star_slot(
            &self,
            broadcaster_id: BroadcasterId,
            moderator_id: ModeratorId,
            session_id: &str,
            guest_id: &str,
            slot_id: &str,
            should_reinvite_guest: Option<&str>,
        ) -> NoContent;
        /// <https://dev.twitch.tv/docs/api/reference/#update-guest-star-slot-settings>
        fn update_guest_star_slot_settings(
            &self,
            broadcaster_id: BroadcasterId,
            moderator_id: ModeratorId,
            session_id: &str,
            slot_id: &str,
            opts: UpdateSlotSettingsRequest,
        ) -> NoContent;
    }
    impl {
        get_channel_guest_star_settings => {
            endpoint_type: EndpointType::GetChannelGuestStarSettings,
            method: Method::GET,
            path: [GUEST_STAR, "channel_settings"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(MODERATOR_ID, moderator_id)
            }
        }
        update_channel_guest_star_settings => {
            endpoint_type: EndpointType::UpdateChannelGuestStarSettings,
            method: Method::PUT,
            path: [GUEST_STAR, "channel_settings"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                into_query(request)
            }
        }
        get_guest_star_session => {
            endpoint_type: EndpointType::GetGuestStarSession,
            method: Method::GET,
            path: [GUEST_STAR, "session"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(MODERATOR_ID, moderator_id)
            }
        }
        create_guest_star_session => {
            endpoint_type: EndpointType::CreateGuestStarSession,
            method: Method::POST,
            path: [GUEST_STAR, "session"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id)
            }
        }
        end_guest_star_session => {
            endpoint_type: EndpointType::EndGuestStarSession,
            method: Method::DELETE,
            path: [GUEST_STAR, "session"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query("session_id", session_id)
            }
        }
        get_guest_star_invites => {
            endpoint_type: EndpointType::GetGuestStarInvites,
            method: Method::GET,
            path: [GUEST_STAR, "invites"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(MODERATOR_ID, moderator_id),
                query("session_id", session_id)
            }
        }
        send_guest_star_invites => {
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
        delete_guest_star_invites => {
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
        assign_guest_star_slot => {
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
        update_guest_star_slot => {
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
        delete_guest_star_slot => {
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
        update_guest_star_slot_settings => {
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
        guest_star::GuestStarAPI,
        test_utils::TwitchApiTest,
        types::{BroadcasterId, ModeratorId},
    };

    #[tokio::test]
    async fn get_channel_guest_star_settings_endpoint() {
        let suite = TwitchApiTest::new().await;

        suite.mock_guest_star_success().await;

        let response = suite
            .execute("/guest_star/channel_settings", |api| {
                api.get_channel_guest_star_settings(
                    BroadcasterId::new("123456789"),
                    ModeratorId::new("987654321"),
                )
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 1);

        let settings = &response.data[0];
        assert!(settings.is_moderator_send_live_enabled);
        assert_eq!(settings.slot_count, 6);
        assert!(!settings.is_browser_source_audio_enabled);
        assert_eq!(settings.group_layout.as_str(), "HORIZONTAL_LAYOUT");
        assert_eq!(settings.browser_source_token, "secure_token_abc123");
    }

    #[tokio::test]
    async fn get_guest_star_session_endpoint() {
        let suite = TwitchApiTest::new().await;

        suite.mock_guest_star_success().await;

        let response = suite
            .execute("/guest_star/session", |api| {
                api.get_guest_star_session(
                    BroadcasterId::new("123456789"),
                    ModeratorId::new("987654321"),
                )
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 1);

        let session = &response.data[0];
        assert_eq!(session.id.as_str(), "live_session_789");
        assert_eq!(session.guests.len(), 2);

        let first_guest = &session.guests[0];
        assert_eq!(first_guest.slot_id, Some("1".to_string()));
        assert_eq!(first_guest.user_id.as_str(), "guest_user_123");
        assert_eq!(first_guest.user_display_name, "ActiveGuest");
        assert!(first_guest.is_live);
        assert_eq!(first_guest.volume, 85);
        assert!(first_guest.audio_settings.is_guest_enabled);
        assert!(!first_guest.video_settings.is_guest_enabled);

        let second_guest = &session.guests[1];
        assert_eq!(second_guest.slot_id, Some("2".to_string()));
        assert_eq!(second_guest.user_id.as_str(), "guest_user_456");
        assert_eq!(second_guest.user_display_name, "SecondGuest");
        assert!(!second_guest.is_live);
        assert_eq!(second_guest.volume, 60);
        assert!(!second_guest.video_settings.is_available);
    }

    #[tokio::test]
    async fn get_guest_star_invites_endpoint() {
        let suite = TwitchApiTest::new().await;

        suite.mock_guest_star_success().await;

        let response = suite
            .execute("/guest_star/invites", |api| {
                api.get_guest_star_invites(
                    BroadcasterId::new("123456789"),
                    ModeratorId::new("987654321"),
                    "session123",
                )
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 2);

        let first_invite = &response.data[0];
        assert_eq!(first_invite.user_id.as_str(), "pending_user_1");
        assert_eq!(first_invite.invited_at, "2024-01-15T16:00:00Z");
        assert!(first_invite.is_audio_enabled);
        assert!(first_invite.is_video_enabled);

        let second_invite = &response.data[1];
        assert_eq!(second_invite.user_id.as_str(), "ready_user_2");
        assert_eq!(second_invite.invited_at, "2024-01-15T15:30:00Z");
        assert!(second_invite.is_audio_enabled);
        assert!(!second_invite.is_video_enabled);
    }

    #[tokio::test]
    async fn create_guest_star_session_endpoint() {
        let suite = TwitchApiTest::new().await;

        suite.mock_guest_star_success().await;

        let response = suite
            .execute("/guest_star/session", |api| {
                api.create_guest_star_session(BroadcasterId::new("123456789"))
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 1);

        let session = &response.data[0];
        assert_eq!(session.id.as_str(), "new_session_abc123");
        assert_eq!(session.guests.len(), 0);
    }

    #[tokio::test]
    async fn guest_star_api_error_response() {
        let suite = TwitchApiTest::new().await;

        suite.mock_guest_star_failure().await;

        let response = suite
            .execute("/guest_star/channel_settings", |api| {
                api.get_channel_guest_star_settings(
                    BroadcasterId::new("123456789"),
                    ModeratorId::new("987654321"),
                )
            })
            .send()
            .await;

        match response {
            Ok(response) => {
                panic!("Expected Error, got: {response:?}")
            }
            Err(e) => {
                assert!(e.is_api());
            }
        }
    }
}
