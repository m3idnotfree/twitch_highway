use serde::Serialize;

use crate::types::UserId;

define_request!(
    #[derive(Serialize)]
    CheckAutoModStatusRequest<'a> {
        req: {
            data: &'a [CheckAutoMod]
        };
        to_json
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
     ManageHeldAutoModMeussageRequest<'a> {
        req: {
            user_id: UserId,
            msg_id: &'a str,
            action: AutoModAction
        };
        to_json
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
        to_json
    }
);

define_request!(
    #[derive(Serialize)]
    BanUsersRequest<'a>{
        req: {
            data: BanUserRequest<'a>
        };
        to_json
    }
);

define_request!(
    #[derive(Serialize)]
    BanUserRequest<'a> {
        req: {
            user_id: UserId
        },
        opts: {
            duration: u64,
            reason: &'a str
       }
    }
);

define_request!(
    #[derive(Serialize)]
    AddBlockedTermRequest<'a> {
        req: {
            text: &'a str
        };
        to_json
    }
);

define_request!(
    #[derive(Serialize)]
    UpdateShieldModeStatusRequest {
        req: {
            is_active: bool
        };
        to_json
    }
);

define_request!(
    #[derive(Serialize)]
    WarnChatUserRequest<'a> {
        req: {
            data: &'a[WarnChatUser<'a>]
        };
        to_json
    }
);

define_request!(
    #[derive(Serialize)]
    WarnChatUser<'a> {
        req: {
            user_id: UserId,
            reason: &'a str
        }
    }
);
