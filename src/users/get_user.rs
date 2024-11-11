use std::sync::Arc;

use asknothingx2_util::{
    api::{APIRequest, HeaderBuilder, HeaderMap, Method},
    oauth::{AccessToken, ClientId},
};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{Error, GetUsersError, Result};

#[derive(Debug)]
pub struct GetUsers {
    access_token: Arc<AccessToken>,
    client_id: Arc<ClientId>,
    url: Url,
    id: Vec<String>,
    login: Vec<String>,
}

impl GetUsers {
    pub fn new(access_token: Arc<AccessToken>, client_id: Arc<ClientId>, url: Url) -> Self {
        Self {
            access_token,
            client_id,
            url,
            id: Vec::new(),
            login: Vec::new(),
        }
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

impl APIRequest for GetUsers {
    fn method(&self) -> Method {
        Method::GET
    }

    fn headers(&self) -> HeaderMap {
        HeaderBuilder::new()
            .authorization("Bearer", self.access_token.secret().as_str())
            .append("Cliend-ID", self.client_id.as_str())
            .unwrap()
            .build()
    }

    fn url(&self) -> Url {
        let mut url = self.url.clone();
        if !self.id.is_empty() {
            let afs = self
                .id
                .clone()
                .into_iter()
                .map(|x| ("id".to_string(), x.to_string()))
                .collect::<Vec<(String, String)>>();

            url.query_pairs_mut().extend_pairs(afs);
        }

        if !self.login.is_empty() {
            let afs = self
                .login
                .clone()
                .into_iter()
                .map(|x| ("login".to_string(), x.to_string()))
                .collect::<Vec<(String, String)>>();

            url.query_pairs_mut().extend_pairs(afs);
        }

        url
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
    pub kind: String,
    pub broadcaster_type: String,
    pub description: String,
    pub profile_image_url: String,
    pub view_count: u64,
    pub created_at: String,
}

// #[cfg(test)]
// mod tests {
//     use std::sync::Arc;
//
//     use url::Url;
//
//     use crate::users::GetUsers;
//
//     #[test]
//     fn get_users_id() {
//         let get_user = GetUsers {
//             login: Vec::new(),
//             client_id: Arc::new(ClientId::new("uo6dggojyb8d6soh92zknwmi5ej1q2".to_string())),
//             access_token: Arc::new(AccessToken::new(
//                 "cfabdegwdoklmawdzdo98xt2fo512y".to_string(),
//             )),
//             id: Vec::new(),
//             url: Url::parse("https://api.twitch.tv/helix/users").unwrap(),
//         };
//
//         let get_user = get_user.add_id("141981764".to_string()).unwrap();
//
//         let expected_headers = auth_client_id_headers(
//             &AccessToken::new("cfabdegwdoklmawdzdo98xt2fo512y".to_string()),
//             &ClientId::new("uo6dggojyb8d6soh92zknwmi5ej1q2".to_string()),
//         );
//
//         assert_eq!(expected_headers, get_user.headers());
//         assert_eq!(None, get_user.body().unwrap());
//         assert_eq!(http::method::Method::GET, get_user.method());
//         assert_eq!(
//             "https://api.twitch.tv/helix/users?id=141981764".to_string(),
//             get_user.url()
//         );
//     }
//
//     #[test]
//     fn get_users_login() {
//         let get_user = GetUsers {
//             login: Vec::new(),
//             client_id: Arc::new(ClientId::new("uo6dggojyb8d6soh92zknwmi5ej1q2".to_string())),
//             access_token: Arc::new(AccessToken::new(
//                 "uo6dggojyb8d6soh92zknwmi5ej1q2".to_string(),
//             )),
//             id: Vec::new(),
//             url: Url::parse("https://api.twitch.tv/helix/users").unwrap(),
//         };
//
//         let get_user = get_user.add_login("twitchdev".to_string()).unwrap();
//
//         assert_eq!(http::method::Method::GET, get_user.method());
//         assert_eq!(
//             "https://api.twitch.tv/helix/users?login=twitchdev".to_string(),
//             get_user.url()
//         );
//     }
//
//     #[test]
//     fn get_users_id_login() {
//         let get_user = GetUsers {
//             login: Vec::new(),
//             client_id: Arc::new(ClientId::new("uo6dggojyb8d6soh92zknwmi5ej1q2".to_string())),
//             access_token: Arc::new(AccessToken::new(
//                 "uo6dggojyb8d6soh92zknwmi5ej1q2".to_string(),
//             )),
//             id: Vec::new(),
//             url: Url::parse("https://api.twitch.tv/helix/users").unwrap(),
//         };
//
//         let get_user = get_user
//             .add_login("twitchdev".to_string())
//             .unwrap()
//             .add_id("141981764".to_string())
//             .unwrap();
//
//         assert_eq!(http::method::Method::GET, get_user.method());
//         assert_eq!(
//             "https://api.twitch.tv/helix/users?id=141981764&login=twitchdev".to_string(),
//             get_user.url()
//         );
//     }
// }
