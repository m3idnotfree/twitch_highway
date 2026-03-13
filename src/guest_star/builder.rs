use crate::{
    guest_star::GroupLayout,
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
    Client, Error,
};

#[derive(Debug)]
pub struct UpdateChannelGuestStarSettings<'a> {
    client: &'a Client,
    broadcaster_id: &'a BroadcasterId,
    is_moderator_send_live_enabled: Option<bool>,
    slot_count: Option<u64>,
    is_browser_source_audio_enabled: Option<bool>,
    group_layout: Option<GroupLayout>,
    regenerate_browser_sources: Option<bool>,
}

impl<'a> UpdateChannelGuestStarSettings<'a> {
    pub fn new(client: &'a Client, broadcaster_id: &'a BroadcasterId) -> Self {
        Self {
            client,
            broadcaster_id,
            is_moderator_send_live_enabled: None,
            slot_count: None,
            is_browser_source_audio_enabled: None,
            group_layout: None,
            regenerate_browser_sources: None,
        }
    }

    pub fn is_moderator_send_live_enabled(mut self, value: bool) -> Self {
        self.is_moderator_send_live_enabled = Some(value);
        self
    }

    pub fn slot_count(mut self, value: u64) -> Self {
        self.slot_count = Some(value);
        self
    }

    pub fn is_browser_source_audio_enabled(mut self, value: bool) -> Self {
        self.is_browser_source_audio_enabled = Some(value);
        self
    }

    pub fn group_layout(mut self, value: GroupLayout) -> Self {
        self.group_layout = Some(value);
        self
    }

    pub fn regenerate_browser_sources(mut self, value: bool) -> Self {
        self.regenerate_browser_sources = Some(value);
        self
    }

    pub async fn send(self) -> Result<(), Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([GUEST_STAR, CHANNEL_SETTINGS]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, self.broadcaster_id);
        if let Some(val) = self.is_moderator_send_live_enabled {
            url.query_pairs_mut()
                .append_pair(IS_MODERATOR_SEND_LIVE_ENABLED, &val.to_string());
        }
        if let Some(val) = self.slot_count {
            url.query_pairs_mut()
                .append_pair(SLOT_COUNT, &val.to_string());
        }
        if let Some(val) = self.is_browser_source_audio_enabled {
            url.query_pairs_mut()
                .append_pair(IS_BROWSER_SOURCE_AUDIO_ENABLED, &val.to_string());
        }
        if let Some(val) = self.group_layout {
            url.query_pairs_mut()
                .append_pair(GROUP_LAYOUT, val.as_ref());
        }
        if let Some(val) = self.regenerate_browser_sources {
            url.query_pairs_mut()
                .append_pair(REGENERATE_BROWSER_SOURCES, &val.to_string());
        }

        let req = self.client.http_client().put(url);
        self.client.no_content(req).await
    }
}

#[derive(Debug)]

pub struct UpdateGuestStarSlot<'a> {
    client: &'a Client,
    broadcaster_id: &'a BroadcasterId,
    moderator_id: &'a ModeratorId,
    session_id: &'a SessionId,
    source_slot_id: &'a str,
    destination_slot_id: Option<&'a str>,
}

impl<'a> UpdateGuestStarSlot<'a> {
    pub fn new(
        client: &'a Client,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
        session_id: &'a SessionId,
        source_slot_id: &'a str,
    ) -> Self {
        Self {
            client,
            broadcaster_id,
            moderator_id,
            session_id,
            source_slot_id,
            destination_slot_id: None,
        }
    }

    pub fn destination_slot_id(mut self, value: &'a str) -> Self {
        self.destination_slot_id = Some(value);
        self
    }

    pub async fn send(self) -> Result<(), Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut().unwrap().extend([GUEST_STAR, SLOT]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, self.broadcaster_id)
            .append_pair(MODERATOR_ID, self.moderator_id)
            .append_pair(SESSION_ID, self.session_id)
            .append_pair(SOURCE_SLOT_ID, self.source_slot_id);
        if let Some(val) = self.destination_slot_id {
            url.query_pairs_mut().append_pair(DESTINATION_SLOT_ID, val);
        }

        let req = self.client.http_client().patch(url);
        self.client.no_content(req).await
    }
}

#[derive(Debug)]
pub struct DeleteGuestStarSlot<'a> {
    client: &'a Client,
    broadcaster_id: &'a BroadcasterId,
    moderator_id: &'a ModeratorId,
    session_id: &'a SessionId,
    guest_id: &'a UserId,
    slot_id: &'a str,
    should_reinvite_guest: Option<&'a str>,
}

impl<'a> DeleteGuestStarSlot<'a> {
    pub fn new(
        client: &'a Client,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
        session_id: &'a SessionId,
        guest_id: &'a UserId,
        slot_id: &'a str,
    ) -> Self {
        Self {
            client,
            broadcaster_id,
            moderator_id,
            session_id,
            guest_id,
            slot_id,
            should_reinvite_guest: None,
        }
    }

    pub fn should_reinvite_guest(mut self, value: &'a str) -> Self {
        self.should_reinvite_guest = Some(value);
        self
    }

    pub async fn send(self) -> Result<(), Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut().unwrap().extend([GUEST_STAR, SLOT]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, self.broadcaster_id)
            .append_pair(MODERATOR_ID, self.moderator_id)
            .append_pair(SESSION_ID, self.session_id)
            .append_pair(GUEST_ID, self.guest_id)
            .append_pair(SLOT_ID, self.slot_id);

        if let Some(val) = self.should_reinvite_guest {
            url.query_pairs_mut()
                .append_pair(SHOULD_REINVITE_GUEST, val);
        }

        let req = self.client.http_client().delete(url);
        self.client.no_content(req).await
    }
}

#[derive(Debug)]
pub struct UpdateGuestStarSlotSettings<'a> {
    client: &'a Client,
    broadcaster_id: &'a BroadcasterId,
    moderator_id: &'a ModeratorId,
    session_id: &'a SessionId,
    slot_id: &'a str,
    is_audio_enabled: Option<bool>,
    is_video_enabled: Option<bool>,
    is_live: Option<bool>,
    volume: Option<u64>,
}

impl<'a> UpdateGuestStarSlotSettings<'a> {
    pub fn new(
        client: &'a Client,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
        session_id: &'a SessionId,
        slot_id: &'a str,
    ) -> Self {
        Self {
            client,
            broadcaster_id,
            moderator_id,
            session_id,
            slot_id,
            is_audio_enabled: None,
            is_video_enabled: None,
            is_live: None,
            volume: None,
        }
    }

    pub fn is_audio_enabled(mut self, value: bool) -> Self {
        self.is_audio_enabled = Some(value);
        self
    }

    pub fn is_video_enabled(mut self, value: bool) -> Self {
        self.is_video_enabled = Some(value);
        self
    }

    pub fn is_live(mut self, value: bool) -> Self {
        self.is_live = Some(value);
        self
    }

    pub fn volume(mut self, value: u64) -> Self {
        self.volume = Some(value);
        self
    }

    pub async fn send(self) -> Result<(), Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([GUEST_STAR, SLOT_SETTINGS]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, self.broadcaster_id)
            .append_pair(MODERATOR_ID, self.moderator_id)
            .append_pair(SESSION_ID, self.session_id)
            .append_pair(SLOT_ID, self.slot_id);
        if let Some(val) = self.is_audio_enabled {
            url.query_pairs_mut()
                .append_pair(IS_AUDIO_ENABLED, &val.to_string());
        }
        if let Some(val) = self.is_video_enabled {
            url.query_pairs_mut()
                .append_pair(IS_VIDEO_ENABLED, &val.to_string());
        }
        if let Some(val) = self.is_live {
            url.query_pairs_mut().append_pair(IS_LIVE, &val.to_string());
        }
        if let Some(val) = self.volume {
            url.query_pairs_mut().append_pair(VOLUME, &val.to_string());
        }

        let req = self.client.http_client().patch(url);
        self.client.no_content(req).await
    }
}
