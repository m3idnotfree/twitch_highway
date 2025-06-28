use serde::Serialize;

use crate::types::RedemptionId;

use super::types::RedemptionStatus;

define_request!(
    #[derive(Serialize)]
    CustomRewardsRequiredBody<'a> {
        req: {
            title: &'a str,
            cost: u64
        };
    }
);

define_request!(
    #[derive(Serialize)]
    CustomRewardsBody<'a> {
        opts: {
            #[serde(skip_serializing_if = "Option::is_none")]
            title: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            prompt: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            background_color: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            cost: u64,
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
            should_redemptions_skip_request_queue: bool
        };
    }
);

define_request!(
    #[derive(Serialize)]
    RedemptionStatusQuery {
        req: {
            status: RedemptionStatus
        };
        to_json
    }
);

define_request!(
    #[derive(Serialize)]
    CustomRewardRedemptionQuery<'a> {
        opts: {
            sort: &'a str,
            status: RedemptionStatus,
            id: RedemptionId,
        };
        apply_to_url
    }
);

define_request!(
    #[derive(Serialize, Default)]
    UpdateCustomRewardRequest<'a> {
        opts: {
            #[serde(skip_serializing_if = "Option::is_none")]
            title: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            prompt: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            background_color: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            cost: u64,
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
        };
        to_json
    }
);
