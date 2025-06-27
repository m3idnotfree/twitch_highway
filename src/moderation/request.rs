use serde::Serialize;

use crate::types::UserId;

define_request!(
    #[derive(Serialize)]
    CheckAutoModStatusRequest {
        req: {
            data: Vec<CheckAutoMod>
        };
        into_request_body
    }
);

define_request!(
    #[derive(Serialize)]
     CheckAutoMod {
        req: {
            msg_id: String,
            msg_text: String,
        }
    }
);

define_request!(
    #[derive(Serialize)]
     ManageHeldAutoModMeussageRequest {
        req: {
            user_id: UserId,
            msg_id: String,
            action: AutoModAction
        };
        into_request_body
    }
);

#[derive(Debug, Serialize)]
pub enum AutoModAction {
    ALLOW,
    DENY,
}

define_request!(
    #[derive(Default, Serialize)]
    UpdateAutoModSettingsRequest {
        opts: {
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
        };
        into_request_body
    }
);

define_request!(
    #[derive(Serialize)]
    BanUsersRequest{
        req: {
            data: BanUserRequest
        };
        into_request_body
    }
);

define_request!(
    #[derive(Serialize)]
    BanUserRequest {
        req: {
            user_id: UserId
        },
        opts: {
            duration: u64,
            reason: String
       }
    }
);

define_request!(
    #[derive(Serialize)]
    AddBlockedTermRequest {
        req: {
            text: String
        };
        into_request_body
    }
);

define_request!(
    #[derive(Serialize)]
    UpdateShieldModeStatusRequest {
        req: {
            is_active: bool
        };
        into_request_body
    }
);

define_request!(
    #[derive(Serialize)]
    WarnChatUserRequest {
        req: {
            data: Vec<WarnChatUser>
        };
        into_request_body
    }
);

define_request!(
    #[derive(Serialize)]
    WarnChatUser {
        req: {
            user_id: UserId,
            reason: String
        }
    }
);
