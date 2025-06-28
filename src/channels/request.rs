use serde::Serialize;

use crate::types::GameId;

define_request!(
    #[derive(Default, Serialize)]
    ModifyChannelRequest<'a> {
        opts: {
            #[serde(skip_serializing_if = "Option::is_none")]
            broadcaster_language: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            title: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            game_id: GameId,
            #[serde(skip_serializing_if = "Option::is_none")]
            delay: u64,
            #[serde(skip_serializing_if = "Option::is_none")]
            tags: &'a [&'a str],
            #[serde(skip_serializing_if = "Option::is_none")]
            content_classification_labels: &'a [&'a ContentClassificationLabel],
            #[serde(skip_serializing_if = "Option::is_none")]
            is_branded_content: bool
        };
        to_json
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
