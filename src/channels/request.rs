use serde::{Deserialize, Serialize};

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
        into_json
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

#[derive(Debug, Serialize, Deserialize)]
pub enum ContentClassificationLabelsID {
    DebatedSocialIssuesAndPolitics,
    DrugsIntoxication,
    SexualThemes,
    ViolentGraphic,
    Gambling,
    ProfanityVulgarity,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::{
        channels::request::{
            ContentClassificationLabel, ContentClassificationLabelsID, ModifyChannelRequest,
        },
        types::GameId,
    };

    #[test]
    fn content_classification_labels_id_enum() {
        let labels = vec![
            ContentClassificationLabelsID::DebatedSocialIssuesAndPolitics,
            ContentClassificationLabelsID::DrugsIntoxication,
            ContentClassificationLabelsID::SexualThemes,
            ContentClassificationLabelsID::ViolentGraphic,
            ContentClassificationLabelsID::Gambling,
            ContentClassificationLabelsID::ProfanityVulgarity,
        ];

        for label in labels {
            let serialized = serde_json::to_string(&label).unwrap();

            let deserialized: ContentClassificationLabelsID =
                serde_json::from_str(&serialized).unwrap();
            let re_serialized = serde_json::to_string(&deserialized).unwrap();
            assert_eq!(serialized, re_serialized);
        }
    }

    #[test]
    fn content_classification_label_structure() {
        let label =
            ContentClassificationLabel::new(ContentClassificationLabelsID::SexualThemes, true);

        let serialized = serde_json::to_string(&label).unwrap();
        let expected = json!({
            "id": "SexualThemes",
            "is_enabled": true
        });

        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&serialized).unwrap(),
            expected
        );

        let label_disabled =
            ContentClassificationLabel::new(ContentClassificationLabelsID::Gambling, false);

        let serialized_disabled = serde_json::to_string(&label_disabled).unwrap();
        let expected_disabled = json!({
            "id": "Gambling",
            "is_enabled": false
        });

        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&serialized_disabled).unwrap(),
            expected_disabled
        );
    }

    #[test]
    fn modify_channel_request_all_fields() {
        let tags = vec!["Gaming", "English", "Competitive"];
        let classification_labels = [
            &ContentClassificationLabel::new(
                ContentClassificationLabelsID::ProfanityVulgarity,
                true,
            ),
            &ContentClassificationLabel::new(ContentClassificationLabelsID::ViolentGraphic, false),
        ];

        let request = ModifyChannelRequest::new()
            .broadcaster_language("en")
            .title("My Awesome Stream")
            .game_id(GameId::new("509658"))
            .delay(30)
            .tags(&tags)
            .content_classification_labels(&classification_labels)
            .is_branded_content(true);

        let json = request.into_json().unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["broadcaster_language"], "en");
        assert_eq!(parsed["title"], "My Awesome Stream");
        assert_eq!(parsed["game_id"], "509658");
        assert_eq!(parsed["delay"], 30);
        assert_eq!(parsed["tags"], json!(["Gaming", "English", "Competitive"]));
        assert_eq!(parsed["is_branded_content"], true);

        let labels_array = parsed["content_classification_labels"].as_array().unwrap();
        assert_eq!(labels_array.len(), 2);
        assert_eq!(labels_array[0]["id"], "ProfanityVulgarity");
        assert_eq!(labels_array[0]["is_enabled"], true);
        assert_eq!(labels_array[1]["id"], "ViolentGraphic");
        assert_eq!(labels_array[1]["is_enabled"], false);
    }
}
