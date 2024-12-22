mod channel_badges;
pub use channel_badges::*;
mod global_badges;
pub use global_badges::*;

use asknothingx2_util::oauth::{AccessToken, ClientId};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct BadgeAPI {
    access_token: AccessToken,
    client_id: ClientId,
}

impl BadgeAPI {
    pub fn new(access_token: AccessToken, client_id: ClientId) -> Self {
        Self {
            access_token,
            client_id,
        }
    }

    pub fn get_channel<T: Into<String>>(&self, broadcaster_id: T) -> GetChannelBadge {
        GetChannelBadge::new(
            self.access_token.clone(),
            self.client_id.clone(),
            broadcaster_id.into(),
        )
    }

    pub fn get_global(&self) -> GetGlobalBadges {
        GetGlobalBadges::new(self.access_token.clone(), self.client_id.clone())
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BadgeResponse {
    pub data: Vec<Badges>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Badges {
    pub set_id: String,
    pub versions: Vec<Versions>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Badge {
    pub set_id: String,
    pub versions: Versions,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Versions {
    pub id: String,
    pub image_url_1x: String,
    pub image_url_2x: String,
    pub image_url_4x: String,
    pub title: String,
    pub description: String,
    pub click_action: Option<String>,
    pub click_url: Option<String>,
}

#[cfg(test)]
mod tests {
    use crate::{badges::BadgeResponse, expect_response_json};

    #[test]
    fn badge_response() {
        expect_response_json!("{\n  \"data\": [\n    {\n      \"set_id\": \"bits\",\n      \"versions\": [\n        {\n          \"id\": \"1\",\n          \"image_url_1x\": \"https://static-cdn.jtvnw.net/badges/v1/743a0f3b-84b3-450b-96a0-503d7f4a9764/1\",\n          \"image_url_2x\": \"https://static-cdn.jtvnw.net/badges/v1/743a0f3b-84b3-450b-96a0-503d7f4a9764/2\",\n          \"image_url_4x\": \"https://static-cdn.jtvnw.net/badges/v1/743a0f3b-84b3-450b-96a0-503d7f4a9764/3\",\n          \"title\": \"cheer 1\",\n          \"description\": \"cheer 1\",\n          \"click_action\": \"visit_url\",\n          \"click_url\": \"https://bits.twitch.tv\"\n        }\n      ]\n    },\n    {\n      \"set_id\": \"subscriber\",\n      \"versions\": [\n        {\n          \"id\": \"0\",\n          \"image_url_1x\": \"https://static-cdn.jtvnw.net/badges/v1/eb4a8a4c-eacd-4f5e-b9f2-394348310442/1\",\n          \"image_url_2x\": \"https://static-cdn.jtvnw.net/badges/v1/eb4a8a4c-eacd-4f5e-b9f2-394348310442/2\",\n          \"image_url_4x\": \"https://static-cdn.jtvnw.net/badges/v1/eb4a8a4c-eacd-4f5e-b9f2-394348310442/3\",\n          \"title\": \"Subscriber\",\n          \"description\": \"Subscriber\",\n          \"click_action\": \"subscribe_to_channel\",\n          \"click_url\": null\n        }\n      ]\n    }\n  ]\n}",
            BadgeResponse);
    }
}
