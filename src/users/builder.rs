use crate::{
    Client, Error,
    types::{
        BroadcasterId, UserId,
        constants::{
            AFTER, BLOCKS, BROADCASTER_ID, EXTENSIONS, FIRST, ID, LOGIN, REASON, SOURCE_CONTEXT,
            TARGET_USER_ID, USER_ID, USERS,
        },
    },
    users::{
        BlockReason, BlockSourceContext, BlockUserListResponse, Component, Overlay, Panel,
        UserActiveExtensions, UserActiveExtensionsResponse, UsersInfoResponse,
    },
};

#[derive(Debug)]
pub struct GetUsers<'a> {
    client: &'a Client,
    ids: Option<&'a [UserId]>,
    logins: Option<&'a [&'a str]>,
}

impl<'a> GetUsers<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self {
            client,
            ids: None,
            logins: None,
        }
    }

    pub fn ids(mut self, value: &'a [UserId]) -> Self {
        self.ids = Some(value);
        self
    }

    pub fn logins(mut self, value: &'a [&'a str]) -> Self {
        self.logins = Some(value);
        self
    }

    pub async fn send(self) -> Result<UsersInfoResponse, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut().unwrap().push(USERS);

        if let Some(ids) = self.ids {
            url.query_pairs_mut()
                .extend_pairs(ids.iter().map(|id| (ID, id)));
        }

        if let Some(logins) = self.logins {
            url.query_pairs_mut()
                .extend_pairs(logins.iter().map(|login| (LOGIN, login)));
        }

        let req = self.client.http_client().get(url);
        self.client.json(req).await
    }
}

#[derive(Debug)]
pub struct GetUserBlockList<'a> {
    client: &'a Client,
    broadcaster_id: &'a BroadcasterId,
    first: Option<u8>,
    after: Option<&'a str>,
}

impl<'a> GetUserBlockList<'a> {
    pub fn new(client: &'a Client, broadcaster_id: &'a BroadcasterId) -> Self {
        Self {
            client,
            broadcaster_id,
            first: None,
            after: None,
        }
    }

    pub fn first(mut self, value: u8) -> Self {
        self.first = Some(value);
        self
    }

    pub fn after(mut self, value: &'a str) -> Self {
        self.after = Some(value);
        self
    }

    pub async fn send(self) -> Result<BlockUserListResponse, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut().unwrap().extend([USERS, BLOCKS]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, self.broadcaster_id);

        if let Some(val) = self.first {
            url.query_pairs_mut().append_pair(FIRST, &val.to_string());
        }

        if let Some(val) = self.after {
            url.query_pairs_mut().append_pair(AFTER, val);
        }

        let req = self.client.http_client().get(url);
        self.client.json(req).await
    }
}

#[derive(Debug)]
pub struct CreateBlockUser<'a> {
    client: &'a Client,
    target_user_id: &'a UserId,
    source_context: Option<BlockSourceContext>,
    reason: Option<BlockReason>,
}

impl<'a> CreateBlockUser<'a> {
    pub fn new(client: &'a Client, target_user_id: &'a UserId) -> Self {
        Self {
            client,
            target_user_id,
            source_context: None,
            reason: None,
        }
    }

    pub fn source_context(mut self, value: BlockSourceContext) -> Self {
        self.source_context = Some(value);
        self
    }

    pub fn reason(mut self, value: BlockReason) -> Self {
        self.reason = Some(value);
        self
    }

    pub async fn send(self) -> Result<(), Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut().unwrap().extend([USERS, BLOCKS]);

        url.query_pairs_mut()
            .append_pair(TARGET_USER_ID, self.target_user_id.as_ref());

        if let Some(val) = self.source_context {
            url.query_pairs_mut()
                .append_pair(SOURCE_CONTEXT, val.as_ref());
        }

        if let Some(val) = self.reason {
            url.query_pairs_mut().append_pair(REASON, val.as_ref());
        }

        let req = self.client.http_client().put(url);
        self.client.no_content(req).await
    }
}

#[derive(Debug)]
pub struct GetUserActiveExtensions<'a> {
    client: &'a Client,
    user_id: Option<&'a UserId>,
}

impl<'a> GetUserActiveExtensions<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self {
            client,
            user_id: None,
        }
    }

    pub fn user_id(mut self, value: &'a UserId) -> Self {
        self.user_id = Some(value);
        self
    }

    pub async fn send(self) -> Result<UserActiveExtensionsResponse, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut().unwrap().extend([USERS, EXTENSIONS]);

        if let Some(val) = self.user_id {
            url.query_pairs_mut().append_pair(USER_ID, val);
        }

        let req = self.client.http_client().get(url);
        self.client.json(req).await
    }
}

#[derive(Debug)]
pub struct UpdateUserExtensions<'a> {
    client: &'a Client,
    data: UserActiveExtensions,
}

impl<'a> UpdateUserExtensions<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self {
            client,
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

    pub async fn send(self) -> Result<UserActiveExtensionsResponse, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut().unwrap().extend([USERS, EXTENSIONS]);

        let req = self.client.http_client().put(url).json(&self.data);
        self.client.json(req).await
    }
}
