use asknothingx2_util::api::{APIRequest, HeaderMap, Method};
use twitch_oauth_token::types::Scope;
use url::Url;

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
