use asknothingx2_util::serde::{deserialize_empty_object_as_none, serialize_none_as_empty_object};
use serde::{Deserialize, Serialize};

use crate::types::Pagination;

use super::types::{Clip, CreateClip};

#[derive(Debug, Deserialize)]
pub struct CreateClipsResponse {
    pub data: Vec<CreateClip>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClipsInfoResponse {
    pub data: Vec<Clip>,
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

    use crate::clips::response::{ClipsInfoResponse, CreateClipsResponse};

    #[test]
    fn create_clips_response_deserialization() {
        let json_data = json!({
            "data": [
                {
                    "edit_url": "https://clips.twitch.tv/edit/AwesomeClip123",
                    "id": "AwesomeClip123"
                }
            ]
        });

        let response: CreateClipsResponse = serde_json::from_value(json_data).unwrap();

        assert_eq!(response.data.len(), 1);

        let created_clip = &response.data[0];
        assert_eq!(created_clip.id.as_str(), "AwesomeClip123");
        assert!(created_clip.edit_url.contains("AwesomeClip123"));
        assert!(created_clip
            .edit_url
            .starts_with("https://clips.twitch.tv/edit/"));
    }

    #[test]
    fn clips_info_response_deserialization() {
        let json_data = json!({
            "data": [
                {
                    "id": "RandomClip456",
                    "url": "https://clips.twitch.tv/RandomClip456",
                    "embed_url": "https://clips.twitch.tv/embed?clip=RandomClip456",
                    "broadcaster_id": "123456789",
                    "broadcaster_name": "TestStreamer",
                    "creator_id": "987654321",
                    "creator_name": "ClipCreator",
                    "video_id": "v123456789",
                    "game_id": "509658",
                    "language": "en",
                    "title": "Amazing Play!",
                    "view_count": 1500,
                    "created_at": "2024-01-15T10:30:00Z",
                    "thumbnail_url": "https://clips-media-assets2.twitch.tv/thumbnail.jpg",
                    "duration": 30.5,
                    "vod_offset": 3600,
                    "is_featured": false
                },
                {
                    "id": "AnotherClip789",
                    "url": "https://clips.twitch.tv/AnotherClip789",
                    "embed_url": "https://clips.twitch.tv/embed?clip=AnotherClip789",
                    "broadcaster_id": "123456789",
                    "broadcaster_name": "TestStreamer",
                    "creator_id": "111222333",
                    "creator_name": "AnotherCreator",
                    "video_id": "v987654321",
                    "game_id": "21779",
                    "language": "en",
                    "title": "Epic Moment",
                    "view_count": 2500,
                    "created_at": "2024-01-16T15:45:00Z",
                    "thumbnail_url": "https://clips-media-assets2.twitch.tv/another_thumbnail.jpg",
                    "duration": 45.2,
                    "vod_offset": null,
                    "is_featured": true
                }
            ],
            "pagination": {
                "cursor": "eyJiI..."
            }
        });

        let response: ClipsInfoResponse = serde_json::from_value(json_data).unwrap();

        assert_eq!(response.data.len(), 2);
        assert!(response.pagination.is_some());
        assert_eq!(response.pagination.unwrap().cursor, "eyJiI...");

        let first_clip = &response.data[0];
        assert_eq!(first_clip.id.as_str(), "RandomClip456");
        assert_eq!(first_clip.broadcaster_id.as_str(), "123456789");
        assert_eq!(first_clip.broadcaster_name, "TestStreamer");
        assert_eq!(first_clip.creator_name, "ClipCreator");
        assert_eq!(first_clip.title, "Amazing Play!");
        assert_eq!(first_clip.view_count, 1500);
        assert_eq!(first_clip.duration, 30.5);
        assert_eq!(first_clip.vod_offset, Some(3600));
        assert!(!first_clip.is_featured);

        let second_clip = &response.data[1];
        assert_eq!(second_clip.id.as_str(), "AnotherClip789");
        assert_eq!(second_clip.creator_name, "AnotherCreator");
        assert_eq!(second_clip.title, "Epic Moment");
        assert_eq!(second_clip.view_count, 2500);
        assert_eq!(second_clip.duration, 45.2);
        assert_eq!(second_clip.vod_offset, None);
        assert!(second_clip.is_featured);
    }

    #[test]
    fn clips_response_empty_data() {
        let json_data = json!({
            "data": []
        });

        let response: ClipsInfoResponse = serde_json::from_value(json_data).unwrap();
        assert_eq!(response.data.len(), 0);
        assert!(response.pagination.is_none());
    }

    #[test]
    fn clips_response_empty_pagination() {
        let json_data = json!({
            "data": [
                {
                    "id": "SingleClip123",
                    "url": "https://clips.twitch.tv/SingleClip123",
                    "embed_url": "https://clips.twitch.tv/embed?clip=SingleClip123",
                    "broadcaster_id": "123456789",
                    "broadcaster_name": "SoloStreamer",
                    "creator_id": "111111111",
                    "creator_name": "OnlyCreator",
                    "video_id": "v333333333",
                    "game_id": "509658",
                    "language": "en",
                    "title": "Solo Play",
                    "view_count": 100,
                    "created_at": "2024-01-17T12:00:00Z",
                    "thumbnail_url": "https://clips-media-assets2.twitch.tv/solo_clip.jpg",
                    "duration": 15.0,
                    "vod_offset": 900,
                    "is_featured": false
                }
            ],
            "pagination": {}
        });

        let response: ClipsInfoResponse = serde_json::from_value(json_data).unwrap();
        assert_eq!(response.data.len(), 1);
        assert!(response.pagination.is_none());
    }
}
