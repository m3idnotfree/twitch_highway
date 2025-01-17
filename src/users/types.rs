use std::collections::HashMap;

use asknothingx2_util::serde::serialize_none_as_empty_string;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::{
    types::{Id, UserId},
    IntoRequestBody,
};

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

#[derive(Debug, Serialize, Deserialize)]
pub struct UserActiveExtensions {
    pub panel: HashMap<String, Panel>,
    pub overlay: HashMap<String, Overlay>,
    pub component: HashMap<String, Component>,
}

impl UserActiveExtensions {
    pub fn new(
        panel: HashMap<String, Panel>,
        overlay: HashMap<String, Overlay>,
        component: HashMap<String, Component>,
    ) -> Self {
        Self {
            panel,
            overlay,
            component,
        }
    }
}

impl IntoRequestBody for UserActiveExtensions {
    fn as_body(&self) -> Option<String> {
        Some(serde_json::to_string(self).unwrap())
    }
}

request_struct!(
    #[derive( Serialize, Deserialize)]
    Panel {
        required {
            pub active: bool,
        },
        optional {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub id: Id,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub version: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub name: String,
        }
    }
);

request_struct!(
    #[derive(Serialize, Deserialize)]
    Overlay {
        required {
            pub active: bool,
        },
        optional {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub id: Id,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub version: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub name: String,
        }
    }
);

request_struct!(
    #[derive(Serialize, Deserialize)]
    Component {
        required {
            pub active: bool
        },
        optional {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub id: Id,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub version: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub name: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub x: u64,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub y: u64
        }
    }
);
