use asknothingx2_util::serde::{deserialize_empty_object_as_none, serialize_none_as_empty_object};
use serde::{Deserialize, Serialize};

use crate::types::Pagination;

use super::types::{ChannelEditor, ChannelFollower, ChannelInfo, FollowedChannel};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelInfoResponse {
    pub data: Vec<ChannelInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelEditorsResponse {
    pub data: Vec<ChannelEditor>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FollowerdChannelsResponse {
    pub data: Option<Vec<FollowedChannel>>,
    pub total: u64,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelFollowersResponse {
    pub data: Option<Vec<ChannelFollower>>,
    pub total: u64,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
}

#[cfg(test)]
mod tests {
    use chrono::Timelike;
    use serde_json::json;

    use crate::channels::response::{
        ChannelEditorsResponse, ChannelFollowersResponse, ChannelInfoResponse,
        FollowerdChannelsResponse,
    };

    #[test]
    fn channel_info_response_deserialization() {
        let json_data = json!({
            "data": [
                {
                    "broadcaster_id": "123456789",
                    "broadcaster_login": "teststreamer",
                    "broadcaster_name": "TestStreamer",
                    "broadcaster_language": "en",
                    "game_id": "509658",
                    "game_name": "Just Chatting",
                    "title": "Testing My Stream Setup",
                    "delay": 0,
                    "tags": ["English", "Gaming", "Casual"],
                    "content_classification_labels": ["ProfanityVulgarity"],
                    "is_branded_content": false
                },
                {
                    "broadcaster_id": "987654321",
                    "broadcaster_login": "anotherstreamer",
                    "broadcaster_name": "AnotherStreamer",
                    "broadcaster_language": "es",
                    "game_id": "32982",
                    "game_name": "Grand Theft Auto V",
                    "title": "Jugando GTA V con amigos",
                    "delay": 30,
                    "tags": ["Spanish", "Action", "Multiplayer"],
                    "content_classification_labels": ["ViolentGraphic", "ProfanityVulgarity"],
                    "is_branded_content": true
                }
            ]
        });

        let response: ChannelInfoResponse = serde_json::from_value(json_data).unwrap();

        assert_eq!(response.data.len(), 2);

        let first_channel = &response.data[0];
        assert_eq!(first_channel.broadcaster_id.as_str(), "123456789");
        assert_eq!(first_channel.broadcaster_login, "teststreamer");
        assert_eq!(first_channel.broadcaster_name, "TestStreamer");
        assert_eq!(first_channel.broadcaster_language, "en");
        assert_eq!(first_channel.game_id.as_str(), "509658");
        assert_eq!(first_channel.game_name, "Just Chatting");
        assert_eq!(first_channel.title, "Testing My Stream Setup");
        assert_eq!(first_channel.delay, 0);
        assert_eq!(first_channel.tags, vec!["English", "Gaming", "Casual"]);
        assert_eq!(
            first_channel.content_classification_labels,
            vec!["ProfanityVulgarity"]
        );
        assert!(!first_channel.is_branded_content);

        let second_channel = &response.data[1];
        assert_eq!(second_channel.broadcaster_id.as_str(), "987654321");
        assert_eq!(second_channel.broadcaster_language, "es");
        assert_eq!(second_channel.game_name, "Grand Theft Auto V");
        assert_eq!(second_channel.delay, 30);
        assert!(second_channel.is_branded_content);
        assert_eq!(second_channel.content_classification_labels.len(), 2);
        assert!(second_channel
            .content_classification_labels
            .contains(&"ViolentGraphic".to_string()));
        assert!(second_channel
            .content_classification_labels
            .contains(&"ProfanityVulgarity".to_string()));
    }

    #[test]
    fn channel_editors_response_deserialization() {
        let json_data = json!({
            "data": [
                {
                    "user_id": "editor123",
                    "user_name": "ChannelEditor",
                    "created_at": "2023-12-01T15:30:00Z"
                },
                {
                    "user_id": "editor456",
                    "user_name": "AnotherEditor",
                    "created_at": "2023-11-15T10:20:00Z"
                }
            ]
        });

        let response: ChannelEditorsResponse = serde_json::from_value(json_data).unwrap();

        assert_eq!(response.data.len(), 2);

        let first_editor = &response.data[0];
        assert_eq!(first_editor.user_id.as_str(), "editor123");
        assert_eq!(first_editor.user_name, "ChannelEditor");
        assert_eq!(first_editor.created_at.hour(), 15);
        assert_eq!(first_editor.created_at.minute(), 30);

        let second_editor = &response.data[1];
        assert_eq!(second_editor.user_id.as_str(), "editor456");
        assert_eq!(second_editor.user_name, "AnotherEditor");
    }

    #[test]
    fn followed_channels_response_deserialization() {
        let json_data = json!({
            "data": [
                {
                    "broadcaster_id": "123456789",
                    "broadcaster_login": "followedstreamer",
                    "broadcaster_name": "FollowedStreamer",
                    "followed_at": "2023-12-01T15:30:00Z"
                }
            ],
            "total": 1,
            "pagination": {
                "cursor": "eyJiI..."
            }
        });

        let response: FollowerdChannelsResponse = serde_json::from_value(json_data).unwrap();

        assert!(response.data.is_some());
        let followed_channels = response.data.unwrap();
        assert_eq!(followed_channels.len(), 1);
        assert_eq!(response.total, 1);
        assert!(response.pagination.is_some());

        let followed_channel = &followed_channels[0];
        assert_eq!(followed_channel.broadcaster_id.as_str(), "123456789");
        assert_eq!(followed_channel.broadcaster_login, "followedstreamer");
        assert_eq!(followed_channel.broadcaster_name, "FollowedStreamer");
        assert_eq!(followed_channel.followed_at.hour(), 15);
    }

    #[test]
    fn channel_followers_response_deserialization() {
        let json_data = json!({
            "data": [
                {
                    "user_id": "follower123",
                    "user_login": "testfollower",
                    "user_name": "TestFollower",
                    "followed_at": "2023-12-01T15:30:00Z"
                },
                {
                    "user_id": "follower456",
                    "user_login": "anotherfollower",
                    "user_name": "AnotherFollower",
                    "followed_at": "2023-11-30T12:00:00Z"
                }
            ],
            "total": 2,
            "pagination": {}
        });

        let response: ChannelFollowersResponse = serde_json::from_value(json_data).unwrap();

        assert!(response.data.is_some());
        let followers = response.data.unwrap();
        assert_eq!(followers.len(), 2);
        assert_eq!(response.total, 2);
        assert!(response.pagination.is_none());

        let first_follower = &followers[0];
        assert_eq!(first_follower.user_id.as_str(), "follower123");
        assert_eq!(first_follower.user_login, "testfollower");
        assert_eq!(first_follower.user_name, "TestFollower");

        let second_follower = &followers[1];
        assert_eq!(second_follower.user_id.as_str(), "follower456");
        assert_eq!(second_follower.followed_at.hour(), 12);
    }

    #[test]
    fn empty_responses() {
        let followed_null_data = json!({
            "data": null,
            "total": 0,
            "pagination": {}
        });

        let response: FollowerdChannelsResponse =
            serde_json::from_value(followed_null_data).unwrap();
        assert!(response.data.is_none());
        assert_eq!(response.total, 0);

        let followers_empty = json!({
            "data": [],
            "total": 0
        });

        let response: ChannelFollowersResponse = serde_json::from_value(followers_empty).unwrap();
        assert!(response.data.is_some());
        assert_eq!(response.data.unwrap().len(), 0);
        assert_eq!(response.total, 0);
    }
}
