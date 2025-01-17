use serde::Serialize;

use crate::{types::UserId, IntoRequestBody};

request_struct!(
    #[derive(Serialize)]
    CheckAutoModStatusRequest {
        required {
            data: Vec<CheckAutoMod>
        }
    }
);

impl IntoRequestBody for CheckAutoModStatusRequest {
    fn as_body(&self) -> Option<String> {
        Some(serde_json::to_string(&self).unwrap())
    }
}

request_struct!(
    #[derive(Serialize)]
     CheckAutoMod {
        required {
            msg_id: String,
            msg_text: String,
        }
    }
);

request_struct!(
    #[derive(Serialize)]
     ManageHeldAutoModMeussageRequest {
        required {
            user_id: UserId,
            msg_id: String,
            action: AutoModAction
        }
    }
);
impl IntoRequestBody for ManageHeldAutoModMeussageRequest {
    fn as_body(&self) -> Option<String> {
        Some(serde_json::to_string(&self).unwrap())
    }
}

#[derive(Debug, Serialize)]
pub enum AutoModAction {
    ALLOW,
    DENY,
}

request_struct!(
    #[derive(Serialize)]
    UpdateAutoModSettingsRequest {
        #[serde(skip_serializing_if = "Option::is_none")]
        aggression: u64,
        #[serde(skip_serializing_if = "Option::is_none")]
        bullying: u64,
        #[serde(skip_serializing_if = "Option::is_none")]
        disability: u64,
        #[serde(skip_serializing_if = "Option::is_none")]
        misogyny: u64,
        #[serde(skip_serializing_if = "Option::is_none")]
        overall_level: u64,
        #[serde(skip_serializing_if = "Option::is_none")]
        race_ethnicity_or_religion: u64,
        #[serde(skip_serializing_if = "Option::is_none")]
        sex_based_terms: u64,
        #[serde(skip_serializing_if = "Option::is_none")]
        sexuality_sex_or_gender: u64,
        #[serde(skip_serializing_if = "Option::is_none")]
        swearing: u64,
    }
);

impl IntoRequestBody for UpdateAutoModSettingsRequest {
    fn as_body(&self) -> Option<String> {
        Some(serde_json::to_string(&self).unwrap())
    }
}

request_struct!(
    #[derive(Serialize)]
    BanUsersRequest{
        required {
            data: BanUserRequest
        }
    }
);
impl IntoRequestBody for BanUsersRequest {
    fn as_body(&self) -> Option<String> {
        Some(serde_json::to_string(&self).unwrap())
    }
}

request_struct!(
    #[derive(Serialize)]
    BanUserRequest {
        required {
            user_id: UserId
        },
        optional {
            duration: u64,
            reason: String
       }
    }

);
request_struct!(
    #[derive(Serialize)]
    AddBlockedTermRequest {
        required {
            text: String
        }
    }
);

impl IntoRequestBody for AddBlockedTermRequest {
    fn as_body(&self) -> Option<String> {
        Some(serde_json::to_string(&self).unwrap())
    }
}

request_struct!(
    #[derive(Serialize)]
    UpdateShieldModeStatusRequest {
        required {
            is_active: bool
        }
    }
);

impl IntoRequestBody for UpdateShieldModeStatusRequest {
    fn as_body(&self) -> Option<String> {
        Some(serde_json::to_string(&self).unwrap())
    }
}

request_struct!(
    #[derive(Serialize)]
    WarnChatUserRequest {
        required {
            data: Vec<WarnChatUser>
        }
    }
);

request_struct!(
    #[derive(Serialize)]
    WarnChatUser {
        required {
            user_id: UserId,
            reason: String
        }
    }
);

impl IntoRequestBody for WarnChatUserRequest {
    fn as_body(&self) -> Option<String> {
        Some(serde_json::to_string(&self).unwrap())
    }
}
