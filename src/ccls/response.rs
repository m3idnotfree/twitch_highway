use serde::{Deserialize, Serialize};

use super::types::Ccls;

#[derive(Debug, Serialize, Deserialize)]
pub struct CclsResponse {
    pub data: Vec<Ccls>,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::ccls::response::CclsResponse;

    #[test]
    fn ccls_response_deserialization() {
        let json_data = json!({
            "data": [
                {
                    "id": "DrugsIntoxication",
                    "description": "Excessive tobacco glorification, any marijuana consumption/use, legal drug and alcohol induced intoxication, discussions of illegal drugs.",
                    "name": "Drugs, Intoxication, or Excessive Tobacco Use"
                },
                {
                    "id": "SexualThemes",
                    "description": "Content that focuses on sexualized physical attributes and activities, sexual topics, or experiences.",
                    "name": "Sexual Themes"
                },
                {
                    "id": "ViolentGraphic",
                    "description": "Simulated or real violence with sustained visceral content including mutilation, gore, or death.",
                    "name": "Violent and Graphic Depictions"
                },
                {
                    "id": "Gambling",
                    "description": "Games/activities that utilize or simulate gambling/betting with real or virtual currencies or items.",
                    "name": "Gambling"
                },
                {
                    "id": "ProfanityVulgarity",
                    "description": "Frequent use of profanity or vulgar language.",
                    "name": "Significant Profanity or Vulgarity"
                }
            ]
        });

        let response: CclsResponse = serde_json::from_value(json_data).unwrap();

        assert_eq!(response.data.len(), 5);

        let drugs_label = &response.data[0];
        assert_eq!(drugs_label.id.as_str(), "DrugsIntoxication");
        assert_eq!(
            drugs_label.name,
            "Drugs, Intoxication, or Excessive Tobacco Use"
        );
        assert!(drugs_label.description.contains("marijuana consumption"));

        let sexual_themes = &response.data[1];
        assert_eq!(sexual_themes.id.as_str(), "SexualThemes");
        assert_eq!(sexual_themes.name, "Sexual Themes");
        assert!(sexual_themes
            .description
            .contains("sexualized physical attributes"));

        let gambling = response
            .data
            .iter()
            .find(|label| label.id.as_str() == "Gambling")
            .unwrap();
        assert_eq!(gambling.name, "Gambling");
        assert!(gambling.description.contains("betting"));
    }

    #[test]
    fn ccls_response_empty_data() {
        let json_data = json!({
            "data": []
        });

        let response: CclsResponse = serde_json::from_value(json_data).unwrap();
        assert_eq!(response.data.len(), 0);
    }
}
