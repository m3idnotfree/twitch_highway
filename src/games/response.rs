use asknothingx2_util::serde::{deserialize_empty_object_as_none, serialize_none_as_empty_object};
use serde::{Deserialize, Serialize};

use crate::types::{Category, Pagination};

#[derive(Debug, Serialize, Deserialize)]
pub struct GamesResponse {
    pub data: Vec<Category>,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::games::response::GamesResponse;

    #[test]
    fn games_response_deserialization() {
        let json_data = json!({
            "data": [
                {
                    "id": "21779",
                    "name": "League of Legends",
                    "box_art_url": "https://static-cdn.jtvnw.net/ttv-boxart/21779-{width}x{height}.jpg",
                    "igdb_id": "115"
                },
                {
                    "id": "509658",
                    "name": "Just Chatting",
                    "box_art_url": "https://static-cdn.jtvnw.net/ttv-boxart/509658-{width}x{height}.jpg"
                },
                {
                    "id": "516575",
                    "name": "VALORANT",
                    "box_art_url": "https://static-cdn.jtvnw.net/ttv-boxart/516575-{width}x{height}.jpg",
                    "igdb_id": "25646"
                }
            ],
            "pagination": {
                "cursor": "eyJiI..."
            }
        });

        let response: GamesResponse = serde_json::from_value(json_data).unwrap();

        assert_eq!(response.data.len(), 3);
        assert!(response.pagination.is_some());
        assert_eq!(response.pagination.unwrap().cursor, "eyJiI...");

        let lol = &response.data[0];
        assert_eq!(lol.id.as_str(), "21779");
        assert_eq!(lol.name, "League of Legends");
        assert!(lol.box_art_url.is_some());
        assert_eq!(lol.igdb_id.as_ref().unwrap(), "115");

        let just_chatting = &response.data[1];
        assert_eq!(just_chatting.id.as_str(), "509658");
        assert_eq!(just_chatting.name, "Just Chatting");
        assert!(just_chatting.box_art_url.is_some());
        assert!(just_chatting.igdb_id.is_none());

        let valorant = &response.data[2];
        assert_eq!(valorant.id.as_str(), "516575");
        assert_eq!(valorant.name, "VALORANT");
        assert!(valorant.box_art_url.is_some());
        assert_eq!(valorant.igdb_id.as_ref().unwrap(), "25646");
    }

    #[test]
    fn games_response_empty_data() {
        let json_data = json!({
            "data": []
        });

        let response: GamesResponse = serde_json::from_value(json_data).unwrap();
        assert_eq!(response.data.len(), 0);
        assert!(response.pagination.is_none());
    }

    #[test]
    fn games_response_empty_pagination() {
        let json_data = json!({
            "data": [
                {
                    "id": "single_game",
                    "name": "Single Game",
                    "box_art_url": "https://static-cdn.jtvnw.net/ttv-boxart/single_game-{width}x{height}.jpg"
                }
            ],
            "pagination": {}
        });

        let response: GamesResponse = serde_json::from_value(json_data).unwrap();
        assert_eq!(response.data.len(), 1);
        assert!(response.pagination.is_none());
    }
}
