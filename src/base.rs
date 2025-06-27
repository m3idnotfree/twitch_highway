use std::sync::LazyLock;

use asknothingx2_util::{
    api::{HeaderBuilder, HeaderMap},
    oauth::{AccessToken, ClientId},
};
use chrono::{DateTime, FixedOffset, SecondsFormat};
use url::Url;

use crate::types::JWTToken;

const TWITCH_API_BASE: &str = "https://api.twitch.tv/helix";
const BEARER: &str = "Bearer";

static BASE_URL: LazyLock<Url> = LazyLock::new(|| url::Url::parse(TWITCH_API_BASE).unwrap());

pub struct HeadersBuilder(HeaderBuilder);

impl HeadersBuilder {
    #[inline]
    pub fn base(access_token: &AccessToken, client_id: &ClientId) -> Self {
        let mut headers = HeaderBuilder::new();
        headers
            .authorization(BEARER, access_token.secret().as_str())
            .client_id(client_id.as_str());
        Self(headers)
    }

    #[inline]
    pub fn base_with_jwt(jwt_token: &JWTToken, client_id: &ClientId) -> Self {
        let mut headers = HeaderBuilder::new();
        headers
            .authorization(BEARER, jwt_token.as_str())
            .client_id(client_id.as_str());
        Self(headers)
    }

    #[inline]
    pub fn json(&mut self) -> &mut Self {
        self.0.content_type_json();
        self
    }

    #[inline]
    pub fn build(self) -> HeaderMap {
        self.0.build()
    }
}

pub struct UrlBuilder(Url);

impl Default for UrlBuilder {
    fn default() -> Self {
        UrlBuilder(BASE_URL.clone())
    }
}

impl UrlBuilder {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn path<T: AsRef<str>, L: IntoIterator<Item = T>>(&mut self, path: L) -> &mut Self {
        self.0.path_segments_mut().unwrap().extend(path);
        self
    }

    #[inline]
    pub fn query<K: AsRef<str>>(&mut self, key: &str, value: K) -> &mut Self {
        self.0.query_pairs_mut().append_pair(key, value.as_ref());
        self
    }

    #[inline]
    pub fn query_opt<T: AsRef<str>>(&mut self, key: &str, value: Option<T>) -> &mut Self {
        if let Some(value) = value {
            self.0.query_pairs_mut().append_pair(key, value.as_ref());
        }
        self
    }

    pub fn query_extend<K: AsRef<str>, V: AsRef<str>, L: IntoIterator<Item = (K, V)>>(
        &mut self,
        querys: L,
    ) -> &mut Self {
        self.0.query_pairs_mut().extend_pairs(querys);
        self
    }

    pub fn query_opt_extend<K: AsRef<str>, V: AsRef<str>, L: IntoIterator<Item = (K, V)>>(
        &mut self,
        querys: Option<L>,
    ) -> &mut Self {
        if let Some(querys) = querys {
            self.0.query_pairs_mut().extend_pairs(querys);
        }
        self
    }

    #[inline]
    pub fn query_u64(&mut self, key: &str, value: u64) -> &mut Self {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(value);
        self.0.query_pairs_mut().append_pair(key, s);
        self
    }

    #[inline]
    pub fn query_opt_u64(&mut self, key: &str, value: Option<u64>) -> &mut Self {
        if let Some(v) = value {
            self.query_u64(key, v);
        }
        self
    }
    /// RFC 3339 format
    /// Includes seconds presision
    /// Uses "Z" suffix
    pub fn date(&mut self, key: &str, value: DateTime<FixedOffset>) -> &mut Self {
        self.0
            .query_pairs_mut()
            .append_pair(key, &value.to_rfc3339_opts(SecondsFormat::Secs, true));
        self
    }

    pub fn date_opt(&mut self, key: &str, value: Option<DateTime<FixedOffset>>) -> &mut Self {
        if let Some(date) = value {
            self.0
                .query_pairs_mut()
                .append_pair(key, &date.to_rfc3339_opts(SecondsFormat::Secs, true));
        }
        self
    }

    #[inline]
    pub fn build(self) -> Url {
        self.0
    }
}
