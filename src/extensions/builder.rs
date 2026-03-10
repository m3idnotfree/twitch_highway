use serde::Serialize;

use crate::{
    extensions::{
        ConfigurationSegmentResponse, ExtensionLiveChannelsRespnose,
        ExtensionsBitsProductsResponse, Segment,
    },
    request::{NoContent, TwitchAPIRequest},
    types::{
        constants::{AFTER, BITS, CONFIGURATIONS, EXTENSIONS, EXTENSION_ID, FIRST, LIVE, SEGMENT},
        BroadcasterId, Cost, ExtensionId, JWTToken,
    },
    Client,
};

define_request_builder! {
#[derive(Debug)]
GetExtensionConfigurationSegmentBuilder<'a> {
    req: {
        jwt_token: JWTToken [skip],
        extension_id: &'a ExtensionId [key = EXTENSION_ID],
        segments: &'a [Segment] [key = SEGMENT ,convert = extend_as_ref]
    },
    opts: {broadcaster_id: &'a BroadcasterId}
} -> ConfigurationSegmentResponse;
    endpoint: GetExtensionConfigurationSegment,
    method: GET,
    path: [EXTENSIONS, CONFIGURATIONS],
    header: [jwt, jwt_token]
}

#[derive(Debug, Serialize)]
pub struct SetExtensionConfigurationSegmentBuilder<'a> {
    #[serde(skip)]
    api: &'a Client,
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
    pub fn new(api: &'a Client, extension_id: &'a ExtensionId, segment: Segment) -> Self {
        Self {
            api,
            extension_id,
            segment,
            broadcaster_id: None,
            content: None,
            version: None,
        }
    }

    opt_method!(broadcaster_id, &'a BroadcasterId);
    opt_method!(content, &'a str);
    opt_method!(version, &'a str);

    pub fn build(self) -> TwitchAPIRequest<NoContent> {
        let mut url = self.api.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend(&[EXTENSIONS, CONFIGURATIONS]);

        let body = serde_json::to_string(&self).ok();

        TwitchAPIRequest::new(
            crate::request::EndpointType::ModifyChannelInformation,
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

    pub async fn json(self) -> Result<NoContent, crate::Error> {
        self.build().json().await
    }
}

define_request_builder! {
    #[derive(Debug)]
    GetExtensionLiveChannelsBuilder<'a> {
        req: {extension_id: &'a ExtensionId [key = EXTENSION_ID]},
        opts: {
            first: u8 [key = FIRST, convert = to_string],
            after: &'a str [key = AFTER]
        }
    } -> ExtensionLiveChannelsRespnose;
    endpoint: GetExtensionLiveChannels,
    method: GET,
    path: [EXTENSIONS, LIVE],
}

#[derive(Debug, Serialize)]
pub struct UpdateExtensionBitsProductBuilder<'a> {
    #[serde(skip)]
    api: &'a Client,

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
    pub fn new(api: &'a Client, sku: &'a str, cost: Cost, display_name: &'a str) -> Self {
        Self {
            api,
            sku,
            cost,
            display_name,
            expiration: None,
            in_development: None,
            is_broadcast: None,
        }
    }
    opt_method!(expiration, &'a str);
    opt_method!(in_development, bool);
    opt_method!(is_broadcast, bool);

    pub fn build(self) -> TwitchAPIRequest<ExtensionsBitsProductsResponse> {
        let mut url = self.api.base_url();
        url.path_segments_mut().unwrap().extend(&[BITS, EXTENSIONS]);

        let body = serde_json::to_string(&self).ok();

        TwitchAPIRequest::new(
            crate::request::EndpointType::GetReleasedExtensions,
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

    pub async fn json(self) -> Result<ExtensionsBitsProductsResponse, crate::Error> {
        self.build().json().await
    }
}
