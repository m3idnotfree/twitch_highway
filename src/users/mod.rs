mod builder;
mod response;
mod types;

pub use builder::{
    CreateBlockUser, GetUserActiveExtensions, GetUserBlockList, GetUsers, UpdateUserExtensions,
};
pub use response::{
    BlockUserListResponse, GetAuthorizationByUserResponse, UpdateUsersResponse,
    UserActiveExtensionsResponse, UserExtensionsResponse, UsersInfoResponse,
};
pub use types::{
    BlockReason, BlockSourceContext, BlockUser, BroadcasterType, Component, ExtensionType, Overlay,
    Panel, User, UserActiveExtensions, UserAuthorization, UserExtension, UserType,
};

use std::future::Future;

use crate::{
    types::{
        constants::{
            AUTHORIZATION, BLOCKS, DESCRIPTION, EXTENSIONS, LIST, TARGET_USER_ID, USERS, USER_ID,
        },
        BroadcasterId, UserId,
    },
    Client, Error,
};

pub trait UserAPI {
    /// See <https://dev.twitch.tv/docs/api/reference/#get-users>
    fn get_users<'a>(&'a self) -> GetUsers<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#update-user>
    fn update_user(
        &self,
        description: &str,
    ) -> impl Future<Output = Result<UpdateUsersResponse, Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#get-user-block-list>
    fn get_user_block_list<'a>(&'a self, broadcaster_id: &'a BroadcasterId)
        -> GetUserBlockList<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#block-user>
    fn block_user<'a>(&'a self, target_user_id: &'a UserId) -> CreateBlockUser<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#unblock-user>
    fn unblock_user(
        &self,
        target_user_id: &UserId,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#get-user-extensions>
    fn get_user_extensions(
        &self,
    ) -> impl Future<Output = Result<UserExtensionsResponse, Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#get-user-active-extensions>
    fn get_user_active_extensions<'a>(&'a self) -> GetUserActiveExtensions<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#update-user-extensions>
    fn update_user_extensions<'a>(&'a self) -> UpdateUserExtensions<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#get-authorization-by-user>
    fn get_authorization_by_user(
        &self,
        user_ids: &[UserId],
    ) -> impl Future<Output = Result<GetAuthorizationByUserResponse, Error>> + Send;
}

impl UserAPI for Client {
    fn get_users<'a>(&'a self) -> GetUsers<'a> {
        GetUsers::new(self)
    }

    async fn update_user(&self, description: &str) -> Result<UpdateUsersResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().push(USERS);

        url.query_pairs_mut().append_pair(DESCRIPTION, description);

        self.json(self.http_client().put(url)).await
    }

    fn get_user_block_list<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
    ) -> GetUserBlockList<'a> {
        GetUserBlockList::new(self, broadcaster_id)
    }

    fn block_user<'a>(&'a self, target_user_id: &'a UserId) -> CreateBlockUser<'a> {
        CreateBlockUser::new(self, target_user_id)
    }

    async fn unblock_user(&self, target_user_id: &UserId) -> Result<(), Error> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().extend([USERS, BLOCKS]);

        url.query_pairs_mut()
            .append_pair(TARGET_USER_ID, target_user_id);

        self.no_content(self.http_client().delete(url)).await
    }

    async fn get_user_extensions(&self) -> Result<UserExtensionsResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([USERS, EXTENSIONS, LIST]);

        self.json(self.http_client().get(url)).await
    }

    fn get_user_active_extensions<'a>(&'a self) -> GetUserActiveExtensions<'a> {
        GetUserActiveExtensions::new(self)
    }

    fn update_user_extensions<'a>(&'a self) -> UpdateUserExtensions<'a> {
        UpdateUserExtensions::new(self)
    }

    async fn get_authorization_by_user(
        &self,
        user_ids: &[UserId],
    ) -> Result<GetAuthorizationByUserResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([AUTHORIZATION, USERS]);

        url.query_pairs_mut()
            .extend_pairs(user_ids.iter().map(|id| (USER_ID, id)));

        self.json(self.http_client().get(url)).await
    }
}
