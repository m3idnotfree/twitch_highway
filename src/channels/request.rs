use serde::Serialize;

use crate::{types::GameId, IntoRequestBody};

new_request_struct!(
    #[derive(Serialize)]
    ModifyChannelRequest {
        string: {
            #[serde(skip_serializing_if = "Option::is_none")]
            broadcaster_language: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            title: String
        },
        any: {
            #[serde(skip_serializing_if = "Option::is_none")]
            game_id: GameId,
            #[serde(skip_serializing_if = "Option::is_none")]
            delay: u64,
            #[serde(skip_serializing_if = "Option::is_none")]
            tags:Vec<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            content_classification_labels:Vec<ContentClassificationLabel>,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_branded_content: bool
        }
    }
);

impl IntoRequestBody for ModifyChannelRequest {
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
