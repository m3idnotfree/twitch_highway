use serde::{Deserialize, Serialize};

use crate::types::UserId;

define_request!(
    #[derive(Serialize)]
    CheckAutoModStatusRequest<'a> {
        req: {
            data: &'a [CheckAutoMod]
        };
        into_json
    }
);

define_request!(
    #[derive(Serialize, Deserialize)]
     CheckAutoMod {
        req: {
            msg_id: String | into,
            msg_text: String | into,
        }
    }
);

define_request!(
    #[derive(Serialize)]
     ManageHeldAutoModMeussageRequest<'a> {
        req: {
            user_id: &'a UserId,
            msg_id: &'a str,
            action: AutoModAction
        };
        into_json
    }
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
        into_json
    }
);

define_request!(
    #[derive(Serialize)]
    BanUserRequest<'a> {
        req: {
            user_id: &'a UserId
        },
        opts: {
            #[serde(skip_serializing_if = "Option::is_none")]
            duration: u64,
            #[serde(skip_serializing_if = "Option::is_none")]
            reason: &'a str
       }
    }
);

define_request!(
    #[derive(Serialize)]
    BanUserRequestWrapper<'a>{
        req: {
            data: BanUserRequest<'a>
        };
        into_json
    }
);

define_request!(
    #[derive(Serialize)]
    AddBlockedTermRequest<'a> {
        req: {
            text: &'a str
        };
        into_json
    }
);

define_request!(
    #[derive(Serialize)]
    UpdateShieldModeStatusRequest {
        req: {
            is_active: bool
        };
        into_json
    }
);

define_request!(
    #[derive(Serialize)]
    WarnChatUserRequest<'a> {
        req: {
            data: WarnChatUser<'a>
        };
        into_json
    }
);

define_request!(
    #[derive(Serialize)]
    WarnChatUser<'a> {
        req: {
            user_id: &'a UserId,
            reason: &'a str
        }
    }
);

#[cfg(test)]
mod tests {
    use crate::moderation::request::AutoModAction;

    #[test]
    fn automod_action_enum() {
        let actions = vec![
            (AutoModAction::ALLOW, "ALLOW"),
            (AutoModAction::DENY, "DENY"),
        ];

        for (action, expected_str) in actions {
            let serialized = serde_json::to_string(&action).unwrap();
            assert_eq!(serialized, format!("\"{}\"", expected_str));

            let deserialized: AutoModAction = serde_json::from_str(&serialized).unwrap();
            let re_serialized = serde_json::to_string(&deserialized).unwrap();
            assert_eq!(serialized, re_serialized);
        }
    }
}
