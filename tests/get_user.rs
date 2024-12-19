use std::{fs::File, io::BufReader};

use asknothingx2_util::{
    api::api_request,
    oauth::{AccessToken, ClientId},
};
use serde::{Deserialize, Serialize};
use twitch_highway::{BroadcasterType, GetUsersResponse, UserAPI};
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
struct TestToken {
    user_id: String,
    client_id: ClientId,
    secret: String,
    name: String,
    token: AccessToken,
    url: Url,
}

fn get_token() -> TestToken {
    let file = File::open("./tests/test_token.json").unwrap();
    let reader = BufReader::new(file);

    serde_json::from_reader(reader).unwrap()
}

#[cfg(feature = "test")]
#[tokio::test]
async fn base() {
    use twitch_highway::{GetUserType, TestUrl};

    let token = get_token();

    let user_api = UserAPI::new(&token.token, &token.client_id);
    let mut get_user = user_api.get_users().add_id(token.user_id).unwrap();
    let user_url = "http://localhost:8080/mock/users";

    get_user.with_url(user_url);
    let response = api_request(get_user).await;
    assert!(response.is_ok());
    let get_user_response: GetUsersResponse = response.unwrap().json().await.unwrap();
    let first_user = get_user_response.data.first().unwrap();
    assert_eq!(first_user.id, "35249427");
    assert_eq!(first_user.kind, GetUserType::Normal);
    assert_eq!(first_user.broadcaster_type, BroadcasterType::Partner);

    let de = serde_json::to_string(first_user).unwrap();
    assert!(de.contains("\"type\":\"\""));
    assert!(de.contains("\"broadcaster_type\":\"partner\""));
}
