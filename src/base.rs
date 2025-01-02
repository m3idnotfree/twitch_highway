use std::fmt;

use asknothingx2_util::{
    api::{HeaderBuilder, HeaderMap},
    oauth::{AccessToken, ClientId},
};
use url::Url;

pub trait TwitchAPIBase {
    fn access_token(&self) -> &AccessToken;
    fn client_id(&self) -> &ClientId;

    fn build_headers(&self) -> HH {
        HH::base(self.access_token(), self.client_id())
    }
    fn build_url(&self) -> UU {
        UU::new()
    }
}

pub struct TwitchAPI {
    access_token: AccessToken,
    client_id: ClientId,
}

impl TwitchAPI {
    pub fn new(access_token: AccessToken, client_id: ClientId) -> Self {
        Self {
            access_token,
            client_id,
        }
    }
}
impl TwitchAPIBase for TwitchAPI {
    fn access_token(&self) -> &AccessToken {
        &self.access_token
    }
    fn client_id(&self) -> &ClientId {
        &self.client_id
    }
}

pub struct HH(HeaderBuilder);
impl HH {
    pub fn base(access_token: &AccessToken, client_id: &ClientId) -> Self {
        Self(
            HeaderBuilder::new()
                .authorization("Bearer", access_token.secret().as_str())
                .client_id(client_id.as_str()),
        )
    }

    pub fn json(mut self) -> Self {
        self.0 = self.0.content_type_json();
        self
    }

    pub fn build(self) -> HeaderMap {
        self.0.build()
    }
}

pub struct UU(Url);

impl Default for UU {
    fn default() -> Self {
        UU(url::Url::parse(crate::TWITCH_API_BASE).unwrap())
    }
}

impl UU {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn path<T: AsRef<str>, L: IntoIterator<Item = T>>(mut self, path: L) -> Self {
        self.0.path_segments_mut().unwrap().extend(path);
        self
    }
    pub fn query<T: AsRef<str>, L: IntoIterator<Item = (T, T)>>(mut self, querys: L) -> Self {
        self.0.query_pairs_mut().extend_pairs(querys);
        self
    }
    pub fn query_option<T: Into<String>>(mut self, key: &str, value: Option<T>) -> Self {
        if let Some(value) = value {
            self.0.query_pairs_mut().append_pair(key, &value.into());
        }
        self
    }

    pub fn query_option_display<T: fmt::Display>(mut self, key: &str, value: Option<T>) -> Self {
        if let Some(value) = value {
            self.0
                .query_pairs_mut()
                .append_pair(key, &value.to_string());
        }
        self
    }

    pub fn query_option_extend<K: AsRef<str>, T: AsRef<str>, L: IntoIterator<Item = (T, K)>>(
        mut self,
        querys: Option<L>,
    ) -> Self {
        if let Some(querys) = querys {
            self.0.query_pairs_mut().extend_pairs(querys);
        }
        self
    }
    pub fn build(self) -> Url {
        self.0
    }
}
