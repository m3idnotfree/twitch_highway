use serde::{Deserialize, Serialize};

use crate::base::{IntoQueryPairs, QueryParams};

use super::types::GroupLayout;

request_struct!(
    #[derive(Debug, Default, Serialize, Deserialize)]
    GustStarSettingRequest {
        #[serde(skip_serializing_if = "Option::is_none")]
        is_moderator_send_live_enabled: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        slot_count: u64,
        #[serde(skip_serializing_if = "Option::is_none")]
        is_browser_source_audio_enabled: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        group_layout: GroupLayout,
        #[serde(skip_serializing_if = "Option::is_none")]
        regenerate_browser_sources: bool,
    }
);

impl IntoQueryPairs for GustStarSettingRequest {
    fn into_query_pairs(self) -> Vec<(&'static str, String)> {
        let mut params = QueryParams::new();
        params
            .push_opt(
                "is_moderator_send_live_enabled",
                self.is_moderator_send_live_enabled.map(|x| x.to_string()),
            )
            .push_opt("slot_count", self.slot_count.map(|x| x.to_string()))
            .push_opt(
                "is_browser_source_audio_enabled",
                self.is_browser_source_audio_enabled.map(|x| x.to_string()),
            )
            .push_opt("group_layout", self.group_layout)
            .push_opt(
                "regenerate_browser_sources",
                self.regenerate_browser_sources.map(|x| x.to_string()),
            );

        params.build()
    }
}
request_struct!(
    #[derive(Debug, Default, Serialize, Deserialize)]
    UpdateSlotSettingsRequest {
        is_audio_enabled: bool,
        is_video_enabled: bool,
        is_live: bool,
        volume: u64
    }
);

impl IntoQueryPairs for UpdateSlotSettingsRequest {
    fn into_query_pairs(self) -> Vec<(&'static str, String)> {
        let mut params = QueryParams::new();

        params
            .push_opt(
                "is_audio_enabled",
                self.is_audio_enabled.map(|x| x.to_string()),
            )
            .push_opt(
                "is_video_enabled",
                self.is_video_enabled.map(|x| x.to_string()),
            )
            .push_opt("is_live", self.is_live.map(|x| x.to_string()))
            .push_opt("volume", self.volume.map(|x| x.to_string()));

        params.build()
    }
}
