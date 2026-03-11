use std::collections::HashMap;

use asknothingx2_util::serde::serialize_none_as_empty_string;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use twitch_oauth_token::Scope;

use crate::types::{ExtensionId, UserId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserType {
    Admin,
    GlobalMod,
    Staff,
    #[serde(rename(serialize = "", deserialize = ""))]
    Normal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BroadcasterType {
    Affiliate,
    Partner,
    #[serde(rename(serialize = "", deserialize = ""))]
    Normal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockUser {
    pub user_id: UserId,
    pub user_login: String,
    pub display_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserExtension {
    pub id: ExtensionId,
    pub version: String,
    pub name: String,
    pub can_activate: bool,
    #[serde(rename = "type")]
    pub kind: Vec<ExtensionType>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExtensionType {
    Component,
    Mobile,
    Overlay,
    Panel,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct UserActiveExtensions {
    panel: HashMap<String, Panel>,
    overlay: HashMap<String, Overlay>,
    component: HashMap<String, Component>,
}

impl UserActiveExtensions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_panel(mut self, panel: Panel) -> Self {
        let count = self.panel.len() + 1;
        self.panel.insert(format!("{}", count), panel);
        self
    }

    pub fn add_panels(mut self, panels: Vec<Panel>) -> Self {
        let count = self.panel.len() + 1;
        let mut c = count;
        for panel in panels.into_iter() {
            self.panel.insert(format!("{}", c), panel);
            c += 1;
        }
        self
    }

    pub fn add_overlay(mut self, overlay: Overlay) -> Self {
        let count = self.overlay.len() + 1;
        self.overlay.insert(format!("{}", count), overlay);
        self
    }

    pub fn add_component(mut self, component: Component) -> Self {
        let count = self.overlay.len() + 1;
        self.component.insert(format!("{}", count), component);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BlockSourceContext {
    Chat,
    Whisper,
}

impl BlockSourceContext {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Chat => "chat",
            Self::Whisper => "whisper",
        }
    }
}

impl AsRef<str> for BlockSourceContext {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<BlockSourceContext> for String {
    fn from(value: BlockSourceContext) -> Self {
        value.as_str().to_string()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BlockReason {
    Harassment,
    Spam,
    Other,
}

impl BlockReason {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Harassment => "harassment",
            Self::Spam => "spam",
            Self::Other => "other",
        }
    }
}

impl AsRef<str> for BlockReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<BlockReason> for String {
    fn from(value: BlockReason) -> String {
        value.as_str().to_string()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Panel {
    active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<ExtensionId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

impl Panel {
    pub fn new(active: bool) -> Self {
        Self {
            active,
            id: None,
            version: None,
            name: None,
        }
    }

    pub fn id(mut self, value: ExtensionId) -> Self {
        self.id = Some(value);
        self
    }

    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Overlay {
    active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<ExtensionId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

impl Overlay {
    pub fn new(active: bool) -> Self {
        Self {
            active,
            id: None,
            version: None,
            name: None,
        }
    }

    pub fn id(mut self, value: ExtensionId) -> Self {
        self.id = Some(value);
        self
    }

    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<ExtensionId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    x: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    y: Option<u64>,
}

impl Component {
    pub fn new(active: bool) -> Self {
        Self {
            active,
            id: None,
            version: None,
            name: None,
            x: None,
            y: None,
        }
    }

    pub fn id(mut self, value: ExtensionId) -> Self {
        self.id = Some(value);
        self
    }

    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn x(mut self, value: u64) -> Self {
        self.x = Some(value);
        self
    }

    pub fn y(mut self, value: u64) -> Self {
        self.y = Some(value);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAuthorization {
    pub user_id: UserId,
    pub user_name: String,
    pub user_login: String,
    pub scopes: Vec<Scope>,
}
