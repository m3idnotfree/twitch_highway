use serde::Serialize;

use crate::{
    moderation::{
        AutoModSettingsResponse, BanUsersResponse, BlockedTermsResponse, GetBannedUsersResponse,
        ModeratedChannelResponse, ModeratorsResponse, UnbanRequestResponse, UnbanRequestStatus,
    },
    types::{
        constants::{
            AFTER, AUTOMOD, BANNED, BANS, BEFORE, BLOCKED_TERMS, BROADCASTER_ID, CHANNELS, CHAT,
            FIRST, MODERATION, MODERATORS, MODERATOR_ID, SETTINGS, STATUS, UNBAN_REQUESTS, USER_ID,
            VIPS,
        },
        BroadcasterId, ModeratorId, UserId,
    },
    Client, Error,
};

#[derive(Debug, Serialize)]
pub struct UpdateAutomodSettingsBuilder<'a> {
    #[serde(skip)]
    client: &'a Client,
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
        client: &'a Client,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
    ) -> Self {
        Self {
            client,
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

    pub fn aggression(mut self, value: u64) -> Self {
        self.aggression = Some(value);
        self
    }

    pub fn bullying(mut self, value: u64) -> Self {
        self.bullying = Some(value);
        self
    }

    pub fn disability(mut self, value: u64) -> Self {
        self.disability = Some(value);
        self
    }

    pub fn misogyny(mut self, value: u64) -> Self {
        self.misogyny = Some(value);
        self
    }

    pub fn overall_level(mut self, value: u64) -> Self {
        self.overall_level = Some(value);
        self
    }

    pub fn race_ethnicity_or_religion(mut self, value: u64) -> Self {
        self.race_ethnicity_or_religion = Some(value);
        self
    }

    pub fn sex_based_terms(mut self, value: u64) -> Self {
        self.sex_based_terms = Some(value);
        self
    }

    pub fn sexuality_sex_or_gender(mut self, value: u64) -> Self {
        self.sexuality_sex_or_gender = Some(value);
        self
    }

    pub fn swearing(mut self, value: u64) -> Self {
        self.swearing = Some(value);
        self
    }

    pub async fn send(self) -> Result<AutoModSettingsResponse, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([MODERATION, AUTOMOD, SETTINGS]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, self.broadcaster_id)
            .append_pair(MODERATOR_ID, self.moderator_id);

        let req = self.client.http_client().put(url).json(&self);
        self.client.json(req).await
    }
}

#[derive(Debug)]

pub struct GetBannedUsersBuilder<'a> {
    client: &'a Client,
    broadcaster_id: &'a BroadcasterId,
    user_ids: Option<&'a [UserId]>,
    first: Option<u8>,
    after: Option<&'a str>,
    before: Option<&'a str>,
}

impl<'a> GetBannedUsersBuilder<'a> {
    pub fn new(client: &'a Client, broadcaster_id: &'a BroadcasterId) -> Self {
        Self {
            client,
            broadcaster_id,
            user_ids: None,
            first: None,
            after: None,
            before: None,
        }
    }

    pub fn user_ids(mut self, value: &'a [UserId]) -> Self {
        self.user_ids = Some(value);
        self
    }

    pub fn first(mut self, value: u8) -> Self {
        self.first = Some(value);
        self
    }

    pub fn after(mut self, value: &'a str) -> Self {
        self.after = Some(value);
        self
    }

    pub fn before(mut self, value: &'a str) -> Self {
        self.before = Some(value);
        self
    }

    pub async fn send(self) -> Result<GetBannedUsersResponse, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([MODERATION, BANNED]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, self.broadcaster_id);
        if let Some(ids) = self.user_ids {
            url.query_pairs_mut()
                .extend_pairs(ids.iter().map(|id| (USER_ID, id)));
        }
        if let Some(val) = self.first {
            url.query_pairs_mut().append_pair(FIRST, &val.to_string());
        }
        if let Some(val) = self.after {
            url.query_pairs_mut().append_pair(AFTER, val);
        }
        if let Some(val) = self.before {
            url.query_pairs_mut().append_pair(BEFORE, val);
        }

        let req = self.client.http_client().get(url);
        self.client.json(req).await
    }
}

#[derive(Debug, Serialize)]
pub struct BanUserBuilder<'a> {
    #[serde(skip)]
    client: &'a Client,
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
        client: &'a Client,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
        user_id: &'a UserId,
    ) -> Self {
        Self {
            client,
            broadcaster_id,
            moderator_id,
            user_id,
            duration: None,
            reason: None,
        }
    }

    pub fn duration(mut self, value: u64) -> Self {
        self.duration = Some(value);
        self
    }

    pub fn reason(mut self, value: &'a str) -> Self {
        self.reason = Some(value);
        self
    }

    pub async fn send(self) -> Result<BanUsersResponse, Error> {
        let Self {
            client,
            broadcaster_id,
            moderator_id,
            user_id,
            duration,
            reason,
        } = self;

        let mut url = client.base_url();
        url.path_segments_mut().unwrap().extend([MODERATION, BANS]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id)
            .append_pair(MODERATOR_ID, moderator_id);

        let req = self
            .client
            .http_client()
            .post(url)
            .json(&BanUserRequestBody {
                data: BanUserData {
                    user_id,
                    duration,
                    reason,
                },
            });
        client.json(req).await
    }
}

#[derive(Serialize)]
struct BanUserRequestBody<'a> {
    data: BanUserData<'a>,
}

#[derive(Serialize)]
struct BanUserData<'a> {
    user_id: &'a UserId,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<&'a str>,
}

#[derive(Debug)]
pub struct GetUnbanRequestsBuilder<'a> {
    client: &'a Client,
    broadcaster_id: &'a BroadcasterId,
    moderator_id: &'a ModeratorId,
    status: UnbanRequestStatus,
    user_id: Option<&'a UserId>,
    first: Option<u8>,
    after: Option<&'a str>,
}

impl<'a> GetUnbanRequestsBuilder<'a> {
    pub fn new(
        client: &'a Client,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
        status: UnbanRequestStatus,
    ) -> Self {
        Self {
            client,
            broadcaster_id,
            moderator_id,
            status,
            user_id: None,
            first: None,
            after: None,
        }
    }

    pub fn user_id(mut self, value: &'a UserId) -> Self {
        self.user_id = Some(value);
        self
    }

    pub fn first(mut self, value: u8) -> Self {
        self.first = Some(value);
        self
    }

    pub fn after(mut self, value: &'a str) -> Self {
        self.after = Some(value);
        self
    }

    pub async fn send(self) -> Result<UnbanRequestResponse, Error> {
        let mut url = self.client.base_url();
        url.path_segments_mut()
            .unwrap()
            .extend([MODERATION, UNBAN_REQUESTS]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, self.broadcaster_id)
            .append_pair(MODERATOR_ID, self.moderator_id)
            .append_pair(STATUS, self.status.as_ref());
        if let Some(val) = self.user_id {
            url.query_pairs_mut().append_pair(USER_ID, val);
        }
        if let Some(val) = self.first {
            url.query_pairs_mut().append_pair(FIRST, &val.to_string());
        }
        if let Some(val) = self.after {
            url.query_pairs_mut().append_pair(AFTER, val);
        }

        let req = self.client.http_client().get(url);
        self.client.json(req).await
    }
}

#[derive(Debug)]
pub struct ResolveUnbanRequestBuilder<'a> {
    client: &'a Client,
    broadcaster_id: &'a BroadcasterId,
    moderator_id: &'a ModeratorId,
    unban_request_id: &'a str,
    status: UnbanRequestStatus,
    resolution_text: Option<&'a str>,
}

impl<'a> ResolveUnbanRequestBuilder<'a> {
    pub fn new(
        client: &'a Client,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
        unban_request_id: &'a str,
        status: UnbanRequestStatus,
    ) -> Self {
        Self {
            client,
            broadcaster_id,
            moderator_id,
            unban_request_id,
            status,
            resolution_text: None,
        }
    }

    pub fn resolution_text(mut self, value: &'a str) -> Self {
        self.resolution_text = Some(value);
        self
    }

    pub async fn send(self) -> Result<UnbanRequestResponse, Error> {
        let mut url = self.client.base_url();
        url.path_segments_mut()
            .unwrap()
            .extend([MODERATION, UNBAN_REQUESTS]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, self.broadcaster_id)
            .append_pair(MODERATOR_ID, self.moderator_id)
            .append_pair("unban_request_id", self.unban_request_id)
            .append_pair(STATUS, self.status.as_ref());
        if let Some(val) = self.resolution_text {
            url.query_pairs_mut().append_pair("resolution_text", val);
        }

        let req = self.client.http_client().patch(url);
        self.client.json(req).await
    }
}

#[derive(Debug)]
pub struct GetBlockedTermsBuilder<'a> {
    client: &'a Client,
    broadcaster_id: &'a BroadcasterId,
    moderator_id: &'a ModeratorId,
    first: Option<u8>,
    after: Option<&'a str>,
}

impl<'a> GetBlockedTermsBuilder<'a> {
    pub fn new(
        client: &'a Client,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
    ) -> Self {
        Self {
            client,
            broadcaster_id,
            moderator_id,
            first: None,
            after: None,
        }
    }

    pub fn first(mut self, value: u8) -> Self {
        self.first = Some(value);
        self
    }

    pub fn after(mut self, value: &'a str) -> Self {
        self.after = Some(value);
        self
    }

    pub async fn send(self) -> Result<BlockedTermsResponse, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([MODERATION, BLOCKED_TERMS]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, self.broadcaster_id)
            .append_pair(MODERATOR_ID, self.moderator_id);
        if let Some(val) = self.first {
            url.query_pairs_mut().append_pair(FIRST, &val.to_string());
        }
        if let Some(val) = self.after {
            url.query_pairs_mut().append_pair(AFTER, val);
        }

        let req = self.client.http_client().get(url);
        self.client.json(req).await
    }
}

#[derive(Debug)]
pub struct DeleteChatMessagesBuilder<'a> {
    client: &'a Client,
    broadcaster_id: &'a BroadcasterId,
    moderator_id: &'a ModeratorId,
    message_id: Option<&'a str>,
}

impl<'a> DeleteChatMessagesBuilder<'a> {
    pub fn new(
        client: &'a Client,
        broadcaster_id: &'a BroadcasterId,
        moderator_id: &'a ModeratorId,
    ) -> Self {
        Self {
            client,
            broadcaster_id,
            moderator_id,
            message_id: None,
        }
    }

    pub fn message_id(mut self, value: &'a str) -> Self {
        self.message_id = Some(value);
        self
    }

    pub async fn send(self) -> Result<(), Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut().unwrap().extend([MODERATION, CHAT]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, self.broadcaster_id)
            .append_pair(MODERATOR_ID, self.moderator_id);
        if let Some(val) = self.message_id {
            url.query_pairs_mut().append_pair("message_id", val);
        }

        let req = self.client.http_client().delete(url);
        self.client.no_content(req).await
    }
}

#[derive(Debug)]
pub struct GetModeratedChannelsBuilder<'a> {
    client: &'a Client,
    user_id: &'a UserId,
    first: Option<u8>,
    after: Option<&'a str>,
}

impl<'a> GetModeratedChannelsBuilder<'a> {
    pub fn new(client: &'a Client, user_id: &'a UserId) -> Self {
        Self {
            client,
            user_id,
            first: None,
            after: None,
        }
    }

    pub fn first(mut self, value: u8) -> Self {
        self.first = Some(value);
        self
    }

    pub fn after(mut self, value: &'a str) -> Self {
        self.after = Some(value);
        self
    }

    pub async fn send(self) -> Result<ModeratedChannelResponse, Error> {
        let mut url = self.client.base_url();
        url.path_segments_mut()
            .unwrap()
            .extend([MODERATION, CHANNELS]);

        url.query_pairs_mut().append_pair(USER_ID, self.user_id);
        if let Some(val) = self.first {
            url.query_pairs_mut().append_pair(FIRST, &val.to_string());
        }
        if let Some(val) = self.after {
            url.query_pairs_mut().append_pair(AFTER, val);
        }

        let req = self.client.http_client().get(url);
        self.client.json(req).await
    }
}

#[derive(Debug)]
pub struct GetModeratorsBuilder<'a> {
    client: &'a Client,
    broadcaster_id: &'a BroadcasterId,
    user_ids: Option<&'a [UserId]>,
    first: Option<u8>,
    after: Option<&'a str>,
}

impl<'a> GetModeratorsBuilder<'a> {
    pub fn new(client: &'a Client, broadcaster_id: &'a BroadcasterId) -> Self {
        Self {
            client,
            broadcaster_id,
            user_ids: None,
            first: None,
            after: None,
        }
    }

    pub fn user_ids(mut self, value: &'a [UserId]) -> Self {
        self.user_ids = Some(value);
        self
    }

    pub fn first(mut self, value: u8) -> Self {
        self.first = Some(value);
        self
    }

    pub fn after(mut self, value: &'a str) -> Self {
        self.after = Some(value);
        self
    }

    pub async fn send(self) -> Result<ModeratorsResponse, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([MODERATION, MODERATORS]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, self.broadcaster_id);
        if let Some(ids) = self.user_ids {
            url.query_pairs_mut()
                .extend_pairs(ids.iter().map(|id| (USER_ID, id)));
        }
        if let Some(val) = self.first {
            url.query_pairs_mut().append_pair(FIRST, &val.to_string());
        }
        if let Some(val) = self.after {
            url.query_pairs_mut().append_pair(AFTER, val);
        }

        let req = self.client.http_client().get(url);
        self.client.json(req).await
    }
}

#[derive(Debug)]
pub struct GetVipsBuilder<'a> {
    client: &'a Client,
    broadcaster_id: &'a BroadcasterId,
    user_ids: Option<&'a [UserId]>,
    first: Option<u8>,
    after: Option<&'a str>,
}

impl<'a> GetVipsBuilder<'a> {
    pub fn new(client: &'a Client, broadcaster_id: &'a BroadcasterId) -> Self {
        Self {
            client,
            broadcaster_id,
            user_ids: None,
            first: None,
            after: None,
        }
    }

    pub fn user_ids(mut self, value: &'a [UserId]) -> Self {
        self.user_ids = Some(value);
        self
    }

    pub fn first(mut self, value: u8) -> Self {
        self.first = Some(value);
        self
    }

    pub fn after(mut self, value: &'a str) -> Self {
        self.after = Some(value);
        self
    }

    pub async fn send(self) -> Result<ModeratorsResponse, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut().unwrap().extend([CHANNELS, VIPS]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, self.broadcaster_id);
        if let Some(ids) = self.user_ids {
            url.query_pairs_mut()
                .extend_pairs(ids.iter().map(|id| (USER_ID, id)));
        }
        if let Some(val) = self.first {
            url.query_pairs_mut().append_pair(FIRST, &val.to_string());
        }
        if let Some(val) = self.after {
            url.query_pairs_mut().append_pair(AFTER, val);
        }

        let req = self.client.http_client().get(url);
        self.client.json(req).await
    }
}
