use crate::{
    request::{NoContent, TwitchAPIRequest},
    types::{
        constants::{
            AFTER, BLOCKS, BROADCASTER_ID, EXTENSIONS, FIRST, ID, LOGIN, REASON, SOURCE_CONTEXT,
            TARGET_USER_ID, USERS, USER_ID,
        },
        BroadcasterId, UserId,
    },
    users::{
        BlockReason, BlockSourceContext, BlockUserListResponse, Component, Overlay, Panel,
        UserActiveExtensions, UserActiveExtensionsResponse, UsersInfoResponse,
    },
    Client,
};

define_request_builder! {
    #[derive(Debug)]
    GetUsersBuilder<'a> {
        ids: &'a [UserId] [key = ID, convert = extend],
        logins: &'a [&'a str] [key = LOGIN, convert = extend]
    } -> UsersInfoResponse;
    endpoint: GetUsers,
    method: GET,
    path: [USERS],
}

define_request_builder! {
    #[derive(Debug)]
    GetUserBlockListBuilder<'a> {
        req: {broadcaster_id: &'a BroadcasterId [key = BROADCASTER_ID]}
        opts: {
            first: u8 [key = FIRST, convert = to_string],
            after: &'a str [key = AFTER]
        }
    } -> BlockUserListResponse;
    endpoint: GetUserBlockList,
    method: GET,
    path: [USERS, BLOCKS],
}

define_request_builder! {
    #[derive(Debug)]
    BlockUserBuilder<'a> {
        req: {target_user_id: &'a UserId [key = TARGET_USER_ID, convert = as_ref]},
        opts: {
            source_context: BlockSourceContext [key = SOURCE_CONTEXT, convert = as_ref],
            reason: BlockReason [key = REASON, convert = as_ref]
        }
    } -> NoContent;
    endpoint: BlockUser,
    method: PUT,
    path: [USERS, BLOCKS],

}

define_request_builder! {
    #[derive(Debug)]
    GetUserActiveExtensionsBuilder<'a> {
        user_id: &'a UserId [key = USER_ID]
    } -> UserActiveExtensionsResponse;
    endpoint: GetUserActiveExtensions,
    method: GET,
    path: [USERS, EXTENSIONS],
}

#[derive(Debug)]
pub struct UpdateUserExtensionsBuilder<'a> {
    api: &'a Client,
    data: UserActiveExtensions,
}

impl<'a> UpdateUserExtensionsBuilder<'a> {
    pub fn new(api: &'a Client) -> Self {
        Self {
            api,
            data: UserActiveExtensions::new(),
        }
    }

    pub fn add_panel(mut self, panel: Panel) -> Self {
        self.data = self.data.add_panel(panel);
        self
    }

    pub fn add_panels(mut self, panels: Vec<Panel>) -> Self {
        self.data = self.data.add_panels(panels);
        self
    }

    pub fn add_overlay(mut self, overlay: Overlay) -> Self {
        self.data = self.data.add_overlay(overlay);
        self
    }

    pub fn add_component(mut self, component: Component) -> Self {
        self.data = self.data.add_component(component);
        self
    }

    pub fn build(self) -> TwitchAPIRequest<UserActiveExtensionsResponse> {
        let mut url = self.api.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend(&[USERS, EXTENSIONS]);

        let body = serde_json::to_string(&self.data).ok();

        TwitchAPIRequest::new(
            crate::request::EndpointType::UpdateUserExtensions,
            url,
            reqwest::Method::PUT,
            self.api.header_json(),
            body,
            self.api.http_client().clone(),
        )
    }

    pub async fn send(self) -> Result<reqwest::Response, crate::Error> {
        self.build().send().await
    }

    pub async fn json(self) -> Result<UserActiveExtensionsResponse, crate::Error> {
        self.build().json().await
    }
}
