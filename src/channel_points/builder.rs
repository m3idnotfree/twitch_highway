use serde::Serialize;

use crate::{
    channel_points::{
        CustomRewardsRedemptionResponse, CustomRewardsResponse, RedemptionStatus, Sort,
    },
    request::TwitchAPIRequest,
    types::constants::{
        AFTER, BROADCASTER_ID, CHANNEL_POINTS, CUSTOM_REWARDS, FIRST, ID, REDEMPTIONS, REWARD_ID,
        SORT, STATUS,
    },
    types::{BroadcasterId, RedemptionId, RewardId},
    TwitchAPI,
};

const ONLY_MANAGEABLE_REWARDS: &str = "only_manageable_rewards";

#[derive(Debug, Serialize)]
pub struct CreateCustomRewardBuilder<'a> {
    #[serde(skip)]
    api: &'a TwitchAPI,
    #[serde(skip)]
    broadcaster_id: &'a BroadcasterId,

    title: &'a str,
    cost: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    prompt: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    background_color: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_user_input_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_max_per_stream_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_per_stream: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_max_per_user_per_stream_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_per_user_per_stream: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_global_cooldown_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    global_cooldown_seconds: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    should_redemptions_skip_request_queue: Option<bool>,
}

impl<'a> CreateCustomRewardBuilder<'a> {
    pub fn new(
        api: &'a TwitchAPI,
        broadcaster_id: &'a BroadcasterId,
        title: &'a str,
        cost: u64,
    ) -> Self {
        Self {
            api,
            broadcaster_id,
            title,
            cost,
            prompt: None,
            background_color: None,
            is_enabled: None,
            is_user_input_required: None,
            is_max_per_stream_enabled: None,
            max_per_stream: None,
            is_max_per_user_per_stream_enabled: None,
            max_per_user_per_stream: None,
            is_global_cooldown_enabled: None,
            global_cooldown_seconds: None,
            should_redemptions_skip_request_queue: None,
        }
    }
    pub fn prompt(mut self, value: &'a str) -> Self {
        self.prompt = Some(value);
        self
    }
    pub fn background_color(mut self, value: &'a str) -> Self {
        self.background_color = Some(value);
        self
    }
    pub fn is_enabled(mut self, value: bool) -> Self {
        self.is_enabled = Some(value);
        self
    }
    pub fn is_user_input_required(mut self, value: bool) -> Self {
        self.is_user_input_required = Some(value);
        self
    }
    pub fn is_max_per_stream_enabled(mut self, value: bool) -> Self {
        self.is_max_per_stream_enabled = Some(value);
        self
    }
    pub fn max_per_stream(mut self, value: u64) -> Self {
        self.max_per_stream = Some(value);
        self
    }
    pub fn is_max_per_user_per_stream_enabled(mut self, value: bool) -> Self {
        self.is_max_per_user_per_stream_enabled = Some(value);
        self
    }
    pub fn max_per_user_per_stream(mut self, value: u64) -> Self {
        self.max_per_user_per_stream = Some(value);
        self
    }
    pub fn is_global_cooldown_enabled(mut self, value: bool) -> Self {
        self.is_global_cooldown_enabled = Some(value);
        self
    }
    pub fn global_cooldown_seconds(mut self, value: u64) -> Self {
        self.global_cooldown_seconds = Some(value);
        self
    }
    pub fn should_redemptions_skip_request_queue(mut self, value: bool) -> Self {
        self.should_redemptions_skip_request_queue = Some(value);
        self
    }

    pub fn build(self) -> TwitchAPIRequest<CustomRewardsResponse> {
        let mut url = self.api.build_url();

        url.path_segments_mut()
            .unwrap()
            .extend(&[CHANNEL_POINTS, CUSTOM_REWARDS]);

        let mut query = url.query_pairs_mut();

        query.append_pair(BROADCASTER_ID, self.broadcaster_id);

        drop(query);

        let body = serde_json::to_string(&self).ok();

        TwitchAPIRequest::new(
            crate::request::EndpointType::CreateCustomRewards,
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

    pub async fn json(self) -> Result<CustomRewardsResponse, crate::Error> {
        self.build().json().await
    }
}

define_request_builder! {
    #[derive(Debug)]
    GetCustomRewardBuilder<'a> {
        req: {broadcaster_id: &'a BroadcasterId [key = BROADCASTER_ID]},
        opts: {
            custom_reward_ids: &'a [RewardId] [key = ID, convert = extend],
            only_manageable_rewards: bool [key = ONLY_MANAGEABLE_REWARDS, convert = to_string],
        }
    } -> CustomRewardsResponse;
    endpoint: GetCustomReward,
    method: GET,
    path: [CHANNEL_POINTS, CUSTOM_REWARDS],
}

define_request_builder! {
    #[derive(Debug)]
    GetCustomRewardRedemptionBuilder<'a> {
        req: {
            broadcaster_id: &'a BroadcasterId [key = BROADCASTER_ID],
            reward_id: &'a RewardId [key = REWARD_ID]
        },
        opts: {
            status: RedemptionStatus [key = STATUS, convert = as_ref],
            sort: Sort [key = SORT, convert = as_ref],
            ids: &'a [RedemptionId] [key = ID, convert = extend],
            first: u8 [key = FIRST, convert = to_string],
            after: &'a str [key = AFTER]
        }
    } -> CustomRewardsRedemptionResponse;
    endpoint: GetCustomRewardRedemption,
    method: GET,
    path: [CHANNEL_POINTS, CUSTOM_REWARDS, REDEMPTIONS],
}

#[derive(Debug, Serialize)]
pub struct UpdateCustomRewardBuilder<'a> {
    #[serde(skip)]
    api: &'a TwitchAPI,
    #[serde(skip)]
    broadcaster_id: &'a BroadcasterId,
    #[serde(skip)]
    reward_id: &'a RewardId,

    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cost: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prompt: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    background_color: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_user_input_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_max_per_stream_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_per_stream: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_max_per_user_per_stream_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_per_user_per_stream: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_global_cooldown_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    global_cooldown_seconds: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_paused: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    should_redemptions_skip_request_queue: Option<bool>,
}

impl<'a> UpdateCustomRewardBuilder<'a> {
    pub fn new(
        api: &'a TwitchAPI,
        broadcaster_id: &'a BroadcasterId,
        reward_id: &'a RewardId,
    ) -> Self {
        Self {
            api,
            broadcaster_id,
            reward_id,
            title: None,
            cost: None,
            prompt: None,
            background_color: None,
            is_enabled: None,
            is_user_input_required: None,
            is_max_per_stream_enabled: None,
            max_per_stream: None,
            is_max_per_user_per_stream_enabled: None,
            max_per_user_per_stream: None,
            is_global_cooldown_enabled: None,
            global_cooldown_seconds: None,
            is_paused: None,
            should_redemptions_skip_request_queue: None,
        }
    }
    pub fn title(mut self, value: &'a str) -> Self {
        self.title = Some(value);
        self
    }
    pub fn cost(mut self, value: u64) -> Self {
        self.cost = Some(value);
        self
    }
    pub fn prompt(mut self, value: &'a str) -> Self {
        self.prompt = Some(value);
        self
    }
    pub fn background_color(mut self, value: &'a str) -> Self {
        self.background_color = Some(value);
        self
    }
    pub fn is_enabled(mut self, value: bool) -> Self {
        self.is_enabled = Some(value);
        self
    }
    pub fn is_user_input_required(mut self, value: bool) -> Self {
        self.is_user_input_required = Some(value);
        self
    }
    pub fn is_max_per_stream_enabled(mut self, value: bool) -> Self {
        self.is_max_per_stream_enabled = Some(value);
        self
    }
    pub fn max_per_stream(mut self, value: u64) -> Self {
        self.max_per_stream = Some(value);
        self
    }
    pub fn is_max_per_user_per_stream_enabled(mut self, value: bool) -> Self {
        self.is_max_per_user_per_stream_enabled = Some(value);
        self
    }
    pub fn max_per_user_per_stream(mut self, value: u64) -> Self {
        self.max_per_user_per_stream = Some(value);
        self
    }
    pub fn is_global_cooldown_enabled(mut self, value: bool) -> Self {
        self.is_global_cooldown_enabled = Some(value);
        self
    }
    pub fn global_cooldown_seconds(mut self, value: u64) -> Self {
        self.global_cooldown_seconds = Some(value);
        self
    }
    pub fn is_paused(mut self, value: bool) -> Self {
        self.is_paused = Some(value);
        self
    }
    pub fn should_redemptions_skip_request_queue(mut self, value: bool) -> Self {
        self.should_redemptions_skip_request_queue = Some(value);
        self
    }

    pub fn build(self) -> TwitchAPIRequest<CustomRewardsResponse> {
        let mut url = self.api.build_url();

        url.path_segments_mut()
            .unwrap()
            .extend(&[CHANNEL_POINTS, CUSTOM_REWARDS]);

        let mut query = url.query_pairs_mut();

        query.append_pair(BROADCASTER_ID, self.broadcaster_id);
        query.append_pair(ID, self.reward_id);

        drop(query);

        let body = serde_json::to_string(&self).ok();

        TwitchAPIRequest::new(
            crate::request::EndpointType::UpdateCustomReward,
            url,
            reqwest::Method::PATCH,
            self.api.header_json(),
            body,
            self.api.client.clone(),
        )
    }

    pub async fn send(self) -> Result<reqwest::Response, crate::Error> {
        self.build().send().await
    }

    pub async fn json(self) -> Result<CustomRewardsResponse, crate::Error> {
        self.build().json().await
    }
}
