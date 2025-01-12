use asknothingx2_eventsub::twitch::subscription_types::request;
use serde::Serialize;

use crate::{types::UserId, AsBody};

request_struct!(
    #[derive(Debug,Serialize)]
    CheckAutoModStatusRequest {
        required {
            data: Vec<CheckAutoMod>
        }
    }
);

impl AsBody for CheckAutoModStatusRequest {
    fn as_body(&self) -> Option<String> {
        Some(serde_json::to_string(&self).unwrap())
    }
}

request_struct!(
    #[derive(Debug,Serialize)]
     CheckAutoMod {
        required {
            msg_id: String,
            msg_text: String,
        }
    }
);

request_struct!(
    #[derive(Debug,Serialize)]
     ManageHeldAutoModMeussageRequest {
        required {
            user_id: UserId,
            msg_id: String,
            action: AutoModAction
        }
    }
);
impl AsBody for ManageHeldAutoModMeussageRequest {
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
    #[derive(Debug, Default, Serialize)]
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

impl AsBody for UpdateAutoModSettingsRequest {
    fn as_body(&self) -> Option<String> {
        Some(serde_json::to_string(&self).unwrap())
    }
}

request_struct!(
    #[derive(Debug, Serialize)]
    BanUsersRequest{
        required {
            data: Vec<BanUserRequest>
        }
    }
);
impl AsBody for BanUsersRequest {
    fn as_body(&self) -> Option<String> {
        Some(serde_json::to_string(&self).unwrap())
    }
}

request_struct!(
    #[derive(Debug, Serialize)]
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
    #[derive(Debug, Serialize)]
    AddBlockedTermRequest {
        required {
            text: String
        }
    }
);

impl AsBody for AddBlockedTermRequest {
    fn as_body(&self) -> Option<String> {
        Some(serde_json::to_string(&self).unwrap())
    }
}

request_struct!(
    #[derive(Debug, Serialize)]
    UpdateShieldModeStatusRequest {
        required {
            is_active: bool
        }
    }
);

impl AsBody for UpdateShieldModeStatusRequest {
    fn as_body(&self) -> Option<String> {
        Some(serde_json::to_string(&self).unwrap())
    }
}

request_struct!(
    #[derive(Debug, Serialize)]
    WarnChatUserRequest {
        required {
            data: Vec<WarnChatUser>
        }
    }
);

request_struct!(
    #[derive(Debug, Serialize)]
    WarnChatUser {
        required {
            user_id: UserId,
            reason: String
        }
    }
);

impl AsBody for WarnChatUserRequest {
    fn as_body(&self) -> Option<String> {
        Some(serde_json::to_string(&self).unwrap())
    }
}
