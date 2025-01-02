use asknothingx2_util::api::Method;
use request::{BlockReason, BlockSourceContext};

use crate::{
    base::{TwitchAPI, TwitchAPIBase},
    request::{EmptyBody, EndpointType, TwitchAPIRequest},
};

pub mod request;
pub mod response;
pub mod types;

pub trait UserAPI: TwitchAPIBase {
    /// https://dev.twitch.tv/docs/api/reference/#get-users
    /// To include the user’s verified email address in the response,
    /// you must use a user access token that includes the
    /// user:read:email scope.
    fn users_info<T: AsRef<str>, L: IntoIterator<Item = T>>(
        &self,
        ids: Option<L>,
        logins: Option<L>,
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
        broadcaster_id: &str,
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
    fn user_active_extensions(&self, user_id: Option<&str>) -> TwitchAPIRequest<EmptyBody>;
    // https://dev.twitch.tv/docs/api/reference/#update-user-extensions
    //fn update_user_extensions(
    //    &self,
    //    data: UserActiveExtensionsData,
    //) -> TwitchAPIRequest<UserActiveExtensionsData>;
}

impl UserAPI for TwitchAPI {
    fn users_info<T: AsRef<str>, L: IntoIterator<Item = T>>(
        &self,
        ids: Option<L>,
        logins: Option<L>,
    ) -> TwitchAPIRequest<EmptyBody> {
        TwitchAPIRequest::new(
            EndpointType::GetUsers,
            self.build_url()
                .path(["users"])
                .query_option_extend(ids.map(|ids| ids.into_iter().map(|id| ("id", id))))
                .query_option_extend(
                    logins.map(|logins| logins.into_iter().map(|login| ("login", login))),
                )
                .build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }

    fn update_user(&self, description: Option<&str>) -> TwitchAPIRequest<EmptyBody> {
        TwitchAPIRequest::new(
            EndpointType::UpdateUser,
            self.build_url()
                .path(["users"])
                .query_option("description", description)
                .build(),
            Method::PUT,
            self.build_headers().build(),
            EmptyBody,
        )
    }

    fn block_list(
        &self,
        broadcaster_id: &str,
        first: Option<&str>,
        after: Option<&str>,
    ) -> TwitchAPIRequest<EmptyBody> {
        TwitchAPIRequest::new(
            EndpointType::GetUserBlockList,
            self.build_url()
                .path(["users", "blocks"])
                .query([("broadcaster_id", broadcaster_id)])
                .query_option("first", first)
                .query_option("after", after)
                .build(),
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
        TwitchAPIRequest::new(
            EndpointType::BlockUser,
            self.build_url()
                .path(["users", "blocks"])
                .query([("target_user_id", target_user_id)])
                .query_option("source_context", source_context)
                .query_option("reason", reason)
                .build(),
            Method::PUT,
            self.build_headers().build(),
            EmptyBody,
        )
    }

    fn unblock_user(&self, target_user_id: &str) -> TwitchAPIRequest<EmptyBody> {
        TwitchAPIRequest::new(
            EndpointType::UnblockUser,
            self.build_url()
                .path(["users", "blocks"])
                .query([("target_user_id", target_user_id)])
                .build(),
            Method::DELETE,
            self.build_headers().build(),
            EmptyBody,
        )
    }

    fn user_extensions(&self) -> TwitchAPIRequest<EmptyBody> {
        TwitchAPIRequest::new(
            EndpointType::GetUserExtensions,
            self.build_url()
                .path(["users", "extensions", "list"])
                .build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }

    fn user_active_extensions(&self, user_id: Option<&str>) -> TwitchAPIRequest<EmptyBody> {
        TwitchAPIRequest::new(
            EndpointType::GetUserActiveExtensions,
            self.build_url()
                .path(["users", "extensions"])
                .query_option("user_id", user_id)
                .build(),
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
