use serde::Serialize;

use crate::types::GameId;

request_struct!(
    #[derive(Serialize)]
    ModifyChannelRequest {
        string {
            #[serde(skip_serializing_if = "Option::is_none")]
            broadcaster_language: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            title: String
        },
        any {
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
    };
    impl_body: true
);

request_struct!(
    #[derive(Serialize)]
    ContentClassificationLabel {
        required {
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
