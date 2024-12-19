use std::sync::Arc;

use asknothingx2_util::{
    api::{APIRequest, HeaderBuilder, HeaderMap, Method},
    oauth::{AccessToken, ClientId},
};
use chrono::{DateTime, FixedOffset};
use serde::{de, Deserialize, Deserializer, Serialize};
use url::Url;

use crate::{Error, GetUsersError, Result};

/// https://dev.twitch.tv/docs/api/reference/#get-users
/// To include the user’s verified email address in the response,
/// you must use a user access token that includes the
/// user:read:email scope.
#[derive(Debug)]
pub struct GetUsers {
    access_token: Arc<AccessToken>,
    client_id: Arc<ClientId>,
    id: Vec<String>,
    login: Vec<String>,
    #[cfg(feature = "test")]
    test_url: crate::test_url::TestUrlHold,
}

impl GetUsers {
    pub fn new(access_token: Arc<AccessToken>, client_id: Arc<ClientId>) -> Self {
        Self {
            access_token,
            client_id,
            id: Vec::new(),
            login: Vec::new(),
            #[cfg(feature = "test")]
            test_url: crate::test_url::TestUrlHold::default(),
        }
    }

    fn get_url(&self) -> Url {
        #[cfg(feature = "test")]
        if let Some(url) = self.test_url.get_test_url() {
            return url;
        }

        let mut url = Url::parse(crate::TWITCH_API_BASE).unwrap();
        url.path_segments_mut().unwrap().push("users");
        url
    }

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

#[cfg(feature = "test")]
crate::impl_testurl!(GetUsers);

impl APIRequest for GetUsers {
    fn method(&self) -> Method {
        Method::GET
    }

    fn headers(&self) -> HeaderMap {
        HeaderBuilder::new()
            .authorization("Bearer", self.access_token.secret().as_str())
            .client_id(self.client_id.as_str())
            .build()
    }

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
    pub email: Option<String>,
}

#[cfg(test)]
mod tests {
    use crate::{
        api_general, expect_APIRequest, expect_headers, expect_response_json, GetUsersResponse,
    };

    use super::GetUsers;

    #[test]
    fn get_users_id() {
        let get_user = api_general!(GetUsers);

        let get_user = get_user.add_id("141981764".to_string()).unwrap();

        let expected_headers = expect_headers!();

        expect_APIRequest!(
            GET,
            expected_headers,
            "https://api.twitch.tv/helix/users?id=141981764",
            json = None,
            text = None,
            urlencoded = None,
            get_user
        );
    }

    #[test]
    fn get_users_login() {
        let get_user = api_general!(GetUsers);

        let get_user = get_user.add_login("twitchdev".to_string()).unwrap();

        let expected_headers = expect_headers!();

        expect_APIRequest!(
            GET,
            expected_headers,
            "https://api.twitch.tv/helix/users?login=twitchdev",
            json = None,
            text = None,
            urlencoded = None,
            get_user
        );
    }

    #[test]
    fn get_users_id_login() {
        let get_user = api_general!(GetUsers);

        let get_user = get_user
            .add_login("twitchdev".to_string())
            .unwrap()
            .add_id("141981764".to_string())
            .unwrap();

        let expected_headers = expect_headers!();

        expect_APIRequest!(
            GET,
            expected_headers,
            "https://api.twitch.tv/helix/users?id=141981764&login=twitchdev",
            json = None,
            text = None,
            urlencoded = None,
            get_user
        );
    }

    #[test]
    fn get_users_login_max() {
        let get_user = api_general!(GetUsers);

        let get_user = get_user.add_login("twitchdev".to_string()).unwrap();
        let over_logins = vec!["twitchdev"; 100];
        let get_user = get_user.add_logins(over_logins);

        assert!(get_user.is_err());
    }

    #[test]
    fn response_get_users() {
        expect_response_json!("{\n  \"data\": [\n    {\n      \"id\": \"141981764\",\n      \"login\": \"twitchdev\",\n      \"display_name\": \"TwitchDev\",\n      \"type\": \"\",\n      \"broadcaster_type\": \"partner\",\n      \"description\": \"Supporting third-party developers building Twitch integrations from chatbots to game integrations.\",\n      \"profile_image_url\": \"https://static-cdn.jtvnw.net/jtv_user_pictures/8a6381c7-d0c0-4576-b179-38bd5ce1d6af-profile_image-300x300.png\",\n      \"offline_image_url\": \"https://static-cdn.jtvnw.net/jtv_user_pictures/3f13ab61-ec78-4fe6-8481-8682cb3b0ac2-channel_offline_image-1920x1080.png\",\n      \"view_count\": 5980557,\n      \"email\": \"not-real@email.com\",\n      \"created_at\": \"2016-12-14T20:32:28Z\"\n    }\n  ]\n}",
            GetUsersResponse
        );
    }
}
