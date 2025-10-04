mod request;
mod response;
mod types;

pub use request::{BlockReason, BlockSourceContext};
pub use response::{
    BlockUserListResponse, UpdateUsersResponse, UserActiveExtensionsResponse,
    UserExtensionsResponse, UsersInfoResponse,
};
pub use types::{
    BlockUser, BroadcasterType, Component, ExtensionType, Overlay, Panel, User,
    UserActiveExtensions, UserExtension, UserType,
};

use crate::{
    request::NoContent,
    types::{
        constants::{BROADCASTER_ID, EXTENSIONS, ID, USER_ID},
        BroadcasterId, Id, PaginationQuery, UserId,
    },
};

const USERS: &str = "users";
const BLOCKS: &str = "blocks";
const LOGIN: &str = "login";

endpoints! {
    UserAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#get-users>
        fn get_users(
            &self,
            ids: Option<&[Id]>,
            logins: Option<&[&str]>,
        ) -> UsersInfoResponse {
            endpoint_type: GetUsers,
            method: GET,
            path: [USERS],
            query_params: {
                opt_extend(ids.map(|ids| ids.iter().map(|id| (ID, id)))),
                opt_extend(logins.map(|lg| lg.iter().map(|l| (LOGIN, l) )))
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#update-user>
        fn update_user(&self, description: &str) -> UpdateUsersResponse {
            endpoint_type: UpdateUser,
            method: PUT,
            path: [USERS],
            query_params: {
                query("description", description)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#get-user-block-list>
        fn get_user_block_list(
            &self,
            broadcaster_id: &BroadcasterId,
            pagination: Option<PaginationQuery>,
        ) -> BlockUserListResponse {
            endpoint_type: GetUserBlockList,
            method: GET,
            path: [USERS, BLOCKS],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                pagination(pagination)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#block-user>
        fn block_user(
            &self,
            target_user_id: &UserId,
            source_context: Option<BlockSourceContext>,
            reason: Option<BlockReason>,
        ) -> NoContent {
            endpoint_type: BlockUser,
            method: PUT,
            path: [USERS, BLOCKS],
            query_params: {
                query("target_user_id", target_user_id),
                opt("source_context", source_context),
                opt("reason", reason)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#unblock-user>
        fn unblock_user(&self, target_user_id: &UserId) -> NoContent {
            endpoint_type: UnblockUser,
            method: DELETE,
            path: [USERS, BLOCKS],
            query_params: {
                query("target_user_id", target_user_id)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#get-user-extensions>
        fn get_user_extensions(&self) -> UserExtensionsResponse {
            endpoint_type: GetUserExtensions,
            method: GET,
            path: [USERS, EXTENSIONS, "list"]
        }

        /// <https://dev.twitch.tv/docs/api/reference/#get-user-active-extensions>
        fn get_user_active_extensions(
            &self,
            user_id: Option<&UserId>,
        ) -> UserActiveExtensionsResponse {
            endpoint_type: GetUserActiveExtensions,
            method: GET,
            path: [USERS, EXTENSIONS],
            query_params: {
                opt(USER_ID, user_id)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#update-user-extensions>
        fn update_user_extensions(
            &self,
            data: UserActiveExtensions,
        ) -> UserActiveExtensionsResponse {
            endpoint_type: UpdateUserExtensions,
            method: PUT,
            path: [USERS, EXTENSIONS],
            headers: [json],
            body: data.into_json()
        }
    }
}
