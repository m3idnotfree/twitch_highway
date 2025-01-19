use serde::Serialize;

use crate::types::UserId;

request_struct!(
    #[derive(Serialize)]
    CheckAutoModStatusRequest {
        required {
            data: Vec<CheckAutoMod>
        }
    };
    impl_body: true
);

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
    };
    impl_body: true
);

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
    };
    impl_body: true
);

request_struct!(
    #[derive(Serialize)]
    BanUsersRequest{
        required {
            data: BanUserRequest
        }
    };
    impl_body: true
);

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
    };
    impl_body: true
);

request_struct!(
    #[derive(Serialize)]
    UpdateShieldModeStatusRequest {
        required {
            is_active: bool
        }
    };
    impl_body: true
);

request_struct!(
    #[derive(Serialize)]
    WarnChatUserRequest {
        required {
            data: Vec<WarnChatUser>
        }
    };
    impl_body: true
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
