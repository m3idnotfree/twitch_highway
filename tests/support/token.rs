#![allow(dead_code)]
use asknothingx2_util::oauth::{ClientId, ClientSecret};
use twitch_highway::APIError;
use twitch_oauth_token::{
    test_url::User,
    types::{Scope, Token},
    TwitchOauth,
};

pub async fn user_token<T: IntoIterator<Item = Scope>>(
    scopes: T,
) -> Result<(Token, ClientId, ClientSecret, String), twitch_highway::Error> {
    use dotenv::dotenv;

    dotenv().ok();

    let (oauth, client_id, cliend_secret, user_id) = oauth_init().await;
    let mut user_token = oauth.user_token(user_id.clone());
    user_token.scopes_mut().extend(scopes);

    let user_token = user_token
        .request_access_token()
        .await
        .expect("Failed to request user access token from mock server");
    if !user_token.is_success() {
        let error_token: APIError = user_token
            .into_json()
            .map_err(|x| twitch_highway::Error::DeserializationError(x.to_string()))?;
        eprint!("{:#?}", error_token);
        return Err(twitch_highway::Error::TwitchAPIError(error_token));
    }

    Ok((
        user_token.into_json::<Token>().unwrap(),
        client_id,
        cliend_secret,
        user_id,
    ))
}

#[allow(dead_code)]
pub async fn app_token<T: IntoIterator<Item = Scope>>(
    scopes: T,
) -> Result<(Token, ClientId, ClientSecret, String), twitch_highway::Error> {
    use dotenv::dotenv;

    dotenv().ok();

    let (oauth, client_id, cliend_secret, user_id) = oauth_init().await;
    let mut app_token = oauth.app_token();
    app_token.scopes_mut().extend(scopes);

    let app_token = app_token
        .request_access_token()
        .await
        .expect("Failed to request app access token from mock server");

    if !app_token.is_success() {
        let error_token: APIError = app_token
            .into_json()
            .map_err(|x| twitch_highway::Error::DeserializationError(x.to_string()))?;
        eprint!("{:#?}", error_token);
        return Err(twitch_highway::Error::TwitchAPIError(error_token));
    }

    Ok((
        app_token.into_json::<Token>().unwrap(),
        client_id,
        cliend_secret,
        user_id,
    ))
}

async fn oauth_init() -> (TwitchOauth, ClientId, ClientSecret, String) {
    use dotenv::dotenv;
    use twitch_oauth_token::{test_url::get_users_info, TwitchOauth};

    dotenv().ok();

    let user_id = std::env::var("USER_ID").expect("USER_ID environment variable is not set");
    let users_info = get_users_info(None)
        .await
        .expect("Failed to connect to Twitch mock server");
    println!("{:?}", users_info);
    let user = users_info
        .data
        .first()
        .expect("Mock server returned empty user data");

    let test_oauth = TwitchOauth::from_credentials(user.ID.clone(), user.Secret.clone(), None)
        .expect("Failed to initialize TwitchOAuth with mock credentials")
        .with_url(None);

    (test_oauth, user.ID.clone(), user.Secret.clone(), user_id)
}
