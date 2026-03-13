use serde::Serialize;

use crate::{
    channel_points::{
        CustomRewardsRedemptionResponse, CustomRewardsResponse, RedemptionStatus, Sort,
    },
    types::constants::{
        AFTER, BROADCASTER_ID, CHANNEL_POINTS, CUSTOM_REWARDS, FIRST, ID, REDEMPTIONS, REWARD_ID,
        SORT, STATUS,
    },
    types::{BroadcasterId, RedemptionId, RewardId},
    Client, Error,
};

const ONLY_MANAGEABLE_REWARDS: &str = "only_manageable_rewards";

#[derive(Debug, Serialize)]
pub struct CreateCustomReward<'a> {
    #[serde(skip)]
    client: &'a Client,
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

impl<'a> CreateCustomReward<'a> {
    pub fn new(
        client: &'a Client,
        broadcaster_id: &'a BroadcasterId,
        title: &'a str,
        cost: u64,
    ) -> Self {
        Self {
            client,
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

    pub async fn send(self) -> Result<CustomRewardsResponse, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([CHANNEL_POINTS, CUSTOM_REWARDS]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, self.broadcaster_id);

        let req = self.client.http_client().post(url).json(&self);
        self.client.json(req).await
    }
}

#[derive(Debug)]
pub struct GetCustomReward<'a> {
    client: &'a Client,
    broadcaster_id: &'a BroadcasterId,
    custom_reward_ids: Option<&'a [RewardId]>,
    only_manageable_rewards: Option<bool>,
}

impl<'a> GetCustomReward<'a> {
    pub fn new(client: &'a Client, broadcaster_id: &'a BroadcasterId) -> Self {
        Self {
            client,
            broadcaster_id,
            custom_reward_ids: None,
            only_manageable_rewards: None,
        }
    }

    pub fn custom_reward_ids(mut self, value: &'a [RewardId]) -> Self {
        self.custom_reward_ids = Some(value);
        self
    }

    pub fn only_manageable_rewards(mut self, value: bool) -> Self {
        self.only_manageable_rewards = Some(value);
        self
    }

    pub async fn send(self) -> Result<CustomRewardsResponse, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([CHANNEL_POINTS, CUSTOM_REWARDS]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, self.broadcaster_id);
        if let Some(val) = self.custom_reward_ids {
            for item in val.iter() {
                url.query_pairs_mut().append_pair(ID, item);
            }
        }
        if let Some(val) = self.only_manageable_rewards {
            url.query_pairs_mut()
                .append_pair(ONLY_MANAGEABLE_REWARDS, &val.to_string());
        }

        let req = self.client.http_client().get(url);
        self.client.json(req).await
    }
}

#[derive(Debug)]
pub struct GetCustomRewardRedemption<'a> {
    client: &'a Client,
    broadcaster_id: &'a BroadcasterId,
    reward_id: &'a RewardId,
    status: Option<RedemptionStatus>,
    sort: Option<Sort>,
    ids: Option<&'a [RedemptionId]>,
    first: Option<u8>,
    after: Option<&'a str>,
}

impl<'a> GetCustomRewardRedemption<'a> {
    pub fn new(
        client: &'a Client,
        broadcaster_id: &'a BroadcasterId,
        reward_id: &'a RewardId,
    ) -> Self {
        Self {
            client,
            broadcaster_id,
            reward_id,
            status: None,
            sort: None,
            ids: None,
            first: None,
            after: None,
        }
    }

    pub fn status(mut self, value: RedemptionStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn sort(mut self, value: Sort) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn ids(mut self, value: &'a [RedemptionId]) -> Self {
        self.ids = Some(value);
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

    pub async fn send(self) -> Result<CustomRewardsRedemptionResponse, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([CHANNEL_POINTS, CUSTOM_REWARDS, REDEMPTIONS]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, self.broadcaster_id)
            .append_pair(REWARD_ID, self.reward_id);
        if let Some(val) = self.status {
            url.query_pairs_mut().append_pair(STATUS, val.as_ref());
        }
        if let Some(val) = self.sort {
            url.query_pairs_mut().append_pair(SORT, val.as_ref());
        }
        if let Some(val) = self.ids {
            for item in val.iter() {
                url.query_pairs_mut().append_pair(ID, item);
            }
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

#[derive(Debug, Serialize)]
pub struct UpdateCustomReward<'a> {
    #[serde(skip)]
    client: &'a Client,
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

impl<'a> UpdateCustomReward<'a> {
    pub fn new(
        client: &'a Client,
        broadcaster_id: &'a BroadcasterId,
        reward_id: &'a RewardId,
    ) -> Self {
        Self {
            client,
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

    pub async fn send(self) -> Result<CustomRewardsResponse, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([CHANNEL_POINTS, CUSTOM_REWARDS]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, self.broadcaster_id)
            .append_pair(ID, self.reward_id);

        let req = self.client.http_client().patch(url).json(&self);
        self.client.json(req).await
    }
}
