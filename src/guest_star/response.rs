use serde::{Deserialize, Serialize};

use super::types::{GuestStarInvite, GustStarSession, GustStarSetting};

#[derive(Debug, Serialize, Deserialize)]
pub struct GuestStarSettingsResponse {
    pub data: Vec<GustStarSetting>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GustStarSessionResponse {
    pub data: Vec<GustStarSession>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GustStarInvitesResponse {
    pub data: Vec<GuestStarInvite>,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::guest_star::response::{
        GuestStarSettingsResponse, GustStarInvitesResponse, GustStarSessionResponse,
    };

    #[test]
    fn guest_star_settings_response_deserialization() {
        let json_data = json!({
            "data": [
                {
                    "is_moderator_send_live_enabled": true,
                    "slot_count": 6,
                    "is_browser_source_audio_enabled": false,
                    "group_layout": "TILED_LAYOUT",
                    "browser_source_token": "token123"
                }
            ]
        });

        let response: GuestStarSettingsResponse = serde_json::from_value(json_data).unwrap();

        assert_eq!(response.data.len(), 1);

        let setting = &response.data[0];
        assert!(setting.is_moderator_send_live_enabled);
        // assert_eq!(setting.slot_count, 6);
        // assert!(!setting.is_browser_source_audio_enabled);
        // assert_eq!(setting.group_layout.as_str(), "TILED_LAYOUT");
        // assert_eq!(setting.browser_source_token, "token123");
    }

    #[test]
    fn guest_star_session_response_deserialization() {
        let json_data = json!({
            "data": [
                {
                    "id": "session456",
                    "guests": [
                        {
                            "slot_id": "slot_1",
                            "user_id": "user789",
                            "user_display_name": "TestGuest",
                            "user_login": "testguest",
                            "is_live": true,
                            "volume": 65,
                            "assigned_at": "2024-01-15T10:30:00Z",
                            "audio_settings": {
                                "is_available": true,
                                "is_host_enabled": true,
                                "is_guest_enabled": true
                            },
                            "video_settings": {
                                "is_available": true,
                                "is_host_enabled": false,
                                "is_guest_enabled": true
                            }
                        }
                    ]
                }
            ]
        });

        let response: GustStarSessionResponse = serde_json::from_value(json_data).unwrap();

        assert_eq!(response.data.len(), 1);

        let session = &response.data[0];
        assert_eq!(session.id.as_str(), "session456");
        assert_eq!(session.guests.len(), 1);

        let guest = &session.guests[0];
        assert_eq!(guest.slot_id, Some("slot_1".to_string()));
        assert_eq!(guest.user_id.as_str(), "user789");
        assert_eq!(guest.user_display_name, "TestGuest");
        assert_eq!(guest.user_login, "testguest");
        assert!(guest.is_live);
        assert_eq!(guest.volume, 65);
        assert!(guest.audio_settings.is_guest_enabled);
        assert!(!guest.video_settings.is_host_enabled);
    }

    #[test]
    fn guest_star_invites_response_deserialization() {
        let json_data = json!({
            "data": [
                {
                    "user_id": "invite_user_1",
                    "invited_at": "2024-01-15T10:00:00Z",
                    "status": "INVITED",
                    "is_audio_enabled": true,
                    "is_video_enabled": true,
                    "is_audio_available": true,
                    "is_video_available": true
                },
                {
                    "user_id": "invite_user_2",
                    "invited_at": "2024-01-15T10:15:00Z",
                    "status": "ACCEPTED",
                    "is_audio_enabled": false,
                    "is_video_enabled": true,
                    "is_audio_available": true,
                    "is_video_available": false
                }
            ]
        });

        let response: GustStarInvitesResponse = serde_json::from_value(json_data).unwrap();

        assert_eq!(response.data.len(), 2);

        let first_invite = &response.data[0];
        assert_eq!(first_invite.user_id.as_str(), "invite_user_1");
        assert_eq!(first_invite.invited_at, "2024-01-15T10:00:00Z");
        assert!(first_invite.is_audio_enabled);
        assert!(first_invite.is_video_enabled);
        assert!(first_invite.is_audio_available);
        assert!(first_invite.is_video_available);

        let second_invite = &response.data[1];
        assert_eq!(second_invite.user_id.as_str(), "invite_user_2");
        assert_eq!(second_invite.invited_at, "2024-01-15T10:15:00Z");
        assert!(!second_invite.is_audio_enabled);
        assert!(second_invite.is_video_enabled);
        assert!(second_invite.is_audio_available);
        assert!(!second_invite.is_video_available);
    }
}
