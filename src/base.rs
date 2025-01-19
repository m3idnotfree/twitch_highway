use asknothingx2_util::{
    api::{HeaderBuilder, HeaderMap},
    oauth::{AccessToken, ClientId},
};
use url::Url;

use chrono::{DateTime, FixedOffset, SecondsFormat};

use crate::types::JWTToken;

const TWITCH_API_BASE: &str = "https://api.twitch.tv/helix";
const BEARER: &str = "Bearer";

pub trait TwitchAPIBase {
    fn access_token(&self) -> &AccessToken;
    fn client_id(&self) -> &ClientId;
    fn build_headers(&self) -> HeadersBuilder {
        HeadersBuilder::base(self.access_token(), self.client_id())
    }
    fn build_jwt_headers(&self, jwt: &JWTToken) -> HeadersBuilder {
        HeadersBuilder::base_with_jwt(jwt, self.client_id())
    }
    fn build_url(&self) -> UrlBuilder {
        UrlBuilder::new()
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
        UrlBuilder(url::Url::parse(TWITCH_API_BASE).unwrap())
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

    pub fn query_pairs<T: IntoQueryPairs>(&mut self, querys: T) -> &mut Self {
        self.0
            .query_pairs_mut()
            .extend_pairs(querys.into_query_pairs());
        self
    }

    pub fn query_opt_pairs<T: IntoQueryPairs>(&mut self, querys: Option<T>) -> &mut Self {
        if let Some(querys) = querys {
            self.0
                .query_pairs_mut()
                .extend_pairs(querys.into_query_pairs());
        }
        self
    }

    #[inline]
    pub fn build(self) -> Url {
        self.0
    }
}

pub trait IntoQueryPairs {
    fn into_query_pairs(self) -> Vec<(&'static str, String)>;
}

#[cfg(any(
    feature = "analytics",
    feature = "bits",
    feature = "ccls",
    feature = "channel_points",
    feature = "channels",
    feature = "charity",
    feature = "chat",
    feature = "clips",
    feature = "entitlements",
    feature = "extensions",
    feature = "games",
    feature = "goals",
    feature = "guest-star",
    feature = "hype-train",
    feature = "moderation",
    feature = "polls",
    feature = "predictions",
    feature = "raid",
    feature = "schedule",
    feature = "search",
    feature = "streams",
    feature = "subscriptions",
    feature = "teams",
    feature = "users",
    feature = "videos",
    feature = "whispers",
))]
#[derive(Debug, Default)]
pub struct QueryParams(Vec<(&'static str, String)>);

#[cfg(any(
    feature = "analytics",
    feature = "bits",
    feature = "ccls",
    feature = "channel_points",
    feature = "channels",
    feature = "charity",
    feature = "chat",
    feature = "clips",
    feature = "entitlements",
    feature = "extensions",
    feature = "games",
    feature = "goals",
    feature = "guest-star",
    feature = "hype-train",
    feature = "moderation",
    feature = "polls",
    feature = "predictions",
    feature = "raid",
    feature = "schedule",
    feature = "search",
    feature = "streams",
    feature = "subscriptions",
    feature = "teams",
    feature = "users",
    feature = "videos",
    feature = "whispers",
))]
impl QueryParams {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
    #[cfg(any(
        feature = "ads",
        feature = "analytics",
        feature = "bits",
        feature = "ccls",
        feature = "channel_points",
        feature = "channels",
        feature = "charity",
        feature = "chat",
        feature = "clips",
        feature = "entitlements",
        feature = "extensions",
        feature = "games",
        feature = "goals",
        feature = "guest-star",
        feature = "hype-train",
        feature = "moderation",
        feature = "polls",
        feature = "predictions",
        feature = "raid",
        feature = "schedule",
        feature = "search",
        feature = "streams",
        feature = "subscriptions",
        feature = "teams",
        feature = "users",
        feature = "videos",
        feature = "whispers",
    ))]
    #[inline]
    pub fn push_opt<T: Into<String>>(&mut self, key: &'static str, value: Option<T>) -> &mut Self {
        if let Some(v) = value {
            self.0.push((key, v.into()));
        }
        self
    }
    #[cfg(any(
        feature = "clips",
        feature = "entitlements",
        feature = "games",
        feature = "schedule",
        feature = "streams",
        feature = "videos",
    ))]
    pub fn extend_opt<V: Into<String>, L: IntoIterator<Item = (&'static str, V)>>(
        &mut self,
        extend: Option<L>,
    ) -> &mut Self {
        if let Some(v) = extend {
            self.0.extend(v.into_iter().map(|(x, y)| (x, y.into())));
        }
        self
    }

    pub fn date_opt(
        &mut self,
        key: &'static str,
        value: Option<DateTime<FixedOffset>>,
    ) -> &mut Self {
        if let Some(date) = value {
            self.0
                .push((key, date.to_rfc3339_opts(SecondsFormat::Secs, true)));
        }
        self
    }

    #[inline]
    pub fn build(self) -> Vec<(&'static str, String)> {
        self.0
    }
}
