use asknothingx2_eventsub::twitch::subscription_types::channel_subscriptions::ChannelRaidRequest;
use asknothingx2_util::api::Method;
use request::EventSubRequest;

use crate::{
    base::{TwitchAPI, TwitchAPIBase},
    EmptyBody, EndpointType, TwitchAPIRequest,
};

pub mod request;
pub mod response;
pub mod types;

pub trait EventSubAPI: TwitchAPIBase {
    /// https://dev.twitch.tv/docs/api/reference/#delete-eventsub-subscription
    fn delete(&self, id: &str) -> TwitchAPIRequest<EmptyBody>;
    /// https://dev.twitch.tv/docs/api/reference/#get-eventsub-subscriptions
    fn get(&self, request: Option<EventSubRequest>) -> TwitchAPIRequest<EmptyBody>;
    // fn drop_entitlement_grant<T: Into<String>>(
    //     &self,
    //     kind: SubscriptionTypes,
    //     transport: Transport,
    //     organization_id: T,
    // ) -> CreateEventSub<DropEntitlementGrantCondition>;
    // fn extension_bits_transaction_create<T: Into<String>>(
    //     &self,
    //     kind: SubscriptionTypes,
    //     transport: Transport,
    //     extension_client_id: T,
    // ) -> CreateEventSub<ExtensionBitsTransactionCreateCondition>;
    // fn conduit_shard_disablet<T: Into<String>>(
    //     &self,
    //     kind: SubscriptionTypes,
    //     transport: Transport,
    //     client_id: T,
    // ) -> CreateEventSub<ConduitShardDisabledCondition>;
    fn channel_raid_as_webhook<T: Into<String>>(
        &self,
        callback: T,
        secret: Option<T>,
        from_broadcaster_user_id: Option<T>,
        to_broadcacter_user_id: Option<T>,
    ) -> TwitchAPIRequest<ChannelRaidRequest>;
    fn channel_raid_as_websocket<T: Into<String>>(
        &self,
        session_id: T,
        from_broadcaster_user_id: Option<T>,
        to_broadcacter_user_id: Option<T>,
    ) -> TwitchAPIRequest<ChannelRaidRequest>;
}

impl EventSubAPI for TwitchAPI {
    fn delete(&self, id: &str) -> TwitchAPIRequest<EmptyBody> {
        TwitchAPIRequest::new(
            EndpointType::DeleteEventSub,
            self.build_url()
                .path(["eventsub", "subscriptions"])
                .query([("id", id)])
                .build(),
            Method::DELETE,
            self.build_headers().build(),
            EmptyBody,
        )
    }

    fn get(&self, request: Option<EventSubRequest>) -> TwitchAPIRequest<EmptyBody> {
        let request = request.map(|req| {
            let mut params = Vec::new();

            if let Some(status) = &req.status {
                params.push(("status", status.to_string()));
            }
            if let Some(kind) = &req.kind {
                params.push(("type", kind.to_string()));
            }
            if let Some(user_id) = &req.user_id {
                params.push(("user_id", user_id.to_string()))
            }

            params
        });
        TwitchAPIRequest::new(
            EndpointType::GetEventSub,
            self.build_url()
                .path(["eventsub", "subscriptions"])
                .query_option_extend(request)
                .build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }

    fn channel_raid_as_webhook<T: Into<String>>(
        &self,
        callback: T,
        secret: Option<T>,
        from_broadcaster_user_id: Option<T>,
        to_broadcacter_user_id: Option<T>,
    ) -> TwitchAPIRequest<ChannelRaidRequest> {
        let body = ChannelRaidRequest::webhook(callback, secret);
        let body = match (from_broadcaster_user_id, to_broadcacter_user_id) {
            (None, None) => body,
            (Some(from), None) => body.set_from_broadcaster_user_id(from),
            (None, Some(to)) => body.set_to_broadcacter_user_id(to),
            (Some(from), Some(to)) => body
                .set_from_broadcaster_user_id(from)
                .set_to_broadcacter_user_id(to),
        };

        TwitchAPIRequest::new(
            EndpointType::ChannelRaid,
            self.build_url().path(["eventsub", "subscriptions"]).build(),
            Method::POST,
            self.build_headers().json().build(),
            body,
        )
    }

    fn channel_raid_as_websocket<T: Into<String>>(
        &self,
        session_id: T,
        from_broadcaster_user_id: Option<T>,
        to_broadcacter_user_id: Option<T>,
    ) -> TwitchAPIRequest<ChannelRaidRequest> {
        let body = ChannelRaidRequest::websocket(session_id);
        let body = match (from_broadcaster_user_id, to_broadcacter_user_id) {
            (None, None) => body,
            (Some(from), None) => body.set_from_broadcaster_user_id(from),
            (None, Some(to)) => body.set_to_broadcacter_user_id(to),
            (Some(from), Some(to)) => body
                .set_from_broadcaster_user_id(from)
                .set_to_broadcacter_user_id(to),
        };

        TwitchAPIRequest::new(
            EndpointType::ChannelRaid,
            self.build_url().path(["eventsub", "subscriptions"]).build(),
            Method::POST,
            self.build_headers().json().build(),
            body,
        )
    }

    // fn drop_entitlement_grant<T: Into<String>>(
    //     &self,
    //     kind: SubscriptionTypes,
    //     transport: Transport,
    //     organization_id: T,
    // ) -> CreateEventSub<DropEntitlementGrantCondition> {
    //     CreateEventSub::drop_entitlement_grant_condition(
    //         self.access_token.clone(),
    //         self.client_id.clone(),
    //         kind,
    //         transport,
    //         organization_id.into(),
    //     )
    // }
    //
    // fn extension_bits_transaction_create<T: Into<String>>(
    //     &self,
    //     kind: SubscriptionTypes,
    //     transport: Transport,
    //     extension_client_id: T,
    // ) -> CreateEventSub<ExtensionBitsTransactionCreateCondition> {
    //     CreateEventSub::extension_bits_transaction_create_condition(
    //         self.access_token.clone(),
    //         self.client_id.clone(),
    //         kind,
    //         transport,
    //         extension_client_id.into(),
    //     )
    // }
    //
    // fn conduit_shard_disablet<T: Into<String>>(
    //     &self,
    //     kind: SubscriptionTypes,
    //     transport: Transport,
    //     client_id: T,
    // ) -> CreateEventSub<ConduitShardDisabledCondition> {
    //     CreateEventSub::conduit_shard_disabled(
    //         self.access_token.clone(),
    //         self.client_id.clone(),
    //         kind,
    //         transport,
    //         client_id.into(),
    //     )
    // }
}
