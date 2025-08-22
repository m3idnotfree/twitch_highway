use asknothingx2_util::api::Method;
use request::{BlockReason, BlockSourceContext};
use response::{
    BlockUserListResponse, UpdateUsersResponse, UserActiveExtensionsResponse,
    UserExtensionsResponse, UsersInfoResponse,
};
use types::UserActiveExtensions;

use crate::{
    request::{EndpointType, NoContent, TwitchAPIRequest},
    types::{
        constants::{BLOCKS, BROADCASTER_ID, EXTENSIONS, ID, LOGIN, USERS, USER_ID},
        BroadcasterId, Id, PaginationQuery, UserId,
    },
    TwitchAPI,
};

pub mod request;
pub mod response;
pub mod types;

endpoints! {
    UserAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#get-users>
        fn get_users(
            &self,
            ids: Option<&[Id]>,
            logins: Option<&[&str]>,
        ) -> UsersInfoResponse {
            endpoint_type: EndpointType::GetUsers,
            method: Method::GET,
            path: [USERS],
            query_params: {
                opt_extend(ids.map(|ids| ids.iter().map(|id| (ID, id)))),
                opt_extend(logins.map(|lg| lg.iter().map(|l| (LOGIN, l) )))
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#update-user>
        fn update_user(&self, description: Option<&str>) -> UpdateUsersResponse {
            endpoint_type: EndpointType::UpdateUser,
            method: Method::PUT,
            path: [USERS],
            query_params: {
                opt("description", description)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#get-user-block-list>
        fn get_user_block_list(
            &self,
            broadcaster_id: &BroadcasterId,
            pagination: Option<PaginationQuery>,
        ) -> BlockUserListResponse {
            endpoint_type: EndpointType::GetUserBlockList,
            method: Method::GET,
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
            endpoint_type: EndpointType::BlockUser,
            method: Method::PUT,
            path: [USERS, BLOCKS],
            query_params: {
                query("target_user_id", target_user_id),
                opt("source_context", source_context),
                opt("reason", reason)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#unblock-user>
        fn unblock_user(&self, target_user_id: &UserId) -> NoContent {
            endpoint_type: EndpointType::UnblockUser,
            method: Method::DELETE,
            path: [USERS, BLOCKS],
            query_params: {
                query("target_user_id", target_user_id)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#get-user-extensions>
        fn get_user_extensions(&self) -> UserExtensionsResponse {
            endpoint_type: EndpointType::GetUserExtensions,
            method: Method::GET,
            path: [USERS, EXTENSIONS, "list"]
        }

        /// <https://dev.twitch.tv/docs/api/reference/#get-user-active-extensions>
        fn get_user_active_extensions(
            &self,
            user_id: Option<&UserId>,
        ) -> UserActiveExtensionsResponse {
            endpoint_type: EndpointType::GetUserActiveExtensions,
            method: Method::GET,
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
            endpoint_type: EndpointType::UpdateUserExtensions,
            method: Method::PUT,
            path: [USERS, EXTENSIONS],
            headers: [json],
            body: data.into_json()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        types::{BroadcasterId, Id, UserId},
        users::{
            types::{Component, Overlay, Panel, UserActiveExtensions},
            UserAPI,
        },
    };
    api_test!(get_users, [Some(&[Id::new("141981764")]), None]);
    api_test!(update_user, [Some("BaldAngel")]);
    api_test!(
        get_user_block_list,
        [&BroadcasterId::new("141981764"), None]
    );
    api_test!(block_user, [&UserId::new("198704263"), None, None]);
    api_test!(unblock_user, [&UserId::new("198704263")]);
    api_test!(get_user_extensions, []);
    api_test!(get_user_active_extensions, [None]);
    api_test!(
        update_user_extensions,
        [{
            let mut panels = HashMap::new();
            panels.insert(
                "1".to_string(),
                Panel::new(true)
                    .id(Id::new("rh6jq1q334hqc2rr1qlzqbvwlfl3x0"))
                    .version("1.1.8".to_string()),
            );

            panels.insert(
                "2".to_string(),
                Panel::new(true)
                    .id(Id::new("wi08ebtatdc7oj83wtl9uxwz807l8b"))
                    .version("1.1.8"),
            );

            panels.insert(
                "2".to_string(),
                Panel::new(true)
                    .id(Id::new("naty2zwfp7vecaivuve8ef1hohh6bo"))
                    .version("1.1.8"),
            );

            let mut overlays = HashMap::new();
            overlays.insert(
                "1".to_string(),
                Overlay::new(true)
                    .id(Id::new("zfh2irvx2jb4s60f02jq0ajm8vwgka"))
                    .version("1.0.19"),
            );

            let mut components = HashMap::new();
            components.insert(
                "1".to_string(),
                Component::new(true)
                    .id(Id::new("lqnf3zxk0rv0g7gq92mtmnirjz2cjj"))
                    .version("0.0.1")
                    .name("Dev Experience Test")
                    .x(0)
                    .y(0),
            );

            components.insert("2".to_string(), Component::new(false));

            UserActiveExtensions::new(panels, overlays, components)
        }]
    );
}
