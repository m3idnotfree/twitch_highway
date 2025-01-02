use asknothingx2_util::serde::serialize_none_as_empty_string;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub login: String,
    pub display_name: String,
    #[serde(rename = "type")]
    pub kind: GetUserType,
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
pub enum GetUserType {
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
    user_id: String,
    user_login: String,
    display_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserExtension {
    id: String,
    version: String,
    name: String,
    can_activate: bool,
    #[serde(rename = "type")]
    kind: Vec<ExtensionType>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExtensionType {
    Component,
    Mobile,
    Overlay,
    Panel,
}

//#[derive(Debug, Serialize, Deserialize)]
//pub struct UserActiveExtensions {
//    panel: HashMap<String, Panel>,
//    overlay: HashMap<String, Overlay>,
//    component: HashMap<String, Component>,
//}
//
//#[derive(Debug, Serialize, Deserialize)]
//pub struct Panel {
//    active: bool,
//    #[serde(skip_serializing_if = "Option::is_none")]
//    id: Option<String>,
//    #[serde(skip_serializing_if = "Option::is_none")]
//    version: Option<String>,
//    #[serde(skip_serializing_if = "Option::is_none")]
//    name: Option<String>,
//}
//
//#[derive(Debug, Serialize, Deserialize)]
//pub struct Overlay {
//    active: bool,
//    #[serde(skip_serializing_if = "Option::is_none")]
//    id: Option<String>,
//    #[serde(skip_serializing_if = "Option::is_none")]
//    version: Option<String>,
//    #[serde(skip_serializing_if = "Option::is_none")]
//    name: Option<String>,
//}
//
//#[derive(Debug, Serialize, Deserialize)]
//pub struct Component {
//    active: bool,
//    #[serde(skip_serializing_if = "Option::is_none")]
//    id: Option<String>,
//    #[serde(skip_serializing_if = "Option::is_none")]
//    version: Option<String>,
//    #[serde(skip_serializing_if = "Option::is_none")]
//    name: Option<String>,
//    #[serde(skip_serializing_if = "Option::is_none")]
//    x: Option<u64>,
//    #[serde(skip_serializing_if = "Option::is_none")]
//    y: Option<u64>,
//}
