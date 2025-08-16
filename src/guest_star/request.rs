use serde::{Deserialize, Serialize};

use super::types::GroupLayout;

define_request!(
    #[derive(Serialize, Deserialize)]
    GustStarSettingRequest {
        opts: {
            #[serde(skip_serializing_if = "Option::is_none")]
            is_moderator_send_live_enabled: bool ; bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            slot_count: u64 ; u64,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_browser_source_audio_enabled: bool ; bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            group_layout: GroupLayout,
            #[serde(skip_serializing_if = "Option::is_none")]
            regenerate_browser_sources: bool ; bool,
        };
        into_query
    }
);

define_request!(
    #[derive(Serialize, Deserialize)]
    UpdateSlotSettingsRequest {
        opts: {
            is_audio_enabled: bool ; bool,
            is_video_enabled: bool ; bool,
            is_live: bool ; bool,
            volume: u64 ; u64
        };
        into_query
    }
);
