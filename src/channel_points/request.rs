use serde::Serialize;

use crate::{
    base::{IntoQueryPairs, QueryParams},
    types::{constants::ID, RedemptionId},
};

use super::types::RedemptionStatus;

request_struct!(
    #[derive(Serialize)]
    CustomRewardsRequiredBody {
        required {
            title: String,
            cost: u64
        }
    };
    impl_body: true
);

request_struct!(
    #[derive(Serialize)]
    CustomRewardsBody {
        string {
            #[serde(skip_serializing_if = "Option::is_none")]
            title: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            prompt: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            background_color: String,
        },
        any {
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
        }
    };
    impl_body: true
);

request_struct!(
    #[derive(Serialize)]
    RedemptionStatusQuery {
        required {
            status: RedemptionStatus
        }
    };
    impl_body: true
);

request_struct!(
    #[derive(Serialize)]
    CustomRewardRedemptionQuery {
        string {
            sort: String,
        },
        any {
            status: RedemptionStatus,
            id: RedemptionId,
        }
    }
);

impl IntoQueryPairs for CustomRewardRedemptionQuery {
    fn into_query_pairs(self) -> Vec<(&'static str, String)> {
        let mut params = QueryParams::new();

        params
            .push_opt("status", self.status.map(|x| x.to_string()))
            .push_opt(ID, self.id)
            .push_opt("sort", self.sort);

        params.build()
    }
}

request_struct!(
    #[derive(Serialize)]
    UpdateCustomRewardRequest {
        string {
            #[serde(skip_serializing_if = "Option::is_none")]
            title: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            prompt: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            background_color: String,
        },
        any {
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
        }
    };
    impl_body: true
);
