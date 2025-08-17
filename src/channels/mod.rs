use asknothingx2_util::api::Method;
use request::ModifyChannelRequest;
use response::{
    ChannelEditorsResponse, ChannelFollowersResponse, ChannelInfoResponse,
    FollowerdChannelsResponse,
};

use crate::{
    request::{EndpointType, NoContent, TwitchAPIRequest},
    types::{
        constants::{BROADCASTER_ID, CHANNELS, USER_ID},
        BroadcasterId, PaginationQuery, UserId,
    },
    TwitchAPI,
};

pub mod request;
pub mod response;
pub mod types;

endpoints! {
    ChannelsAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#get-channel-information>
        fn get_channel_info(
            &self,
            broadcaster_ids: &[BroadcasterId],
        ) -> ChannelInfoResponse {
            endpoint_type: EndpointType::GetChanelInformation,
            method: Method::GET,
            path: [CHANNELS],
            query_params: {
                extend(broadcaster_ids.iter().map(|x| (BROADCASTER_ID, x)))
            }
        }
        /// <https://dev.twitch.tv/docs/api/reference/#modify-channel-information>
        fn modify_channel_info(
            &self,
            broadcaster_id: BroadcasterId,
            opts: Option<ModifyChannelRequest>,
        ) -> NoContent {
            endpoint_type: EndpointType::ModifyChannelInformation,
            method: Method::PATCH,
            path: [CHANNELS],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id)
            },
            headers: [json],
            body: opts.and_then(|o| o.into_json())
        }
        /// <https://dev.twitch.tv/docs/api/reference/#get-channel-editors>
        fn get_channel_editors(
            &self,
            broadcaster_id: BroadcasterId,
        ) -> ChannelEditorsResponse {
            endpoint_type: EndpointType::GetChannelEditors,
            method: Method::GET,
            path: [CHANNELS, "editors"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id)
            }
        }
        /// <https://dev.twitch.tv/docs/api/reference/#get-followed-channels>
        fn get_followed_channels(
            &self,
            user_id: UserId,
            broadcaster_id: Option<BroadcasterId>,
            pagination: Option<PaginationQuery>,
        ) -> FollowerdChannelsResponse {
            endpoint_type: EndpointType::GetFollowedChannels,
            method: Method::GET,
            path: [CHANNELS, "followed"],
            query_params: {
                query(USER_ID, user_id),
                opt(BROADCASTER_ID, broadcaster_id),
                pagination(pagination)
            }
        }
        /// <https://dev.twitch.tv/docs/api/reference/#get-channel-followers>
        fn get_channel_followers(
            &self,
            user_id: Option<UserId>,
            broadcaster_id: BroadcasterId,
            pagination: Option<PaginationQuery>,
        ) -> ChannelFollowersResponse {
            endpoint_type: EndpointType::GetChannelFollowers,
            method: Method::GET,
            path: [CHANNELS, "followers"],
            query_params: {
                opt(USER_ID, user_id),
                query(BROADCASTER_ID, broadcaster_id),
                pagination(pagination)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        channels::{request::ModifyChannelRequest, ChannelsAPI},
        test_utils::TwitchApiTest,
        types::{BroadcasterId, GameId, PaginationQuery, UserId},
    };

    #[tokio::test]
    pub(crate) async fn get_channel_info() {
        let suite = TwitchApiTest::new().await;

        suite.mock_channels_success().await;

        let broadcaster_ids = vec![BroadcasterId::new("123456789")];

        let response = suite
            .execute("/channels", |api| api.get_channel_info(&broadcaster_ids))
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].broadcaster_id.as_str(), "123456789");
        assert_eq!(response.data[0].title, "Testing My Stream");
    }

    #[tokio::test]
    async fn get_channel_info_multiple_broadcasters() {
        let suite = TwitchApiTest::new().await;

        suite.mock_channels_extra().await;

        let broadcaster_ids = vec![
            BroadcasterId::new("123456789"),
            BroadcasterId::new("987654321"),
        ];

        let response = suite
            .execute("/channels", |api| api.get_channel_info(&broadcaster_ids))
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 2);
        assert_eq!(response.data[0].broadcaster_id.as_str(), "123456789");
        assert_eq!(response.data[1].broadcaster_id.as_str(), "987654321");
        assert!(!response.data[0].is_branded_content);
        assert!(response.data[1].is_branded_content);
    }

    #[tokio::test]
    pub(crate) async fn modify_channel_info() {
        let suite = TwitchApiTest::new().await;

        suite.mock_channels_success().await;

        let tags = vec!["Gaming", "English", "Fun"];

        let modify_request = ModifyChannelRequest::new()
            .title("Updated Stream Title")
            .game_id(GameId::new("509658"))
            .broadcaster_language("en")
            .tags(&tags)
            .is_branded_content(true);

        let response = suite
            .execute("/channels", |api| {
                api.modify_channel_info(BroadcasterId::new("123456789"), Some(modify_request))
            })
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), 204);
    }

    #[tokio::test]
    pub(crate) async fn get_channel_editors() {
        let suite = TwitchApiTest::new().await;

        suite.mock_channels_success().await;

        let response = suite
            .execute("/channels/editors", |api| {
                api.get_channel_editors(BroadcasterId::new("123456789"))
            })
            .json()
            .await
            .unwrap();

        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].user_id.as_str(), "editor123");
        assert_eq!(response.data[0].user_name, "ChannelEditor");
    }

    #[tokio::test]
    pub(crate) async fn get_followed_channels() {
        let suite = TwitchApiTest::new().await;

        suite.mock_channels_success().await;

        let pagination = PaginationQuery::new().first(20);

        let response = suite
            .execute("/channels/followed", |api| {
                api.get_followed_channels(
                    UserId::new("user123"),
                    Some(BroadcasterId::new("123456789")),
                    Some(pagination),
                )
            })
            .json()
            .await
            .unwrap();

        assert!(response.data.is_some());
        let followed = response.data.unwrap();
        assert_eq!(followed.len(), 1);
        assert_eq!(followed[0].broadcaster_id.as_str(), "123456789");
        assert_eq!(response.total, 1);
        assert!(response.pagination.is_some());
    }

    #[tokio::test]
    pub(crate) async fn get_channel_followers() {
        let suite = TwitchApiTest::new().await;

        suite.mock_channels_success().await;

        let pagination = PaginationQuery::new().first(100);

        let response = suite
            .execute("/channels/followers", |api| {
                api.get_channel_followers(
                    Some(UserId::new("follower123")),
                    BroadcasterId::new("123456789"),
                    Some(pagination),
                )
            })
            .json()
            .await
            .unwrap();

        assert!(response.data.is_some());
        let followers = response.data.unwrap();
        assert_eq!(followers.len(), 1);
        assert_eq!(followers[0].user_id.as_str(), "follower123");
        assert_eq!(response.total, 1);
    }

    #[tokio::test]
    async fn channels_api_error_response() {
        let suite = TwitchApiTest::new().await;

        suite.mock_channels_failure().await;

        let broadcaster_ids = vec![BroadcasterId::new("123456789")];

        let response = suite
            .execute("/channels", |api| api.get_channel_info(&broadcaster_ids))
            .send()
            .await;

        match response {
            Ok(response) => {
                panic!("Expected Error, got: {response:?}")
            }
            Err(e) => {
                assert!(e.is_api());
            }
        }
    }
}
