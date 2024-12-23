use asknothingx2_util::api::APIRequest;
use chrono::{DateTime, FixedOffset};
use serde::{de, Deserialize, Deserializer, Serialize};
use url::Url;

use crate::{serde_util::serialize_none_as_empty_string, Error, GetUsersError, Result};

endpoint!(
/// https://dev.twitch.tv/docs/api/reference/#get-users
/// To include the user’s verified email address in the response,
/// you must use a user access token that includes the
/// user:read:email scope.
  GetUsers {
    id: Vec<String>,
    login: Vec<String>
    };
    new = {
        params = {},
        init = {
            id :Vec::new(),
            login: Vec::new()
        }
    },
    url = ["users"],
);

impl GetUsers {
    fn specify_check(&self) -> usize {
        self.id.len() + self.login.len()
    }

    pub fn add_id<S>(mut self, id: S) -> Result<Self>
    where
        S: Into<String>,
    {
        if self.specify_check() == 100 {
            Err(Error::UserError(GetUsersError::AddIdError(format!(
                "error add id curret len: {}",
                self.id.len() + self.login.len()
            ))))
        } else {
            self.id.push(id.into());

            Ok(self)
        }
    }

    pub fn add_ids<S>(mut self, ids: Vec<S>) -> Result<Self>
    where
        S: Into<String>,
    {
        if self.specify_check() + ids.len() > 100 {
            Err(Error::UserError(GetUsersError::AddIdError(format!(
                "error add id curret len: {}",
                self.specify_check() + ids.len()
            ))))
        } else {
            self.id
                .extend(ids.into_iter().map(|x| x.into()).collect::<Vec<String>>());
            Ok(self)
        }
    }

    pub fn add_login<S>(mut self, login: S) -> Result<Self>
    where
        S: Into<String>,
    {
        if self.specify_check() == 100 {
            Err(Error::UserError(GetUsersError::AddIdError(format!(
                "error add id curret len: {}",
                self.specify_check()
            ))))
        } else {
            self.login.push(login.into());
            Ok(self)
        }
    }

    pub fn add_logins<S>(mut self, logins: Vec<S>) -> Result<Self>
    where
        S: Into<String>,
    {
        if self.specify_check() + logins.len() > 100 {
            Err(Error::UserError(GetUsersError::AddIdError(format!(
                "error add id curret len: {}",
                self.specify_check() + logins.len()
            ))))
        } else {
            self.login.extend(
                logins
                    .into_iter()
                    .map(|x| x.into())
                    .collect::<Vec<String>>(),
            );
            Ok(self)
        }
    }
}

impl APIRequest for GetUsers {
    impl_api_request_method!(GET);
    impl_api_request_header!();

    fn url(&self) -> Url {
        let mut url = self.get_url();

        if !self.id.is_empty() {
            let ids = self
                .id
                .clone()
                .into_iter()
                .map(|x| ("id".to_string(), x.to_string()))
                .collect::<Vec<(String, String)>>();

            url.query_pairs_mut().extend_pairs(ids);
        }

        if !self.login.is_empty() {
            let logins = self
                .login
                .clone()
                .into_iter()
                .map(|x| ("login".to_string(), x.to_string()))
                .collect::<Vec<(String, String)>>();

            url.query_pairs_mut().extend_pairs(logins);
        }

        url
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum GetUserType {
    Admin,
    GlobalMod,
    Staff,
    Normal,
}

impl Serialize for GetUserType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            GetUserType::Admin => serializer.serialize_str("admin"),
            GetUserType::GlobalMod => serializer.serialize_str("global_mod"),
            GetUserType::Staff => serializer.serialize_str("staff"),
            GetUserType::Normal => serializer.serialize_str(""),
        }
    }
}

impl<'de> Deserialize<'de> for GetUserType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "admin" => Ok(GetUserType::Admin),
            "global_mod" => Ok(GetUserType::GlobalMod),
            "staff" => Ok(GetUserType::Staff),
            "" => Ok(GetUserType::Normal),
            _ => Err(de::Error::custom("invalid user type")),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum BroadcasterType {
    Affiliate,
    Partner,
    Normal,
}

impl Serialize for BroadcasterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            BroadcasterType::Normal => serializer.serialize_str(""),
            BroadcasterType::Affiliate => serializer.serialize_str("affiliate"),
            BroadcasterType::Partner => serializer.serialize_str("partner"),
        }
    }
}

impl<'de> Deserialize<'de> for BroadcasterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "" => Ok(BroadcasterType::Normal),
            "affiliate" => Ok(BroadcasterType::Affiliate),
            "partner" => Ok(BroadcasterType::Partner),
            _ => Err(de::Error::custom("invalid broadcaster type")),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUsersResponse {
    pub data: Vec<User>,
}

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
    /// NOTE: This field has been deprecated (see Get Users API endpoint – “view_count” deprecation).
    /// Any data in this field is not valid and should not be used.
    pub view_count: u64,
    pub created_at: DateTime<FixedOffset>,

    pub offline_image_url: String,
    /// if the user access token includes the user:read:email scope.
    #[serde(serialize_with = "serialize_none_as_empty_string")]
    pub email: Option<String>,
}
