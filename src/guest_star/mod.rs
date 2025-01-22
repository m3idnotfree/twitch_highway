use asknothingx2_util::api::Method;
use request::{GustStarSettingRequest, UpdateSlotSettingsRequest};
use response::{GuestStarSettingsResponse, GustStarInvitesResponse, GustStarSessionResponse};

use crate::{
    base::TwitchAPIBase,
    request::{EmptyBody, EndpointType, TwitchAPIRequest},
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

#[cfg_attr(docsrs, doc(cfg(feature = "guest-star")))]
pub trait GuestStarAPI: TwitchAPIBase {
    /// <https://dev.twitch.tv/docs/api/reference/#get-channel-guest-star-settings>
    fn get_channel_guest_star_settings(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
    ) -> TwitchAPIRequest<EmptyBody, GuestStarSettingsResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#update-channel-guest-star-settings>
    fn update_channel_guest_star_settings(
        &self,
        broadcaster_id: BroadcasterId,
        request: GustStarSettingRequest,
    ) -> TwitchAPIRequest<EmptyBody, EmptyBody>;
    /// <https://dev.twitch.tv/docs/api/reference/#get-guest-star-session>
    fn get_guest_star_session(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
    ) -> TwitchAPIRequest<EmptyBody, GustStarSessionResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#create-guest-star-session>
    fn create_guest_star_session(
        &self,
        broadcaster_id: BroadcasterId,
    ) -> TwitchAPIRequest<EmptyBody, GustStarSessionResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#end-guest-star-session>
    fn end_guest_star_session(
        &self,
        broadcaster_id: BroadcasterId,
        session_id: String,
    ) -> TwitchAPIRequest<EmptyBody, GustStarSessionResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#get-guest-star-invites>
    fn get_guest_star_invites(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        session_id: &str,
    ) -> TwitchAPIRequest<EmptyBody, GustStarInvitesResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#send-guest-star-invite>
    fn send_guest_star_invites(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        session_id: &str,
        guest_id: &str,
    ) -> TwitchAPIRequest<EmptyBody, EmptyBody>;
    /// <https://dev.twitch.tv/docs/api/reference/#delete-guest-star-invite>
    fn delete_guest_star_invites(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        session_id: &str,
        guest_id: &str,
    ) -> TwitchAPIRequest<EmptyBody, EmptyBody>;
    /// <https://dev.twitch.tv/docs/api/reference/#assign-guest-star-slot>
    fn assign_guest_star_slot(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        session_id: &str,
        guest_id: &str,
        slot_id: &str,
    ) -> TwitchAPIRequest<EmptyBody, EmptyBody>;
    /// <https://dev.twitch.tv/docs/api/reference/#update-guest-star-slot>
    fn update_guest_star_slot(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        session_id: &str,
        source_slot_id: &str,
        destination_slot_id: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody, EmptyBody>;
    /// <https://dev.twitch.tv/docs/api/reference/#delete-guest-star-slot>
    fn delete_guest_star_slot(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        session_id: &str,
        guest_id: &str,
        slot_id: &str,
        should_reinvite_guest: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody, EmptyBody>;
    /// <https://dev.twitch.tv/docs/api/reference/#update-guest-star-slot-settings>
    fn update_guest_star_slot_settings(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        session_id: &str,
        slot_id: &str,
        opts: UpdateSlotSettingsRequest,
    ) -> TwitchAPIRequest<EmptyBody, EmptyBody>;
}

impl GuestStarAPI for TwitchAPI {
    fn get_channel_guest_star_settings(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
    ) -> TwitchAPIRequest<EmptyBody, GuestStarSettingsResponse> {
        let mut url = self.build_url();
        url.path([GUEST_STAR, "channel_settings"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(MODERATOR_ID, moderator_id);

        TwitchAPIRequest::new(
            EndpointType::GetChannelGuestStarSettings,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn update_channel_guest_star_settings(
        &self,
        broadcaster_id: BroadcasterId,
        request: GustStarSettingRequest,
    ) -> TwitchAPIRequest<EmptyBody, EmptyBody> {
        let mut url = self.build_url();
        url.path([GUEST_STAR, "channel_settings"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query_pairs(request);

        TwitchAPIRequest::new(
            EndpointType::UpdateChannelGuestStarSettings,
            url.build(),
            Method::PUT,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn get_guest_star_session(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
    ) -> TwitchAPIRequest<EmptyBody, GustStarSessionResponse> {
        let mut url = self.build_url();
        url.path([GUEST_STAR, "session"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(MODERATOR_ID, moderator_id);

        TwitchAPIRequest::new(
            EndpointType::GetGuestStarSession,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn create_guest_star_session(
        &self,
        broadcaster_id: BroadcasterId,
    ) -> TwitchAPIRequest<EmptyBody, GustStarSessionResponse> {
        let mut url = self.build_url();
        url.path([GUEST_STAR, "session"])
            .query(BROADCASTER_ID, broadcaster_id);

        TwitchAPIRequest::new(
            EndpointType::CreateGuestStarSession,
            url.build(),
            Method::POST,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn end_guest_star_session(
        &self,
        broadcaster_id: BroadcasterId,
        session_id: String,
    ) -> TwitchAPIRequest<EmptyBody, GustStarSessionResponse> {
        let mut url = self.build_url();
        url.path([GUEST_STAR, "session"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query("session_id", session_id);

        TwitchAPIRequest::new(
            EndpointType::EndGuestStarSession,
            url.build(),
            Method::DELETE,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn get_guest_star_invites(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        session_id: &str,
    ) -> TwitchAPIRequest<EmptyBody, GustStarInvitesResponse> {
        let mut url = self.build_url();
        url.path([GUEST_STAR, "invites"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(MODERATOR_ID, moderator_id)
            .query("session_id", session_id);

        TwitchAPIRequest::new(
            EndpointType::GetGuestStarInvites,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn send_guest_star_invites(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        session_id: &str,
        guest_id: &str,
    ) -> TwitchAPIRequest<EmptyBody, EmptyBody> {
        let mut url = self.build_url();
        url.path([GUEST_STAR, "invites"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(MODERATOR_ID, moderator_id)
            .query("session_id", session_id)
            .query("guest_id", guest_id);

        TwitchAPIRequest::new(
            EndpointType::SendGuestStarInvite,
            url.build(),
            Method::POST,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn delete_guest_star_invites(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        session_id: &str,
        guest_id: &str,
    ) -> TwitchAPIRequest<EmptyBody, EmptyBody> {
        let mut url = self.build_url();
        url.path([GUEST_STAR, "invites"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(MODERATOR_ID, moderator_id)
            .query("session_id", session_id)
            .query("guest_id", guest_id);

        TwitchAPIRequest::new(
            EndpointType::DeleteGuestStarInvite,
            url.build(),
            Method::DELETE,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn assign_guest_star_slot(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        session_id: &str,
        guest_id: &str,
        slot_id: &str,
    ) -> TwitchAPIRequest<EmptyBody, EmptyBody> {
        let mut url = self.build_url();
        url.path([GUEST_STAR, "slot"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(MODERATOR_ID, moderator_id)
            .query("session_id", session_id)
            .query("guest_id", guest_id)
            .query("slot_id", slot_id);

        TwitchAPIRequest::new(
            EndpointType::AssignGuestStarSlot,
            url.build(),
            Method::POST,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn update_guest_star_slot(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        session_id: &str,
        source_slot_id: &str,
        destination_slot_id: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody, EmptyBody> {
        let mut url = self.build_url();
        url.path([GUEST_STAR, "slot"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(MODERATOR_ID, moderator_id)
            .query("session_id", session_id)
            .query("source_slot_id", source_slot_id)
            .query_opt("destination_slot_id", destination_slot_id);

        TwitchAPIRequest::new(
            EndpointType::UpdateGuestStarSlot,
            url.build(),
            Method::PATCH,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn delete_guest_star_slot(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        session_id: &str,
        guest_id: &str,
        slot_id: &str,
        should_reinvite_guest: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody, EmptyBody> {
        let mut url = self.build_url();
        url.path([GUEST_STAR, "slot"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(MODERATOR_ID, moderator_id)
            .query("session_id", session_id)
            .query("guest_id", guest_id)
            .query("slot_id", slot_id)
            .query_opt("should_reinvite_guest", should_reinvite_guest);

        TwitchAPIRequest::new(
            EndpointType::DeleteGuestStarSlot,
            url.build(),
            Method::DELETE,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn update_guest_star_slot_settings(
        &self,
        broadcaster_id: BroadcasterId,
        moderator_id: ModeratorId,
        session_id: &str,
        slot_id: &str,
        request: UpdateSlotSettingsRequest,
    ) -> TwitchAPIRequest<EmptyBody, EmptyBody> {
        let mut url = self.build_url();
        url.path([GUEST_STAR, "slot_settings"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(MODERATOR_ID, moderator_id)
            .query("session_id", session_id)
            .query("slot_id", slot_id)
            .query_pairs(request);

        TwitchAPIRequest::new(
            EndpointType::UpdateGuestStarSlotSettings,
            url.build(),
            Method::PATCH,
            self.build_headers().build(),
            EmptyBody,
        )
    }
}
