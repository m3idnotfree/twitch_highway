use serde::Serialize;

use crate::{
    moderation::{
        AutoModSettingsResponse, BanUsersResponse, BlockedTermsResponse, GetBannedUsersResponse,
        ModeratedChannelResponse, ModeratorsResponse, UnbanRequestResponse, UnbanRequestStatus,
    },
    request::{NoContent, TwitchAPIRequest},
    types::{
        constants::{
            AFTER, AUTOMOD, BANNED, BANS, BEFORE, BLOCKED_TERMS, BROADCASTER_ID, CHANNELS, CHAT,
            FIRST, MODERATION, MODERATORS, MODERATOR_ID, SETTINGS, UNBAN_REQUESTS, USER_ID, VIPS,
        },
        BroadcasterId, ModeratorId, UserId,
    },
    TwitchAPI,
};

#[derive(Debug, Serialize)]
pub struct UpdateAutomodSettingsBuilder<'a> {
    #[serde(skip)]
    api: &'a TwitchAPI,
    #[serde(skip)]
    broadcaster_id: &'a BroadcasterId,
    #[serde(skip)]
    moderator_id: &'a ModeratorId,

    #[serde(skip_serializing_if = "Option::is_none")]
    aggression: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bullying: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disability: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    misogyny: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    overall_level: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    race_ethnicity_or_religion: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sex_based_terms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sexuality_sex_or_gender: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    swearing: Option<u64>,
}

impl<'a> UpdateAutomodSettingsBuilder<'a> {
    pub fn new(
        api: &'a TwitchAPI,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
    ) -> Self {
        Self {
            api,
            broadcaster_id,
            moderator_id,
            aggression: None,
            bullying: None,
            disability: None,
            misogyny: None,
            overall_level: None,
            race_ethnicity_or_religion: None,
            sex_based_terms: None,
            sexuality_sex_or_gender: None,
            swearing: None,
        }
    }
    opt_method!(aggression, u64);
    opt_method!(bullying, u64);
    opt_method!(disability, u64);
    opt_method!(misogyny, u64);
    opt_method!(overall_level, u64);
    opt_method!(race_ethnicity_or_religion, u64);
    opt_method!(sex_based_terms, u64);
    opt_method!(sexuality_sex_or_gender, u64);
    opt_method!(swearing, u64);

    pub fn build(self) -> TwitchAPIRequest<AutoModSettingsResponse> {
        let mut url = self.api.build_url();

        url.path_segments_mut()
            .unwrap()
            .extend(&[MODERATION, AUTOMOD, SETTINGS]);

        let mut query = url.query_pairs_mut();

        query.append_pair(BROADCASTER_ID, self.broadcaster_id);
        query.append_pair(MODERATOR_ID, self.moderator_id);

        let body = serde_json::to_string(&self).ok();

        drop(query);

        TwitchAPIRequest::new(
            crate::request::EndpointType::UpdateAutoModSettings,
            url,
            reqwest::Method::PUT,
            self.api.header_json(),
            body,
            self.api.client.clone(),
        )
    }

    pub async fn send(self) -> Result<reqwest::Response, crate::Error> {
        self.build().send().await
    }

    pub async fn json(self) -> Result<AutoModSettingsResponse, crate::Error> {
        self.build().json().await
    }
}

define_request_builder! {
    #[derive(Debug)]
    GetBannedUsersBuilder<'a> {
        req: {broadcaster_id:&'a BroadcasterId [key = BROADCASTER_ID]},
        opts: {
            user_ids: &'a [UserId] [key = USER_ID, convert = extend],
            first: u8 [key = FIRST, convert = to_string],
            after: &'a str [key = AFTER],
            before: &'a str[key = BEFORE],
        }
    } -> GetBannedUsersResponse;
    endpoint_type: GetBannedUsers,
    method: GET,
    path: [MODERATION, BANNED],
}

#[derive(Debug, Serialize)]
pub struct BanUserBuilder<'a> {
    #[serde(skip)]
    api: &'a TwitchAPI,
    #[serde(skip)]
    broadcaster_id: &'a BroadcasterId,
    #[serde(skip)]
    moderator_id: &'a ModeratorId,

    user_id: &'a UserId,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<&'a str>,
}

impl<'a> BanUserBuilder<'a> {
    pub fn new(
        api: &'a TwitchAPI,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
        user_id: &'a UserId,
    ) -> Self {
        Self {
            api,
            broadcaster_id,
            moderator_id,
            user_id,
            duration: None,
            reason: None,
        }
    }

    opt_method!(duration, u64);
    opt_method!(reason, &'a str);

    pub fn build(self) -> TwitchAPIRequest<BanUsersResponse> {
        let mut url = self.api.build_url();
        url.path_segments_mut().unwrap().extend(&[MODERATION, BANS]);

        let mut query = url.query_pairs_mut();

        query.append_pair(BROADCASTER_ID, self.broadcaster_id);
        query.append_pair(MODERATOR_ID, self.moderator_id);

        drop(query);

        let body = serde_json::to_string(&self).ok();
        TwitchAPIRequest::new(
            crate::request::EndpointType::BanUsers,
            url,
            reqwest::Method::POST,
            self.api.header_json(),
            body,
            self.api.client.clone(),
        )
    }

    pub async fn send(self) -> Result<reqwest::Response, crate::Error> {
        self.build().send().await
    }

    pub async fn json(self) -> Result<BanUsersResponse, crate::Error> {
        self.build().json().await
    }
}

define_request_builder! {
    #[derive(Debug)]
    GetUnbanRequestsBuilder<'a> {
        req: {
            broadcaster_id: &'a BroadcasterId [key = BROADCASTER_ID],
            moderator_id: &'a ModeratorId [key = MODERATOR_ID],
            status: UnbanRequestStatus [convert = as_ref],
        },
        opts: {
            user_id: &'a UserId [key = USER_ID],
            first: u8 [key = FIRST, convert = to_string],
            after: &'a str [key = AFTER],
        }
    } -> UnbanRequestResponse;
    endpoint_type: GetUnbanRequests,
    method: GET,
    path: [MODERATION, UNBAN_REQUESTS],
}

define_request_builder! {
    #[derive(Debug)]
    ResolveUnbanRequestBuilder<'a> {
        req: {
            broadcaster_id: &'a BroadcasterId [key = BROADCASTER_ID],
            moderator_id: &'a ModeratorId [key = MODERATOR_ID],
            unban_request_id: &'a str [key = "unban_request_id"],
            status: UnbanRequestStatus [key = "status", convert = as_ref],
        },
        opts: {
            resolution_text: &'a str,
        }
    } -> UnbanRequestResponse;
    endpoint_type: ResolveUnbanRequests,
    method: PATCH,
    path: [MODERATION, "unban_requests"],
}

define_request_builder! {
    #[derive(Debug)]
    GetBlockedTermsBuilder<'a> {
        req: {
            broadcaster_id: &'a BroadcasterId,
            moderator_id: &'a ModeratorId,
        },
        opts: {
            first: u8 [key = FIRST, convert = to_string],
            after: &'a str [key = AFTER],
        }
    } -> BlockedTermsResponse;
    endpoint_type: GetBlockedTerms,
    method: GET,
    path: [MODERATION, BLOCKED_TERMS],
}

define_request_builder! {
    #[derive(Debug)]
    DeleteChatMessagesBuilder<'a> {
        req: {
            broadcaster_id: &'a BroadcasterId,
            moderator_id: &'a ModeratorId,
        },
        opts: {message_id: &'a str}
    } -> NoContent;
    endpoint_type: DeleteChatMessages,
    method: DELETE,
    path: [MODERATION, CHAT],
}

define_request_builder! {
    #[derive(Debug)]
    GetModeratedChannelsBuilder<'a> {
        req: {user_id: &'a UserId},
        opts: {
            first: u8 [key = FIRST, convert = to_string],
            after: &'a str [key = AFTER],
    }
    } -> ModeratedChannelResponse;
        endpoint_type: GetModeratedChannels,
        method: GET,
        path: [MODERATION, CHANNELS],
}

define_request_builder! {
    #[derive(Debug)]
    GetModeratorsBuilder<'a> {
        req: {broadcaster_id: &'a BroadcasterId},
        opts: {
            user_ids: &'a [UserId] [convert = extend],
            first: u8 [key = FIRST, convert = to_string],
            after: &'a str [key = AFTER],
        }
    } -> ModeratorsResponse;
    endpoint_type: GetModerators,
    method: GET,
    path: [MODERATION, MODERATORS],
}

define_request_builder! {
    #[derive(Debug)]
    GetVipsBuilder<'a> {
        req: {broadcaster_id: &'a BroadcasterId},
        opts: {
            user_ids: &'a [UserId] [convert = extend],
            first: u8 [key = FIRST, convert = to_string],
            after: &'a str [key = AFTER],
        }
    } -> ModeratorsResponse;
            endpoint_type: GetVIPs,
            method: GET,
            path: [CHANNELS, VIPS],
}
