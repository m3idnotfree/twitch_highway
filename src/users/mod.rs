mod builder;
mod response;
mod types;

pub use builder::{
    BlockUserBuilder, GetUserActiveExtensionsBuilder, GetUserBlockListBuilder, GetUsersBuilder,
    UpdateUserExtensionsBuilder,
};
pub use response::{
    BlockUserListResponse, GetAuthorizationByUserResponse, UpdateUsersResponse,
    UserActiveExtensionsResponse, UserExtensionsResponse, UsersInfoResponse,
};
pub use types::{
    BlockReason, BlockSourceContext, BlockUser, BroadcasterType, Component, ExtensionType, Overlay,
    Panel, User, UserActiveExtensions, UserAuthorization, UserExtension, UserType,
};

use crate::{
    request::{NoContent, TwitchAPIRequest},
    types::{
        constants::{
            AUTHORIZATION, BLOCKS, DESCRIPTION, EXTENSIONS, LIST, TARGET_USER_ID, USERS, USER_ID,
        },
        BroadcasterId, UserId,
    },
    TwitchAPI,
};

pub trait UserAPI {
    /// Gets information about one or more users
    ///
    /// # Returns
    ///
    /// Returns a [`GetUsersBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::TwitchAPI;
    /// use twitch_highway::{
    ///     types::UserId,
    ///     users::UserAPI,
    /// };
    ///
    /// # async fn example(api: TwitchAPI) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .get_users()
    ///     .ids(&[UserId::from("1234")])
    ///     .json()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `user:read:email`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-users>
    fn get_users<'a>(&'a self) -> GetUsersBuilder<'a>;

    /// Updates the specified user’s information
    ///
    /// # Arguments
    ///
    /// * `description` - update the chanel's description (max 300).
    ///   - To remove the description, specify this parameter but don’t set it’s value (for example, ?description=).
    ///
    /// # Returns
    ///
    /// Returns a [`UpdateUsersResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::TwitchAPI;
    /// use twitch_highway::users::UserAPI;
    ///
    /// # async fn example(api: TwitchAPI) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api.update_user("description").json().await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `user:edit`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#update-user>
    fn update_user(&self, description: &str) -> TwitchAPIRequest<UpdateUsersResponse>;

    /// Gets the list of users that the broadcaster has blocked
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster whose list of blocked users you want to get.
    ///
    /// # Returns
    ///
    /// Returns a [`GetUserBlockListBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::TwitchAPI;
    /// use twitch_highway::{
    ///     types::BroadcasterId,
    ///     users::UserAPI,
    /// };
    ///
    /// # async fn example(api: TwitchAPI) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .get_user_block_list(&BroadcasterId::from("1234"))
    ///     .json()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `user:read:blocked_users`
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-user-block-list>
    fn get_user_block_list<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
    ) -> GetUserBlockListBuilder<'a>;

    /// Blocks the specified user from interacting with or having contact with the broadcaster
    ///
    /// # Arguments
    ///
    /// * `target_user_id` - The ID of the user to block.
    ///
    /// # Returns
    ///
    /// Returns a [`BlockUserBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::TwitchAPI;
    /// use twitch_highway::{
    ///     types::UserId,
    ///     users::{BlockReason, BlockSourceContext, UserAPI},
    /// };
    ///
    /// # async fn example(api: TwitchAPI) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .block_user(&UserId::from("1234"))
    ///     .source_context(BlockSourceContext::Chat)
    ///     .reason(BlockReason::Harassment)
    ///     .json()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `user:manage:blocked_users `
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#block-user>
    fn block_user<'a>(&'a self, target_user_id: &'a UserId) -> BlockUserBuilder<'a>;

    /// Removes the user from the broadcaster’s list of blocked users
    ///
    /// # Arguments
    ///
    /// * `target_user_id` - The ID of the user to remove from the broadcaster’s list of blocked users.
    ///
    /// # Returns
    ///
    /// Returns a [`NoContent`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::TwitchAPI;
    /// use twitch_highway::{
    ///     types::UserId,
    ///     users::UserAPI,
    /// };
    ///
    /// # async fn example(api: TwitchAPI) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api.unblock_user(&UserId::from("1234")).json().await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `user:manage:blocked_users `
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#unblock-user>
    fn unblock_user(&self, target_user_id: &UserId) -> TwitchAPIRequest<NoContent>;

    /// Gets a list of all extensions (both active and inactive) that the broadcaster has installed
    ///
    /// # Returns
    ///
    /// Returns a [`UserExtensionsResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::TwitchAPI;
    /// use twitch_highway::users::UserAPI;
    ///
    /// # async fn example(api: TwitchAPI) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api.get_user_extensions().json().await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// `user:read:broadcast or user:edit:broadcast`
    /// - To include inactive extensions, you must include the user:edit:broadcast scope.
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-user-extensions>
    fn get_user_extensions(&self) -> TwitchAPIRequest<UserExtensionsResponse>;

    /// Gets the active extensions that the broadcaster has installed for each configuration
    ///
    /// # Returns
    ///
    /// Returns a [`GetUserActiveExtensionsBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::TwitchAPI;
    /// use twitch_highway::{
    ///     types::UserId,
    ///     users::UserAPI,
    /// };
    ///
    /// # async fn example(api: TwitchAPI) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .get_user_active_extensions()
    ///     .user_id(&UserId::from("1234"))
    ///     .json()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// No scope required
    ///
    /// NOTE: To include extensions that you have under development, you must specify a user access token that includes the `user:read:broadcast` or `user:edit:broadcast` scope.
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-user-active-extensions>
    fn get_user_active_extensions<'a>(&'a self) -> GetUserActiveExtensionsBuilder<'a>;

    /// Updates an installed extension’s information
    ///
    /// # Returns
    ///
    /// Returns a [`UpdateUserExtensionsBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::TwitchAPI;
    /// use twitch_highway::users::{Component, Overlay, Panel, UserAPI};
    ///
    /// # async fn example(api: TwitchAPI) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .update_user_extensions()
    ///     .add_panel(Panel::new(true))
    ///     .add_overlay(Overlay::new(true))
    ///     .add_component(Component::new(true))
    ///     .json()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// No scope required
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#update-user-extensions>
    fn update_user_extensions<'a>(&'a self) -> UpdateUserExtensionsBuilder<'a>;

    /// Gets the authorization scopes that the specified user(s) have granted the application
    ///
    /// # Returns
    ///
    /// Returns a [`GetAuthorizationByUserResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::TwitchAPI;
    /// use twitch_highway::{
    ///     types::UserId,
    ///     users::UserAPI,
    /// };
    ///
    /// # async fn example(api: TwitchAPI) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .get_authorization_by_user(&[UserId::from("1234"), UserId::from("5678")])
    ///     .json()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// No scope required
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-authorization-by-user>
    fn get_authorization_by_user(
        &self,
        user_ids: &[UserId],
    ) -> TwitchAPIRequest<GetAuthorizationByUserResponse>;
}

impl UserAPI for TwitchAPI {
    fn get_users<'a>(&'a self) -> GetUsersBuilder<'a> {
        GetUsersBuilder::new(self)
    }
    fn update_user(&self, description: &str) -> TwitchAPIRequest<UpdateUsersResponse> {
        let mut url = self.build_url();

        url.path_segments_mut().unwrap().extend(&[USERS]);

        let mut query = url.query_pairs_mut();

        query.append_pair(DESCRIPTION, description);

        drop(query);

        TwitchAPIRequest::new(
            crate::request::EndpointType::UpdateUser,
            url,
            reqwest::Method::PUT,
            self.default_headers(),
            None,
            self.client.clone(),
        )
    }
    fn get_user_block_list<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
    ) -> GetUserBlockListBuilder<'a> {
        GetUserBlockListBuilder::new(self, broadcaster_id)
    }
    fn block_user<'a>(&'a self, target_user_id: &'a UserId) -> BlockUserBuilder<'a> {
        BlockUserBuilder::new(self, target_user_id)
    }
    fn unblock_user(&self, target_user_id: &UserId) -> TwitchAPIRequest<NoContent> {
        let mut url = self.build_url();

        url.path_segments_mut().unwrap().extend(&[USERS, BLOCKS]);

        let mut query = url.query_pairs_mut();

        query.append_pair(TARGET_USER_ID, target_user_id);

        drop(query);

        TwitchAPIRequest::new(
            crate::request::EndpointType::UnblockUser,
            url,
            reqwest::Method::DELETE,
            self.default_headers(),
            None,
            self.client.clone(),
        )
    }
    fn get_user_extensions(&self) -> TwitchAPIRequest<UserExtensionsResponse> {
        let mut url = self.build_url();

        url.path_segments_mut()
            .unwrap()
            .extend(&[USERS, EXTENSIONS, LIST]);

        TwitchAPIRequest::new(
            crate::request::EndpointType::GetUserExtensions,
            url,
            reqwest::Method::GET,
            self.default_headers(),
            None,
            self.client.clone(),
        )
    }
    fn get_user_active_extensions<'a>(&'a self) -> GetUserActiveExtensionsBuilder<'a> {
        GetUserActiveExtensionsBuilder::new(self)
    }
    fn update_user_extensions<'a>(&'a self) -> UpdateUserExtensionsBuilder<'a> {
        UpdateUserExtensionsBuilder::new(self)
    }
    simple_endpoint!(
    fn get_authorization_by_user(
        user_ids: &[UserId] [key = USER_ID, convert = extend]
    ) -> GetAuthorizationByUserResponse;
        endpoint: GetAuthorizationByUser,
        method: GET,
        path: [AUTHORIZATION, USERS],
    );
}
