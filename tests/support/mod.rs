use std::{fs::File, io::BufReader};

use asknothingx2_util::oauth::{AccessToken, ClientId};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct TestToken {
    pub user_id: String,
    pub client_id: ClientId,
    secret: String,
    name: String,
    pub token: AccessToken,
    pub url: Url,
}

pub fn get_token() -> TestToken {
    let file = File::open("./tests/test_token.json").unwrap();
    let reader = BufReader::new(file);

    serde_json::from_reader(reader).unwrap()
}
