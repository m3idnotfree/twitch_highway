use serde::Serialize;

use crate::{
    base::{IntoQueryPairs, QueryParams},
    types::{BroadcasterId, UserId, AFTER, BROADCASTER_ID, FIRST, USER_ID},
    AsBody,
};

request_struct!(
    #[derive(Debug, Default, Serialize)]
    ModifyChannelRequest {
        #[serde(skip_serializing_if = "Option::is_none")]
        game_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        broadcaster_language: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        delay: u64,
        #[serde(skip_serializing_if = "Option::is_none")]
        tags:Vec<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        content_classification_labels:Vec<ContentClassificationLabel>,
        #[serde(skip_serializing_if = "Option::is_none")]
        is_branded_content: bool
    }
);

impl AsBody for ModifyChannelRequest {
    fn as_body(&self) -> Option<String> {
        Some(serde_json::to_string(&self).unwrap())
    }
}
#[derive(Debug, Serialize)]
pub struct ContentClassificationLabel {
    id: ContentClassificationLabelsID,
    is_enabled: bool,
}

impl ContentClassificationLabel {
    pub fn new(id: ContentClassificationLabelsID, is_enabled: bool) -> Self {
        Self { id, is_enabled }
    }

    //pub fn extend<L: IntoIterator<Item = (ContentClassificationLabelsID, bool)>>(label: L) -> Self {}
}
#[derive(Debug, Serialize)]
pub enum ContentClassificationLabelsID {
    DebatedSocialIssuesAndPolitics,
    DrugsIntoxication,
    SexualThemes,
    ViolentGraphic,
    Gambling,
    ProfanityVulgarity,
}

request_struct!(
    #[derive(Debug, Serialize)]
    ChannelFollowRequest {
        required {
            user_id: UserId
        },
        optional {
            broadcaster_id: BroadcasterId,
            first: u64,
            after: String,
        }
    }
);

impl IntoQueryPairs for ChannelFollowRequest {
    fn into_query_pairs(self) -> Vec<(&'static str, String)> {
        let mut params = QueryParams::new();
        params
            .push(USER_ID, self.user_id)
            .push_opt(BROADCASTER_ID, self.broadcaster_id)
            .push_opt(FIRST, self.first.map(|x| x.to_string()))
            .push_opt(AFTER, self.after);

        params.build()
    }
}
