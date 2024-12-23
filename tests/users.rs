use twitch_highway::users::{GetUsers, GetUsersResponse};

mod support;

#[cfg(feature = "users")]
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

#[cfg(feature = "users")]
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

#[cfg(feature = "users")]
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

#[cfg(feature = "users")]
#[test]
fn get_users_login_max() {
    let get_user = api_general!(GetUsers);

    let get_user = get_user.add_login("twitchdev".to_string()).unwrap();
    let over_logins = vec!["twitchdev"; 100];
    let get_user = get_user.add_logins(over_logins);

    assert!(get_user.is_err());
}

#[cfg(feature = "users")]
#[test]
fn response_get_users() {
    expect_response_json!("{\n  \"data\": [\n    {\n      \"id\": \"141981764\",\n      \"login\": \"twitchdev\",\n      \"display_name\": \"TwitchDev\",\n      \"type\": \"\",\n      \"broadcaster_type\": \"partner\",\n      \"description\": \"Supporting third-party developers building Twitch integrations from chatbots to game integrations.\",\n      \"profile_image_url\": \"https://static-cdn.jtvnw.net/jtv_user_pictures/8a6381c7-d0c0-4576-b179-38bd5ce1d6af-profile_image-300x300.png\",\n      \"offline_image_url\": \"https://static-cdn.jtvnw.net/jtv_user_pictures/3f13ab61-ec78-4fe6-8481-8682cb3b0ac2-channel_offline_image-1920x1080.png\",\n      \"view_count\": 5980557,\n      \"email\": \"not-real@email.com\",\n      \"created_at\": \"2016-12-14T20:32:28Z\"\n    }\n  ]\n}",
            GetUsersResponse
        );
}
