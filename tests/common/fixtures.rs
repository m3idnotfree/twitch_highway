#![allow(dead_code)]

use asknothingx2_util::api::preset;
use rand::{rngs::ThreadRng, seq::IndexedRandom};
use twitch_highway::{
    types::{BroadcasterId, UserId},
    TwitchAPI,
};
use twitch_oauth_token::{
    scope::ScopesMut,
    test_oauth::{
        mock_api::MockApiUnits,
        response::{Client, User},
    },
    AppToken, TwitchOauth, UserToken,
};
use url::Url;

use crate::common::{get_stream_data, mock_data::MockDataError};

#[derive(Debug)]
pub struct TwitchFixture {
    pub api: TwitchAPI,
    pub mock_api: MockApiUnits,
    pub clients: Vec<Client>,
    pub users: Vec<User>,
    pub selected_user: User,
    pub selected_client: Client,
    pub user_token: Option<UserToken>,
    pub app_token: Option<AppToken>,
}

impl TwitchFixture {
    pub async fn user_access_token<F>(scope_fn: F) -> Result<Self, FixtureError>
    where
        F: FnOnce(&mut ScopesMut),
    {
        build_user_token(UserSelection::Random, scope_fn).await
    }

    pub async fn user_access_token_with_user_id<F>(
        user_id: &str,
        scope_fn: F,
    ) -> Result<Self, FixtureError>
    where
        F: FnOnce(&mut ScopesMut),
    {
        build_user_token(UserSelection::ById(user_id.to_string()), scope_fn).await
    }

    pub async fn user_access_token_with_partner<F>(scope_fn: F) -> Result<Self, FixtureError>
    where
        F: FnOnce(&mut ScopesMut),
    {
        build_user_token(UserSelection::Partner, scope_fn).await
    }

    pub async fn user_access_token_with_live<F>(scope_fn: F) -> Result<Self, FixtureError>
    where
        F: FnOnce(&mut ScopesMut),
    {
        build_user_token(UserSelection::Live, scope_fn).await
    }

    pub async fn app_access_token() -> Result<Self, FixtureError> {
        let mut rng = rand::rng();
        let (mock_api, clients, users) = try_init().await?;
        let selected_user = select_random_user(&users, &mut rng)?;
        let selected_client = select_random_client(&clients, &mut rng)?;

        let oauth = TwitchOauth::from_credentials(
            selected_client.ID.clone(),
            selected_client.Secret.clone(),
        )
        .with_test();

        let app_token = oauth.app_access_token().await?.app_token().await?;

        let api = TwitchAPI::with_client(
            app_token.access_token.clone(),
            selected_client.ID.clone(),
            preset::testing("twitch-highway-test/1.0")
                .build_client()
                .unwrap(),
        );

        Ok(Self {
            api,
            mock_api,
            clients,
            users,
            selected_user,
            selected_client,
            user_token: None,
            app_token: Some(app_token),
        })
    }

    pub fn get_random_user(&self) -> Result<User, FixtureError> {
        let mut rng = rand::rng();
        select_random_user(&self.users, &mut rng)
    }

    pub fn get_random_client(&self) -> Result<Client, FixtureError> {
        let mut rng = rand::rng();
        select_random_client(&self.clients, &mut rng)
    }

    pub fn selected_broadcaster_id(&self) -> BroadcasterId {
        BroadcasterId::from(self.selected_user.id.clone())
    }

    pub fn selected_user_id(&self) -> UserId {
        UserId::from(self.selected_user.id.clone())
    }

    #[cfg(any(
        feature = "chat",
        feature = "eventsub",
        feature = "guest-star",
        feature = "moderation"
    ))]
    pub fn selected_moderator_id(&self) -> twitch_highway::types::ModeratorId {
        twitch_highway::types::ModeratorId::from(self.selected_user.id.clone())
    }
}

async fn build_user_token<F>(
    user_selection: UserSelection,
    scope_fn: F,
) -> Result<TwitchFixture, FixtureError>
where
    F: FnOnce(&mut ScopesMut),
{
    let mut rng = rand::rng();
    let (mock_api, clients, users) = try_init().await?;

    let selected_user = match user_selection {
        UserSelection::Random => select_random_user(&users, &mut rng)?,
        UserSelection::Partner => select_partner_user(&users, &mut rng)?,
        UserSelection::ById(id) => users
            .iter()
            .find(|u| u.id == id)
            .cloned()
            .ok_or(FixtureError::UserNotFound(id.clone()))?,
        UserSelection::Live => {
            let stream_data = get_stream_data().await?;
            let selected_stream = stream_data
                .data
                .choose(&mut rng)
                .ok_or(FixtureError::NoLiveStreams)?;
            users
                .iter()
                .find(|u| u.id == selected_stream.user_id.as_str())
                .cloned()
                .ok_or(FixtureError::LiveUserNotFound(
                    selected_stream.user_id.to_string(),
                ))?
        }
    };

    let selected_client = select_random_client(&clients, &mut rng)?;

    let oauth =
        TwitchOauth::from_credentials(selected_client.ID.clone(), selected_client.Secret.clone())
            .with_test();

    let mut user_token_request = oauth.user_access_token(&selected_user.id);
    scope_fn(&mut user_token_request.scopes_mut());

    let user_token = user_token_request.send().await?.user_token().await?;

    let api = TwitchAPI::with_client(
        user_token.access_token.clone(),
        selected_client.ID.clone(),
        preset::testing("twitch-highway-test/1.0")
            .build_client()
            .unwrap(),
    )
    .set_url(Url::parse("http://localhost:8080/mock").unwrap());

    Ok(TwitchFixture {
        api,
        mock_api,
        clients,
        users,
        selected_user,
        selected_client,
        user_token: Some(user_token),
        app_token: None,
    })
}

enum UserSelection {
    Random,
    Partner,
    ById(String),
    Live,
}

pub async fn try_init() -> Result<(MockApiUnits, Vec<Client>, Vec<User>), FixtureError> {
    let mock_api = MockApiUnits::new();
    let clients = mock_api.get_clients().await?;
    let users = mock_api.get_users().await?;

    Ok((mock_api, clients.data, users.data))
}

pub fn select_random_user(users: &[User], rng: &mut ThreadRng) -> Result<User, FixtureError> {
    users.choose(rng).cloned().ok_or(FixtureError::NoUsers)
}

pub fn select_partner_user(users: &[User], rng: &mut ThreadRng) -> Result<User, FixtureError> {
    users
        .iter()
        .filter(|u| u.broadcaster_type == "affiliate" || u.broadcaster_type == "partner")
        .collect::<Vec<_>>()
        .choose(rng)
        .copied()
        .cloned()
        .ok_or(FixtureError::NoUsers)
}

pub fn select_random_client(
    clients: &[Client],
    rng: &mut ThreadRng,
) -> Result<Client, FixtureError> {
    clients.choose(rng).cloned().ok_or(FixtureError::NoClients)
}

#[derive(Debug, thiserror::Error)]
pub enum FixtureError {
    #[error("no users available in mock API")]
    NoUsers,
    #[error("user with id '{0}' not found")]
    UserNotFound(String),
    #[error("no clients available in mock API")]
    NoClients,
    #[error("oauth authentication failed: {0}")]
    OauthError(#[from] twitch_oauth_token::Error),

    #[error("no live streams available")]
    NoLiveStreams,
    #[error("live stream user with id '{0}' not found in mock users")]
    LiveUserNotFound(String),
    #[error("mock data error: {0}")]
    MockData(#[from] MockDataError),
}
