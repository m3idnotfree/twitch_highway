use serde::Serialize;

use crate::{
    extensions::{
        ConfigurationSegmentResponse, ExtensionLiveChannelsRespnose,
        ExtensionsBitsProductsResponse, Segment,
    },
    types::{
        constants::{AFTER, BITS, CONFIGURATIONS, EXTENSIONS, EXTENSION_ID, FIRST, LIVE, SEGMENT},
        BroadcasterId, Cost, ExtensionId, JWTToken,
    },
    Client, Error,
};

#[derive(Debug)]
pub struct GetExtensionConfigurationSegmentBuilder<'a> {
    client: &'a Client,
    jwt_token: JWTToken,
    extension_id: &'a ExtensionId,
    segments: &'a [Segment],
    broadcaster_id: Option<&'a BroadcasterId>,
}

impl<'a> GetExtensionConfigurationSegmentBuilder<'a> {
    pub fn new(
        client: &'a Client,
        jwt_token: JWTToken,
        extension_id: &'a ExtensionId,
        segments: &'a [Segment],
    ) -> Self {
        Self {
            client,
            jwt_token,
            extension_id,
            segments,
            broadcaster_id: None,
        }
    }

    pub fn broadcaster_id(mut self, value: &'a BroadcasterId) -> Self {
        self.broadcaster_id = Some(value);
        self
    }

    pub async fn send(self) -> Result<ConfigurationSegmentResponse, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([EXTENSIONS, CONFIGURATIONS]);

        url.query_pairs_mut()
            .append_pair(EXTENSION_ID, self.extension_id)
            .extend_pairs(self.segments.iter().map(|s| (SEGMENT, s.as_ref())));
        if let Some(val) = self.broadcaster_id {
            url.query_pairs_mut().append_pair("broadcaster_id", val);
        }

        let req = self
            .client
            .http_client()
            .get(url)
            .bearer_auth(self.jwt_token);
        self.client.json(req).await
    }
}

#[derive(Debug, Serialize)]
pub struct SetExtensionConfigurationSegmentBuilder<'a> {
    #[serde(skip)]
    client: &'a Client,
    extension_id: &'a ExtensionId,
    segment: Segment,

    #[serde(skip_serializing_if = "Option::is_none")]
    broadcaster_id: Option<&'a BroadcasterId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<&'a str>,
}

impl<'a> SetExtensionConfigurationSegmentBuilder<'a> {
    pub fn new(client: &'a Client, extension_id: &'a ExtensionId, segment: Segment) -> Self {
        Self {
            client,
            extension_id,
            segment,
            broadcaster_id: None,
            content: None,
            version: None,
        }
    }

    pub fn broadcaster_id(mut self, value: &'a BroadcasterId) -> Self {
        self.broadcaster_id = Some(value);
        self
    }

    pub fn content(mut self, value: &'a str) -> Self {
        self.content = Some(value);
        self
    }

    pub fn version(mut self, value: &'a str) -> Self {
        self.version = Some(value);
        self
    }

    pub async fn send(self) -> Result<(), Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([EXTENSIONS, CONFIGURATIONS]);

        let req = self.client.http_client().put(url).json(&self);
        self.client.no_content(req).await
    }
}

#[derive(Debug)]
pub struct GetExtensionLiveChannelsBuilder<'a> {
    client: &'a Client,
    extension_id: &'a ExtensionId,
    first: Option<u8>,
    after: Option<&'a str>,
}

impl<'a> GetExtensionLiveChannelsBuilder<'a> {
    pub fn new(client: &'a Client, extension_id: &'a ExtensionId) -> Self {
        Self {
            client,
            extension_id,
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

    pub async fn send(self) -> Result<ExtensionLiveChannelsRespnose, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut().unwrap().extend([EXTENSIONS, LIVE]);

        url.query_pairs_mut()
            .append_pair(EXTENSION_ID, self.extension_id);
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
#[derive(Debug, Serialize)]
pub struct UpdateExtensionBitsProductBuilder<'a> {
    #[serde(skip)]
    client: &'a Client,

    sku: &'a str,
    cost: Cost,
    display_name: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    expiration: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    in_development: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_broadcast: Option<bool>,
}

impl<'a> UpdateExtensionBitsProductBuilder<'a> {
    pub fn new(client: &'a Client, sku: &'a str, cost: Cost, display_name: &'a str) -> Self {
        Self {
            client,
            sku,
            cost,
            display_name,
            expiration: None,
            in_development: None,
            is_broadcast: None,
        }
    }

    pub fn expiration(mut self, value: &'a str) -> Self {
        self.expiration = Some(value);
        self
    }

    pub fn in_development(mut self, value: bool) -> Self {
        self.in_development = Some(value);
        self
    }

    pub fn is_broadcast(mut self, value: bool) -> Self {
        self.is_broadcast = Some(value);
        self
    }

    pub async fn send(self) -> Result<ExtensionsBitsProductsResponse, Error> {
        let mut url = self.client.base_url();
        url.path_segments_mut().unwrap().extend([BITS, EXTENSIONS]);

        let req = self.client.http_client().put(url).json(&self);
        self.client.json(req).await
    }
}
