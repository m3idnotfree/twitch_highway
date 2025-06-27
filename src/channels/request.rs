use serde::Serialize;

use crate::types::GameId;

define_request!(
    #[derive(Default, Serialize)]
    ModifyChannelRequest {
        opts: {
            #[serde(skip_serializing_if = "Option::is_none")]
            broadcaster_language: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            title: String,
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
        };
        into_request_body
    }
);

define_request!(
    #[derive(Serialize)]
    ContentClassificationLabel {
        req: {
            id: ContentClassificationLabelsID,
            is_enabled: bool,
       }
    }
);

#[derive(Debug, Serialize)]
pub enum ContentClassificationLabelsID {
    DebatedSocialIssuesAndPolitics,
    DrugsIntoxication,
    SexualThemes,
    ViolentGraphic,
    Gambling,
    ProfanityVulgarity,
}
