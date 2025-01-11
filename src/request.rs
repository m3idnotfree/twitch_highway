use std::fmt;

use asknothingx2_util::api::{api_request, APIRequest, APIResponse, HeaderMap, Method, StatusCode};
use serde::{Deserialize, Serialize};
use twitch_oauth_token::types::Scope;
use url::Url;

use crate::Error;

// https://rust-lang.github.io/rust-clippy/master/index.html#wrong_self_convention
pub trait AsBody {
    fn as_body(&self) -> Option<String> {
        None
    }
}

pub struct TwitchAPIRequest<T> {
    kind: EndpointType,
    url: Url,
    method: Method,
    header: HeaderMap,
    body: T,
    #[cfg(feature = "test")]
    test_url: crate::test_url::TestUrlHold,
}

impl<T> TwitchAPIRequest<T>
where
    T: AsBody,
{
    pub fn new(kind: EndpointType, url: Url, method: Method, header: HeaderMap, body: T) -> Self {
        Self {
            kind,
            header,
            method,
            url,
            body,
            #[cfg(feature = "test")]
            test_url: crate::test_url::TestUrlHold::default(),
        }
    }
    pub fn kind(&self) -> &EndpointType {
        &self.kind
    }
    pub fn header(&self) -> &HeaderMap {
        &self.header
    }
    pub fn method(&self) -> &Method {
        &self.method
    }
    pub fn url(&self) -> Url {
        #[cfg(feature = "test")]
        if let Ok(mut url) = self.test_url.get_test_url() {
            url.set_query(self.url.query());
            return url;
        }
        self.url.clone()
    }
    pub fn body(&self) -> &T {
        &self.body
    }

    pub async fn request(self) -> Result<APIResponse, Error> {
        let response = api_request(self).await?;

        Ok(APIResponse::from_response(response).await?)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct APIError {
    #[serde(with = "http_serde::status_code")]
    status: StatusCode,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

impl APIError {
    pub fn status(&self) -> StatusCode {
        self.status
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn error_details(&self) -> Option<&str> {
        self.error.as_deref()
    }
}

impl fmt::Display for APIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "API error ({}): {}{}",
            self.status,
            self.message,
            self.error
                .as_ref()
                .map(|e| format!(" - {}", e))
                .unwrap_or_default()
        )
    }
}

impl<T> APIRequest for TwitchAPIRequest<T>
where
    T: AsBody,
{
    fn url(&self) -> Url {
        self.url()
    }
    fn method(&self) -> Method {
        self.method.clone()
    }
    fn headers(&self) -> HeaderMap {
        self.header.clone()
    }
    fn json(&self) -> Option<String> {
        self.body.as_body()
    }
    fn text(&self) -> Option<Vec<u8>> {
        self.body.as_body().map(|x| x.into_bytes())
    }
    fn urlencoded(&self) -> Option<Vec<u8>> {
        self.body.as_body().map(|x| x.into_bytes())
    }
}

#[cfg(feature = "test")]
impl<L> crate::test_url::TestUrl for TwitchAPIRequest<L> {
    fn with_url<T: Into<String>>(mut self, url: T) -> Self {
        self.test_url.with_url(url);
        self
    }

    fn get_test_url(&self) -> Result<Url, crate::Error> {
        self.test_url.get_test_url()
    }
}

pub struct EmptyBody;
impl AsBody for EmptyBody {}

#[derive(Debug)]
pub enum EndpointType {
    // Ads
    StartCommercial,
    GetAdSchedule,
    SnoozeNextAd,
    // Analytics
    GetExtensionAnalytics,
    GetGameAnalytics,
    // Bits
    GetBitsLeaderboard,
    GetCheermotes,
    GetExtensionTransactions,
    // Channel
    GetChanelInformation,
    ModifyChannelInformation,
    GetChannelEditors,
    GetFollowedChannels,
    GetChannelFollowers,
    // Users
    GetUsers,
    UpdateUser,
    GetUserBlockList,
    BlockUser,
    UnblockUser,
    GetUserExtensions,
    GetUserActiveExtensions,
    UpdateUserExtensions,
    // Chat
    GetChatters,
    GetChannelEmotes,
    GetGlobalEmotes,
    GetEmoteSets,
    GetChannelChatBadges,
    GetGlobalChatBadges,
    GetChatSettings,
    GetShardChatSession,
    GetUserEmotes,
    UpdateChatSettings,
    SendChatAnnouncement,
    SendAShoutout,
    SendChatMessage,
    GetUserChatColor,
    UpdateUserChatColor,
    // EventSub
    DeleteEventSub,
    CreateEventSub,
    GetEventSub,
    ChannelRaid,
}

impl EndpointType {
    pub fn required_scopes(&self) -> Option<Vec<Scope>> {
        match self {
            // Ads
            Self::StartCommercial => Some(vec![Scope::ChannelEditCommercial]),
            Self::GetAdSchedule => Some(vec![Scope::ChannelReadAds]),
            Self::SnoozeNextAd => Some(vec![Scope::ChannelManageAds]),
            // Analytics
            Self::GetExtensionAnalytics => Some(vec![Scope::AnalyticsReadExtensions]),
            Self::GetGameAnalytics => Some(vec![Scope::AnalyticsReadGames]),
            // Bits
            Self::GetBitsLeaderboard => Some(vec![Scope::BitsRead]),
            Self::GetCheermotes => None,
            Self::GetExtensionTransactions => None,
            // Channel
            Self::GetChanelInformation => None,
            Self::ModifyChannelInformation => Some(vec![Scope::ChannelManageBroadcast]),
            Self::GetChannelEditors => Some(vec![Scope::ChannelReadEditors]),
            Self::GetFollowedChannels => Some(vec![Scope::UserReadFollows]),
            Self::GetChannelFollowers => Some(vec![Scope::ModeratorReadFollowers]),
            // Users
            Self::GetUsers => Some(vec![Scope::UserReadEmail]),
            Self::UpdateUser => Some(vec![Scope::UserReadEmail, Scope::UserEdit]),
            Self::GetUserBlockList => Some(vec![Scope::UserReadBlockedUsers]),
            Self::BlockUser => Some(vec![Scope::UserManageBlockedUsers]),
            Self::UnblockUser => Some(vec![Scope::UserManageBlockedUsers]),
            Self::GetUserExtensions => {
                Some(vec![Scope::UserReadBroadcast, Scope::UserEditBroadcast])
            }
            Self::GetUserActiveExtensions => {
                Some(vec![Scope::UserReadBroadcast, Scope::UserEditBroadcast])
            }
            Self::UpdateUserExtensions => Some(vec![Scope::UserEditBroadcast]),
            // Chat
            Self::GetChatters => Some(vec![Scope::ModeratorReadChatters]),
            Self::GetChannelEmotes => None,
            Self::GetGlobalEmotes => None,
            Self::GetEmoteSets => None,
            Self::GetChannelChatBadges => None,
            Self::GetGlobalChatBadges => None,
            Self::GetChatSettings => None,
            Self::GetShardChatSession => None,
            Self::GetUserEmotes => Some(vec![Scope::UserReadEmotes]),
            Self::UpdateChatSettings => Some(vec![Scope::ModeratorManageChatSettings]),
            Self::SendChatAnnouncement => Some(vec![Scope::ModeratorManageAnnouncements]),
            Self::SendAShoutout => None,
            Self::SendChatMessage => Some(vec![
                Scope::UserWriteChat,
                Scope::UserBot,
                Scope::ChannelBot,
            ]),
            Self::GetUserChatColor => None,
            Self::UpdateUserChatColor => Some(vec![Scope::UserManageChatColor]),
            // EventSub
            Self::DeleteEventSub => None,
            Self::CreateEventSub => None,
            Self::GetEventSub => None,
            Self::ChannelRaid => None,
        }
    }

    pub fn token_type(&self) -> TokenType {
        match self {
            // Ads
            Self::StartCommercial => TokenType::User,
            Self::GetAdSchedule => TokenType::User,
            Self::SnoozeNextAd => TokenType::User,
            // Analytics
            Self::GetExtensionAnalytics => TokenType::User,
            Self::GetGameAnalytics => TokenType::User,
            // Bits
            Self::GetBitsLeaderboard => TokenType::User,
            Self::GetCheermotes => TokenType::Any,
            Self::GetExtensionTransactions => TokenType::App,
            // Channel
            Self::GetChanelInformation => TokenType::Any,
            Self::ModifyChannelInformation => TokenType::User,
            Self::GetChannelEditors => TokenType::User,
            Self::GetFollowedChannels => TokenType::User,
            Self::GetChannelFollowers => TokenType::User,
            // Users
            Self::GetUsers => TokenType::Any,
            Self::UpdateUser => TokenType::User,
            Self::GetUserBlockList => TokenType::User,
            Self::BlockUser => TokenType::User,
            Self::UnblockUser => TokenType::User,
            Self::GetUserExtensions => TokenType::User,
            Self::GetUserActiveExtensions => TokenType::Any,
            Self::UpdateUserExtensions => TokenType::User,
            // Chat
            Self::GetChatters => TokenType::User,
            Self::GetChannelEmotes => TokenType::Any,
            Self::GetGlobalEmotes => TokenType::Any,
            Self::GetEmoteSets => TokenType::Any,
            Self::GetChannelChatBadges => TokenType::Any,
            Self::GetGlobalChatBadges => TokenType::Any,
            Self::GetChatSettings => TokenType::Any,
            Self::GetShardChatSession => TokenType::Any,
            Self::GetUserEmotes => TokenType::User,
            Self::UpdateChatSettings => TokenType::User,
            Self::SendChatAnnouncement => TokenType::User,
            Self::SendAShoutout => TokenType::User,
            Self::SendChatMessage => TokenType::Any,
            Self::GetUserChatColor => TokenType::Any,
            Self::UpdateUserChatColor => TokenType::User,
            // EventSub
            Self::CreateEventSub => TokenType::App,
            Self::DeleteEventSub => TokenType::App,
            Self::GetEventSub => TokenType::App,
            Self::ChannelRaid => TokenType::App,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    User,
    App,
    Any,
}
