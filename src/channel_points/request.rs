use serde::Serialize;

use crate::{
    base::{IntoQueryPairs, QueryParams},
    types::{BroadcasterId, Id, AFTER, BROADCASTER_ID, FIRST, ID},
    RequestBody,
};

use super::types::RedemptionStatus;

request_struct!(
    #[derive(Debug,  Serialize)]
    UpdateRedemptionStatusRequest {
        required {
            status: RedemptionStatus
        }
    }
);

impl RequestBody for UpdateRedemptionStatusRequest {
    fn as_body(&self) -> Option<String> {
        Some(serde_json::to_string(self).unwrap())
    }
}

request_struct!(
    #[derive(Debug,Serialize)]
    CreateCustomRewardsRequest{
        required {
            title: String,
            cost: u64
        },
        optional {
            #[serde(skip_serializing_if = "Option::is_none")]
            prompt:String,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_enabled:bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            background_color:String,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_user_input_required:bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_max_per_stream_enabled:bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            max_per_stream:u64,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_max_per_user_per_stream_enabled:bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            max_per_user_per_stream:u64,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_global_cooldown_enabled:bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            global_cooldown_seconds:u64,
            #[serde(skip_serializing_if = "Option::is_none")]
            should_redemptions_skip_request_queue:bool

        }
    }

);

impl RequestBody for CreateCustomRewardsRequest {
    fn as_body(&self) -> Option<String> {
        Some(serde_json::to_string(self).unwrap())
    }
}

request_struct!(
    #[derive(Debug, Serialize)]
    CustomRewardRedemptionRequest {
        required {
            broadcaster_id: BroadcasterId,
            reward_id: String,
        },
        optional {
            status: RedemptionStatus,
            id: Id,
            sort: String,
            after: String,
            first: u64
        }
    }
);

impl IntoQueryPairs for CustomRewardRedemptionRequest {
    fn into_query_pairs(self) -> Vec<(&'static str, String)> {
        let mut params = QueryParams::new();

        params
            .push(BROADCASTER_ID, self.broadcaster_id)
            .push("reward_id", self.reward_id)
            .push_opt("status", self.status.map(|x| x.to_string()))
            .push_opt(ID, self.id)
            .push_opt("sort", self.sort)
            .push_opt(AFTER, self.after)
            .push_opt(FIRST, self.first.map(|x| x.to_string()));

        params.build()
    }
}

request_struct!(
    #[derive(Debug, Default, Serialize)]
    UpdateCustomRewardRequest {
        #[serde(skip_serializing_if = "Option::is_none")]
        title: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        prompt: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        cost: u64,
        #[serde(skip_serializing_if = "Option::is_none")]
        background_color: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        is_enabled: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        is_user_input_required: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        is_max_per_stream_enabled: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        max_per_stream: u64,
        #[serde(skip_serializing_if = "Option::is_none")]
        is_max_per_user_per_stream_enabled: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        max_per_user_per_stream: u64,
        #[serde(skip_serializing_if = "Option::is_none")]
        is_global_cooldown_enabled: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        global_cooldown_seconds: u64,
        #[serde(skip_serializing_if = "Option::is_none")]
        is_paused: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        should_redemptions_skip_request_queue: bool
    }
);

impl RequestBody for UpdateCustomRewardRequest {
    fn as_body(&self) -> Option<String> {
        Some(serde_json::to_string(self).unwrap())
    }
}
