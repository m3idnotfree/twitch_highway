use asknothingx2_util::api::Method;
use request::{BlockReason, BlockSourceContext};

use crate::{
    base::{TwitchAPI, TwitchAPIBase},
    request::{EmptyBody, EndpointType, TwitchAPIRequest},
    types::{BroadcasterId, Id, UserId, AFTER, BROADCASTER_ID, EXTENSIONS, FIRST, ID, USER_ID},
};

pub mod request;
pub mod response;
pub mod types;

const USERS: &str = "users";
const BLOCKS: &str = "blocks";
const LOGIN: &str = "login";

pub trait UserAPI: TwitchAPIBase {
    /// https://dev.twitch.tv/docs/api/reference/#get-users
    /// To include the user’s verified email address in the response,
    /// you must use a user access token that includes the
    /// user:read:email scope.
    fn users_info(
        &self,
        ids: Option<&[Id]>,
        logins: Option<&[String]>,
    ) -> TwitchAPIRequest<EmptyBody>;
    /// https://dev.twitch.tv/docs/api/reference/#update-user
    /// Requires a user access token that includes the user:edit scope.
    ///
    /// The string to update the channel’s description to.
    /// The description is limited to a maximum of 300 characters.
    ///
    /// To remove the description, specify this parameter but don’t set it’s value (for example, ?description=).
    fn update_user(&self, description: Option<&str>) -> TwitchAPIRequest<EmptyBody>;
    /// https://dev.twitch.tv/docs/api/reference/#get-user-block-list
    fn block_list(
        &self,
        broadcaster_id: BroadcasterId,
        first: Option<&str>,
        after: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody>;
    /// https://dev.twitch.tv/docs/api/reference/#block-user
    fn block_user(
        &self,
        target_user_id: &str,
        source_context: Option<BlockSourceContext>,
        reason: Option<BlockReason>,
    ) -> TwitchAPIRequest<EmptyBody>;
    /// https://dev.twitch.tv/docs/api/reference/#unblock-user
    fn unblock_user(&self, target_user_id: &str) -> TwitchAPIRequest<EmptyBody>;
    /// https://dev.twitch.tv/docs/api/reference/#get-user-extensions
    fn user_extensions(&self) -> TwitchAPIRequest<EmptyBody>;
    /// https://dev.twitch.tv/docs/api/reference/#get-user-active-extensions
    fn user_active_extensions(&self, user_id: Option<UserId>) -> TwitchAPIRequest<EmptyBody>;
    // https://dev.twitch.tv/docs/api/reference/#update-user-extensions
    //fn update_user_extensions(
    //    &self,
    //    data: UserActiveExtensionsData,
    //) -> TwitchAPIRequest<UserActiveExtensionsData>;
}

impl UserAPI for TwitchAPI {
    fn users_info(
        &self,
        ids: Option<&[Id]>,
        logins: Option<&[String]>,
    ) -> TwitchAPIRequest<EmptyBody> {
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

    fn update_user(&self, description: Option<&str>) -> TwitchAPIRequest<EmptyBody> {
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
        first: Option<&str>,
        after: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path([USERS, BLOCKS])
            .query([(BROADCASTER_ID, broadcaster_id)])
            .query_opt(FIRST, first)
            .query_opt(AFTER, after);

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
        target_user_id: &str,
        source_context: Option<BlockSourceContext>,
        reason: Option<BlockReason>,
    ) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path([USERS, BLOCKS])
            .query([("target_user_id", target_user_id)])
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

    fn unblock_user(&self, target_user_id: &str) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path([USERS, BLOCKS])
            .query([("target_user_id", target_user_id)]);

        TwitchAPIRequest::new(
            EndpointType::UnblockUser,
            url.build(),
            Method::DELETE,
            self.build_headers().build(),
            EmptyBody,
        )
    }

    fn user_extensions(&self) -> TwitchAPIRequest<EmptyBody> {
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

    fn user_active_extensions(&self, user_id: Option<UserId>) -> TwitchAPIRequest<EmptyBody> {
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

    //fn update_user_extensions(
    //    &self,
    //    data: UserActiveExtensionsData,
    //) -> TwitchAPIRequest<UserActiveExtensionsData> {
    //    TwitchAPIRequest::new(
    //        EndpointType::UpdateUserExtensions,
    //        self.build_url().path(["users", "extensions"]).build(),
    //        Method::PUT,
    //        self.build_headers().build(),
    //        data,
    //    )
    //}
}
