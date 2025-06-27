use std::collections::HashMap;

use asknothingx2_util::serde::serialize_none_as_empty_string;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::types::{Id, UserId};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Id,
    pub login: String,
    pub display_name: String,
    #[serde(rename = "type")]
    pub kind: UserType,
    pub broadcaster_type: BroadcasterType,
    pub description: String,
    pub profile_image_url: String,
    pub offline_image_url: String,
    /// NOTE: This field has been deprecated (see Get Users API endpoint – “view_count” deprecation).
    /// Any data in this field is not valid and should not be used.
    pub view_count: u64,
    /// if the user access token includes the user:read:email scope.
    #[serde(serialize_with = "serialize_none_as_empty_string")]
    pub email: Option<String>,
    pub created_at: DateTime<FixedOffset>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserType {
    Admin,
    GlobalMod,
    Staff,
    #[serde(rename(serialize = "", deserialize = ""))]
    Normal,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BroadcasterType {
    Affiliate,
    Partner,
    #[serde(rename(serialize = "", deserialize = ""))]
    Normal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockUser {
    pub user_id: UserId,
    pub user_login: String,
    pub display_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserExtension {
    pub id: Id,
    pub version: String,
    pub name: String,
    pub can_activate: bool,
    #[serde(rename = "type")]
    pub kind: Vec<ExtensionType>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExtensionType {
    Component,
    Mobile,
    Overlay,
    Panel,
}

define_request!(
    #[derive(Debug, Serialize, Deserialize)]
    UserActiveExtensions {
        req: {
            panel: HashMap<String, Panel>,
            overlay: HashMap<String, Overlay>,
            component: HashMap<String, Component>,
        };
        into_request_body
    }
);

define_request!(
    #[derive(Debug, Serialize, Deserialize)]
    Panel {
        req: {
            active: bool,
        },
        opts: {
            #[serde(skip_serializing_if = "Option::is_none")]
            id: Id,
            #[serde(skip_serializing_if = "Option::is_none")]
            version: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            name: String,
        }
    }
);

define_request!(
    #[derive(Debug, Serialize, Deserialize)]
    Overlay {
        req: {
            active: bool,
        },
        opts: {
            #[serde(skip_serializing_if = "Option::is_none")]
            id: Id,
            #[serde(skip_serializing_if = "Option::is_none")]
            version: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            name: String,
        }
    }
);

define_request!(
    #[derive(Debug, Serialize, Deserialize)]
    Component {
        req: {
            active: bool
        },
        opts: {
            #[serde(skip_serializing_if = "Option::is_none")]
            id: Id,
            #[serde(skip_serializing_if = "Option::is_none")]
            version: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            name: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            x: u64,
            #[serde(skip_serializing_if = "Option::is_none")]
            y: u64
        }
    }
);
