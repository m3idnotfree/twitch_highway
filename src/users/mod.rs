use asknothingx2_util::api::Method;
use request::{BlockReason, BlockSourceContext};
use response::{
    BlockUserListResponse, UpdateUsersResponse, UserActiveExtensionsResponse,
    UserExtensionsResponse, UsersInfoResponse,
};
use types::UserActiveExtensions;

use crate::{
    request::{EmptyBody, EndpointType, TwitchAPIRequest},
    types::{
        constants::{BLOCKS, BROADCASTER_ID, EXTENSIONS, ID, LOGIN, USERS, USER_ID},
        BroadcasterId, Id, PaginationQuery, UserId,
    },
    TwitchAPI,
};

pub mod request;
pub mod response;
pub mod types;

#[cfg_attr(docsrs, doc(cfg(feature = "users")))]
pub trait UserAPI {
    /// <https://dev.twitch.tv/docs/api/reference/#get-users>
    fn users_info(
        &self,
        ids: Option<&[Id]>,
        logins: Option<&[String]>,
    ) -> TwitchAPIRequest<EmptyBody, UsersInfoResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#update-user>
    fn update_user(
        &self,
        description: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody, UpdateUsersResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#get-user-block-list>
    fn block_list(
        &self,
        broadcaster_id: BroadcasterId,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, BlockUserListResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#block-user>
    fn block_user(
        &self,
        target_user_id: UserId,
        source_context: Option<BlockSourceContext>,
        reason: Option<BlockReason>,
    ) -> TwitchAPIRequest<EmptyBody, EmptyBody>;
    /// <https://dev.twitch.tv/docs/api/reference/#unblock-user>
    fn unblock_user(&self, target_user_id: &str) -> TwitchAPIRequest<EmptyBody, EmptyBody>;
    /// <https://dev.twitch.tv/docs/api/reference/#get-user-extensions>
    fn user_extensions(&self) -> TwitchAPIRequest<EmptyBody, UserExtensionsResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#get-user-active-extensions>
    fn user_active_extensions(
        &self,
        user_id: Option<UserId>,
    ) -> TwitchAPIRequest<EmptyBody, UserActiveExtensionsResponse>;
    // <https://dev.twitch.tv/docs/api/reference/#update-user-extensions>
    fn update_user_extensions(
        &self,
        data: UserActiveExtensions,
    ) -> TwitchAPIRequest<UserActiveExtensions, UserActiveExtensionsResponse>;
}

impl UserAPI for TwitchAPI {
    fn users_info(
        &self,
        ids: Option<&[Id]>,
        logins: Option<&[String]>,
    ) -> TwitchAPIRequest<EmptyBody, UsersInfoResponse> {
        let mut url = self.build_url();
        url.path([USERS])
            .query_opt_extend(ids.map(|ids| ids.iter().map(|id| (ID, id))))
            .query_opt_extend(logins.map(|logins| logins.iter().map(|login| (LOGIN, login))));

        TwitchAPIRequest::new(
            EndpointType::GetUsers,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }

    fn update_user(
        &self,
        description: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody, UpdateUsersResponse> {
        let mut url = self.build_url();
        url.path([USERS]).query_opt("description", description);

        TwitchAPIRequest::new(
            EndpointType::UpdateUser,
            url.build(),
            Method::PUT,
            self.build_headers().build(),
            EmptyBody,
        )
    }

    fn block_list(
        &self,
        broadcaster_id: BroadcasterId,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, BlockUserListResponse> {
        let mut url = self.build_url();
        url.path([USERS, BLOCKS])
            .query(BROADCASTER_ID, broadcaster_id);

        if let Some(pagination) = pagination {
            pagination.apply_to_url(&mut url);
        }

        TwitchAPIRequest::new(
            EndpointType::GetUserBlockList,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }

    fn block_user(
        &self,
        target_user_id: UserId,
        source_context: Option<BlockSourceContext>,
        reason: Option<BlockReason>,
    ) -> TwitchAPIRequest<EmptyBody, EmptyBody> {
        let mut url = self.build_url();
        url.path([USERS, BLOCKS])
            .query("target_user_id", target_user_id)
            .query_opt("source_context", source_context)
            .query_opt("reason", reason);

        TwitchAPIRequest::new(
            EndpointType::BlockUser,
            url.build(),
            Method::PUT,
            self.build_headers().build(),
            EmptyBody,
        )
    }

    fn unblock_user(&self, target_user_id: &str) -> TwitchAPIRequest<EmptyBody, EmptyBody> {
        let mut url = self.build_url();
        url.path([USERS, BLOCKS])
            .query("target_user_id", target_user_id);

        TwitchAPIRequest::new(
            EndpointType::UnblockUser,
            url.build(),
            Method::DELETE,
            self.build_headers().build(),
            EmptyBody,
        )
    }

    fn user_extensions(&self) -> TwitchAPIRequest<EmptyBody, UserExtensionsResponse> {
        let mut url = self.build_url();
        url.path([USERS, EXTENSIONS, "list"]);

        TwitchAPIRequest::new(
            EndpointType::GetUserExtensions,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }

    fn user_active_extensions(
        &self,
        user_id: Option<UserId>,
    ) -> TwitchAPIRequest<EmptyBody, UserActiveExtensionsResponse> {
        let mut url = self.build_url();
        url.path([USERS, EXTENSIONS]).query_opt(USER_ID, user_id);

        TwitchAPIRequest::new(
            EndpointType::GetUserActiveExtensions,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }

    fn update_user_extensions(
        &self,
        data: UserActiveExtensions,
    ) -> TwitchAPIRequest<UserActiveExtensions, UserActiveExtensionsResponse> {
        let mut url = self.build_url();
        url.path([USERS, EXTENSIONS]);

        let mut headers = self.build_headers();
        headers.json();

        TwitchAPIRequest::new(
            EndpointType::UpdateUserExtensions,
            url.build(),
            Method::PUT,
            headers.build(),
            data,
        )
    }
}
