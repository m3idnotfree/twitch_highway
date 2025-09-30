pub mod extract;

mod endpoint;
mod fallback;
mod handler;
mod route;
mod service;

use std::{
    collections::HashMap,
    convert::Infallible,
    fmt::{Debug, Formatter, Result as FmtResult},
    future::{self, Ready},
    task::{Context, Poll},
};

use tower::{Layer, Service};

use crate::eventsub::{
    websocket::{
        router::{
            endpoint::Endpoint,
            fallback::{Fallback, NotFound},
            handler::Handler,
            route::{Route, RouteFuture},
        },
        IntoResponse, MessageType, Request, Response,
    },
    SubscriptionType,
};

pub struct Router<S = ()> {
    sub: HashMap<SubscriptionType, Endpoint<S>>,
    msg: HashMap<MessageType, Endpoint<S>>,
}

impl<S> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn route(mut self, route_fn: RouteFn<S>) -> Self {
        let RouteFn { filter, endpoint } = route_fn;
        match filter {
            EventFilter::SessionWelcome
            | EventFilter::SessionKeepalive
            | EventFilter::SessionReconnect
            | EventFilter::Revocation => {
                self.msg.insert(filter.base_type(), endpoint);
            }
            EventFilter::Notification(sub_type) => {
                if let Some(sub_type) = sub_type {
                    self.sub.insert(sub_type, endpoint);
                } else {
                    self.msg.insert(filter.base_type(), endpoint);
                }
            }
        };

        self
    }

    pub fn sub_route<H, T>(mut self, sub_type: SubscriptionType, handler: H) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.sub.insert(sub_type, Endpoint::from_handler(handler));
        self
    }

    pub fn msg_route<H, T>(mut self, msg_type: MessageType, handler: H) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.msg.insert(msg_type, Endpoint::from_handler(handler));
        self
    }

    pub fn layer<L>(self, layer: L) -> Router<S>
    where
        L: Layer<Route> + Clone + Send + Sync + 'static,
        L::Service: Service<Request> + Clone + Send + Sync + 'static,
        <L::Service as Service<Request>>::Response: IntoResponse + 'static,
        <L::Service as Service<Request>>::Error: Into<Infallible> + 'static,
        <L::Service as Service<Request>>::Future: Send + 'static,
    {
        let msg = self
            .msg
            .into_iter()
            .map(|(id, endpoint)| {
                let route = endpoint.layer(layer.clone());
                (id, route)
            })
            .collect();

        let sub = self
            .sub
            .into_iter()
            .map(|(id, endpoint)| {
                let route = endpoint.layer(layer.clone());
                (id, route)
            })
            .collect();

        Router { sub, msg }
    }

    pub fn with_state<S2>(self, state: S) -> Router<S2> {
        let sub = self
            .sub
            .into_iter()
            .map(|(id, endpoint)| {
                let endpoint = endpoint.with_state(state.clone());

                (id, endpoint)
            })
            .collect();

        let msg = self
            .msg
            .into_iter()
            .map(|(id, endpoint)| {
                let endpoint = endpoint.with_state(state.clone());
                (id, endpoint)
            })
            .collect();

        Router { sub, msg }
    }

    pub fn call_with_state(&self, req: Request, state: S) -> RouteFuture<Infallible> {
        if let Some(endpoint) = self.msg.get(&req.message_type) {
            return endpoint.call_with_state(req, state);
        }

        if let Some(sub_type) = req.subscription_type {
            if let Some(endpoint) = self.sub.get(&sub_type) {
                endpoint.call_with_state(req, state)
            } else {
                let fallback = Fallback(Route::new(NotFound));
                fallback.call_with_state(req, state)
            }
        } else {
            let fallback = Fallback(Route::new(NotFound));
            fallback.call_with_state(req, state)
        }
    }
}

impl<S> Clone for Router<S> {
    fn clone(&self) -> Self {
        Self {
            sub: self.sub.clone(),
            msg: self.msg.clone(),
        }
    }
}

impl<S> Default for Router<S> {
    fn default() -> Self {
        Self {
            sub: HashMap::new(),
            msg: HashMap::new(),
        }
    }
}

impl<S> Debug for Router<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct(stringify!(Router))
            .field("subscription_routes", &self.sub.len())
            .field("message_routes", &self.msg.len())
            .field("sub_types", &self.sub.keys().collect::<Vec<_>>())
            .field("msg_types", &self.msg.keys().collect::<Vec<_>>())
            .finish()
    }
}

impl Service<Request> for Router {
    type Response = Response;
    type Error = Infallible;
    type Future = RouteFuture<Infallible>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request) -> Self::Future {
        self.call_with_state(req, ())
    }
}

impl Service<()> for Router {
    type Response = Self;
    type Error = Infallible;
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _req: ()) -> Self::Future {
        future::ready(Ok(self.clone().with_state(())))
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum EventFilter {
    SessionWelcome,
    SessionKeepalive,
    SessionReconnect,
    Revocation,

    Notification(Option<SubscriptionType>),
}

impl EventFilter {
    pub fn base_type(&self) -> MessageType {
        match self {
            Self::SessionWelcome => MessageType::SessionWelcome,
            Self::SessionKeepalive => MessageType::SessionKeepalive,
            Self::SessionReconnect => MessageType::SessionReconnect,
            Self::Revocation => MessageType::Revocation,
            Self::Notification(_) => MessageType::Notification,
        }
    }
}

pub struct RouteFn<S> {
    filter: EventFilter,
    endpoint: Endpoint<S>,
}

macro_rules! route_fn {
    ($fn_name:ident, $type:expr) => {
        pub fn $fn_name<H, T, S>(handler: H) -> RouteFn<S>
        where
            H: Handler<T, S>,
            T: Send + Sync + 'static,
            S: Clone + Send + Sync + 'static,
        {
            RouteFn {
                filter: $type,
                endpoint: Endpoint::from_handler(handler),
            }
        }
    };
}

route_fn!(welcome, EventFilter::SessionWelcome);
route_fn!(keepalive, EventFilter::SessionKeepalive);
route_fn!(reconnect, EventFilter::SessionReconnect);
route_fn!(revocation, EventFilter::Revocation);

// Automod related
route_fn!(
    automod_message_hold,
    EventFilter::Notification(Some(SubscriptionType::AutomodMessageHold))
);
route_fn!(
    automod_message_hold_v2,
    EventFilter::Notification(Some(SubscriptionType::AutomodMessageHoldV2))
);
route_fn!(
    automod_message_update,
    EventFilter::Notification(Some(SubscriptionType::AutomodMessageUpdate))
);
route_fn!(
    automod_message_update_v2,
    EventFilter::Notification(Some(SubscriptionType::AutomodMessageUpdateV2))
);
route_fn!(
    automod_settings_update,
    EventFilter::Notification(Some(SubscriptionType::AutomodSettingsUpdate))
);
route_fn!(
    automod_terms_update,
    EventFilter::Notification(Some(SubscriptionType::AutomodTermsUpdate))
);

// Channel related
route_fn!(
    channel_bits_use,
    EventFilter::Notification(Some(SubscriptionType::ChannelBitsUse))
);
route_fn!(
    channel_update,
    EventFilter::Notification(Some(SubscriptionType::ChannelUpdate))
);
route_fn!(
    channel_follow,
    EventFilter::Notification(Some(SubscriptionType::ChannelFollow))
);
route_fn!(
    channel_ad_break_begin,
    EventFilter::Notification(Some(SubscriptionType::ChannelAdBreakBegin))
);

// Chat related
route_fn!(
    channel_chat_clear,
    EventFilter::Notification(Some(SubscriptionType::ChannelChatClear))
);
route_fn!(
    channel_chat_clear_user_messages,
    EventFilter::Notification(Some(SubscriptionType::ChannelChatClearUserMessages))
);
route_fn!(
    channel_chat_message,
    EventFilter::Notification(Some(SubscriptionType::ChannelChatMessage))
);
route_fn!(
    channel_chat_message_delete,
    EventFilter::Notification(Some(SubscriptionType::ChannelChatMessageDelete))
);
route_fn!(
    channel_chat_notification,
    EventFilter::Notification(Some(SubscriptionType::ChannelChatNotification))
);
route_fn!(
    channel_chat_settings_update,
    EventFilter::Notification(Some(SubscriptionType::ChannelChatSettingsUpdate))
);
route_fn!(
    channel_chat_user_message_hold,
    EventFilter::Notification(Some(SubscriptionType::ChannelChatUserMessageHold))
);
route_fn!(
    channel_chat_user_message_update,
    EventFilter::Notification(Some(SubscriptionType::ChannelChatUserMessageUpdate))
);

// Shared Chat
route_fn!(
    channel_shared_chat_session_begin,
    EventFilter::Notification(Some(SubscriptionType::ChannelSharedChatSessionBegin))
);
route_fn!(
    channel_shared_chat_session_update,
    EventFilter::Notification(Some(SubscriptionType::ChannelSharedChatSessionUpdate))
);
route_fn!(
    channel_shared_chat_session_end,
    EventFilter::Notification(Some(SubscriptionType::ChannelSharedChatSessionEnd))
);

// Subscription related
route_fn!(
    channel_subscribe,
    EventFilter::Notification(Some(SubscriptionType::ChannelSubscribe))
);
route_fn!(
    channel_subscription_end,
    EventFilter::Notification(Some(SubscriptionType::ChannelSubscriptionEnd))
);
route_fn!(
    channel_subscription_gift,
    EventFilter::Notification(Some(SubscriptionType::ChannelSubscriptionGift))
);
route_fn!(
    channel_subscription_message,
    EventFilter::Notification(Some(SubscriptionType::ChannelSubscriptionMessage))
);

// Channel interactions
route_fn!(
    channel_cheer,
    EventFilter::Notification(Some(SubscriptionType::ChannelCheer))
);
route_fn!(
    channel_raid,
    EventFilter::Notification(Some(SubscriptionType::ChannelRaid))
);

// Moderation
route_fn!(
    channel_ban,
    EventFilter::Notification(Some(SubscriptionType::ChannelBan))
);
route_fn!(
    channel_unban,
    EventFilter::Notification(Some(SubscriptionType::ChannelUnban))
);
route_fn!(
    channel_unban_request_create,
    EventFilter::Notification(Some(SubscriptionType::ChannelUnbanRequestCreate))
);
route_fn!(
    channel_unban_request_resolve,
    EventFilter::Notification(Some(SubscriptionType::ChannelUnbanRequestResolve))
);
route_fn!(
    channel_moderate,
    EventFilter::Notification(Some(SubscriptionType::ChannelModerate))
);
route_fn!(
    channel_moderate_v2,
    EventFilter::Notification(Some(SubscriptionType::ChannelModerateV2))
);
route_fn!(
    channel_moderator_add,
    EventFilter::Notification(Some(SubscriptionType::ChannelModeratorAdd))
);
route_fn!(
    channel_moderator_remove,
    EventFilter::Notification(Some(SubscriptionType::ChannelModeratorRemove))
);

// Guest Star (Beta)
route_fn!(
    channel_guest_star_session_begin,
    EventFilter::Notification(Some(SubscriptionType::ChannelGuestStarSessionBegin))
);
route_fn!(
    channel_guest_star_session_end,
    EventFilter::Notification(Some(SubscriptionType::ChannelGuestStarSessionEnd))
);
route_fn!(
    channel_guest_star_guest_update,
    EventFilter::Notification(Some(SubscriptionType::ChannelGuestStarGuestUpdate))
);
route_fn!(
    channel_guest_star_settings_update,
    EventFilter::Notification(Some(SubscriptionType::ChannelGuestStarSettingsUpdate))
);

// Channel Points
route_fn!(
    channel_points_automatic_reward_redemption_add,
    EventFilter::Notification(Some(
        SubscriptionType::ChannelPointsAutomaticRewardRedemptionAdd
    ))
);
route_fn!(
    channel_points_automatic_reward_redemption_add_v2,
    EventFilter::Notification(Some(
        SubscriptionType::ChannelPointsAutomaticRewardRedemptionAddV2
    ))
);
route_fn!(
    channel_points_custom_reward_add,
    EventFilter::Notification(Some(SubscriptionType::ChannelPointsCustomRewardAdd))
);
route_fn!(
    channel_points_custom_reward_update,
    EventFilter::Notification(Some(SubscriptionType::ChannelPointsCustomRewardUpdate))
);
route_fn!(
    channel_points_custom_reward_remove,
    EventFilter::Notification(Some(SubscriptionType::ChannelPointsCustomRewardRemove))
);
route_fn!(
    channel_points_custom_reward_redemption_add,
    EventFilter::Notification(Some(
        SubscriptionType::ChannelPointsCustomRewardRedemptionAdd
    ))
);
route_fn!(
    channel_points_custom_reward_redemption_update,
    EventFilter::Notification(Some(
        SubscriptionType::ChannelPointsCustomRewardRedemptionUpdate
    ))
);

// Polls and Predictions
route_fn!(
    channel_poll_begin,
    EventFilter::Notification(Some(SubscriptionType::ChannelPollBegin))
);
route_fn!(
    channel_poll_progress,
    EventFilter::Notification(Some(SubscriptionType::ChannelPollProgress))
);
route_fn!(
    channel_poll_end,
    EventFilter::Notification(Some(SubscriptionType::ChannelPollEnd))
);
route_fn!(
    channel_prediction_begin,
    EventFilter::Notification(Some(SubscriptionType::ChannelPredictionBegin))
);
route_fn!(
    channel_prediction_progress,
    EventFilter::Notification(Some(SubscriptionType::ChannelPredictionProgress))
);
route_fn!(
    channel_prediction_lock,
    EventFilter::Notification(Some(SubscriptionType::ChannelPredictionLock))
);
route_fn!(
    channel_prediction_end,
    EventFilter::Notification(Some(SubscriptionType::ChannelPredictionEnd))
);

// Suspicious Users
route_fn!(
    channel_suspicious_user_message,
    EventFilter::Notification(Some(SubscriptionType::ChannelSuspiciousUserMessage))
);
route_fn!(
    channel_suspicious_user_update,
    EventFilter::Notification(Some(SubscriptionType::ChannelSuspiciousUserUpdate))
);

// VIP and Warnings
route_fn!(
    channel_vip_add,
    EventFilter::Notification(Some(SubscriptionType::ChannelVIPAdd))
);
route_fn!(
    channel_vip_remove,
    EventFilter::Notification(Some(SubscriptionType::ChannelVIPRemove))
);
route_fn!(
    channel_warning_acknowledgement,
    EventFilter::Notification(Some(SubscriptionType::ChannelWarningAcknowledgement))
);
route_fn!(
    channel_warning_send,
    EventFilter::Notification(Some(SubscriptionType::ChannelWarningSend))
);

// Charity
route_fn!(
    charity_donation,
    EventFilter::Notification(Some(SubscriptionType::CharityDonation))
);
route_fn!(
    charity_campaign_start,
    EventFilter::Notification(Some(SubscriptionType::CharityCampaignStart))
);
route_fn!(
    charity_campaign_progress,
    EventFilter::Notification(Some(SubscriptionType::CharityCampaignProgress))
);
route_fn!(
    charity_campaign_stop,
    EventFilter::Notification(Some(SubscriptionType::CharityCampaignStop))
);

// Conduit
route_fn!(
    conduit_shard_disabled,
    EventFilter::Notification(Some(SubscriptionType::ConduitShardDisabled))
);

// Drops and Extensions
route_fn!(
    drop_entitlement_grant,
    EventFilter::Notification(Some(SubscriptionType::DropEntitlementGrant))
);
route_fn!(
    extension_bits_transaction_create,
    EventFilter::Notification(Some(SubscriptionType::ExtensionBitsTransactionCreate))
);

// Goals and Hype Train
route_fn!(
    goal_begin,
    EventFilter::Notification(Some(SubscriptionType::GoalBegin))
);
route_fn!(
    goal_progress,
    EventFilter::Notification(Some(SubscriptionType::GoalProgress))
);
route_fn!(
    goal_end,
    EventFilter::Notification(Some(SubscriptionType::GoalEnd))
);

route_fn!(
    hype_train_begin_v2,
    EventFilter::Notification(Some(SubscriptionType::HypeTrainBeginV2))
);
route_fn!(
    hype_train_progress_v2,
    EventFilter::Notification(Some(SubscriptionType::HypeTrainProgressV2))
);
route_fn!(
    hype_train_end_v2,
    EventFilter::Notification(Some(SubscriptionType::HypeTrainEndV2))
);

// Shield Mode
route_fn!(
    shield_mode_begin,
    EventFilter::Notification(Some(SubscriptionType::ShieldModeBegin))
);
route_fn!(
    shield_mode_end,
    EventFilter::Notification(Some(SubscriptionType::ShieldModeEnd))
);

// Shoutouts
route_fn!(
    shoutout_create,
    EventFilter::Notification(Some(SubscriptionType::ShoutoutCreate))
);
route_fn!(
    shoutout_received,
    EventFilter::Notification(Some(SubscriptionType::ShoutoutReceived))
);

// Stream Status
route_fn!(
    stream_online,
    EventFilter::Notification(Some(SubscriptionType::StreamOnline))
);
route_fn!(
    stream_offline,
    EventFilter::Notification(Some(SubscriptionType::StreamOffline))
);

// User Authorization
route_fn!(
    user_authorization_grant,
    EventFilter::Notification(Some(SubscriptionType::UserAuthorizationGrant))
);
route_fn!(
    user_authorization_revoke,
    EventFilter::Notification(Some(SubscriptionType::UserAuthorizationRevoke))
);
route_fn!(
    user_update,
    EventFilter::Notification(Some(SubscriptionType::UserUpdate))
);

// Whispers
route_fn!(
    whisper_received,
    EventFilter::Notification(Some(SubscriptionType::WhisperReceived))
);

#[cfg(test)]
const _: () = {
    const fn _route_fn() {
        match panic!("") {
            // Automod related
            #[allow(unreachable_code)]
            SubscriptionType::AutomodMessageHold => {}
            SubscriptionType::AutomodMessageHoldV2 => {}
            SubscriptionType::AutomodMessageUpdate => {}
            SubscriptionType::AutomodMessageUpdateV2 => {}
            SubscriptionType::AutomodSettingsUpdate => {}
            SubscriptionType::AutomodTermsUpdate => {}

            // Channel related
            SubscriptionType::ChannelBitsUse => {}
            SubscriptionType::ChannelUpdate => {}
            SubscriptionType::ChannelFollow => {}
            SubscriptionType::ChannelAdBreakBegin => {}

            // Chat related
            SubscriptionType::ChannelChatClear => {}
            SubscriptionType::ChannelChatClearUserMessages => {}
            SubscriptionType::ChannelChatMessage => {}
            SubscriptionType::ChannelChatMessageDelete => {}
            SubscriptionType::ChannelChatNotification => {}
            SubscriptionType::ChannelChatSettingsUpdate => {}
            SubscriptionType::ChannelChatUserMessageHold => {}
            SubscriptionType::ChannelChatUserMessageUpdate => {}

            // Shared Chat
            SubscriptionType::ChannelSharedChatSessionBegin => {}
            SubscriptionType::ChannelSharedChatSessionUpdate => {}
            SubscriptionType::ChannelSharedChatSessionEnd => {}

            // Subscription related
            SubscriptionType::ChannelSubscribe => {}
            SubscriptionType::ChannelSubscriptionEnd => {}
            SubscriptionType::ChannelSubscriptionGift => {}
            SubscriptionType::ChannelSubscriptionMessage => {}

            // Channel interactions
            SubscriptionType::ChannelCheer => {}
            SubscriptionType::ChannelRaid => {}

            // Moderation
            SubscriptionType::ChannelBan => {}
            SubscriptionType::ChannelUnban => {}
            SubscriptionType::ChannelUnbanRequestCreate => {}
            SubscriptionType::ChannelUnbanRequestResolve => {}
            SubscriptionType::ChannelModerate => {}
            SubscriptionType::ChannelModerateV2 => {}
            SubscriptionType::ChannelModeratorAdd => {}
            SubscriptionType::ChannelModeratorRemove => {}

            // Guest Star (Beta)
            SubscriptionType::ChannelGuestStarSessionBegin => {}
            SubscriptionType::ChannelGuestStarSessionEnd => {}
            SubscriptionType::ChannelGuestStarGuestUpdate => {}
            SubscriptionType::ChannelGuestStarSettingsUpdate => {}

            // Channel Points
            SubscriptionType::ChannelPointsAutomaticRewardRedemptionAdd => {}
            SubscriptionType::ChannelPointsAutomaticRewardRedemptionAddV2 => {}
            SubscriptionType::ChannelPointsCustomRewardAdd => {}
            SubscriptionType::ChannelPointsCustomRewardUpdate => {}
            SubscriptionType::ChannelPointsCustomRewardRemove => {}
            SubscriptionType::ChannelPointsCustomRewardRedemptionAdd => {}
            SubscriptionType::ChannelPointsCustomRewardRedemptionUpdate => {}

            // Polls and Predictions
            SubscriptionType::ChannelPollBegin => {}
            SubscriptionType::ChannelPollProgress => {}
            SubscriptionType::ChannelPollEnd => {}
            SubscriptionType::ChannelPredictionBegin => {}
            SubscriptionType::ChannelPredictionProgress => {}
            SubscriptionType::ChannelPredictionLock => {}
            SubscriptionType::ChannelPredictionEnd => {}

            // Suspicious Users
            SubscriptionType::ChannelSuspiciousUserMessage => {}
            SubscriptionType::ChannelSuspiciousUserUpdate => {}

            // VIP and Warnings
            SubscriptionType::ChannelVIPAdd => {}
            SubscriptionType::ChannelVIPRemove => {}
            SubscriptionType::ChannelWarningAcknowledgement => {}
            SubscriptionType::ChannelWarningSend => {}

            // Charity
            SubscriptionType::CharityDonation => {}
            SubscriptionType::CharityCampaignStart => {}
            SubscriptionType::CharityCampaignProgress => {}
            SubscriptionType::CharityCampaignStop => {}

            // Conduit
            SubscriptionType::ConduitShardDisabled => {}

            // Drops and Extensions
            SubscriptionType::DropEntitlementGrant => {}
            SubscriptionType::ExtensionBitsTransactionCreate => {}

            // Goals and Hype Train
            SubscriptionType::GoalBegin => {}
            SubscriptionType::GoalProgress => {}
            SubscriptionType::GoalEnd => {}
            SubscriptionType::HypeTrainBeginV2 => {}
            SubscriptionType::HypeTrainProgressV2 => {}
            SubscriptionType::HypeTrainEndV2 => {}

            // Shield Mode
            SubscriptionType::ShieldModeBegin => {}
            SubscriptionType::ShieldModeEnd => {}

            // Shoutouts
            SubscriptionType::ShoutoutCreate => {}
            SubscriptionType::ShoutoutReceived => {}

            // Stream Status
            SubscriptionType::StreamOnline => {}
            SubscriptionType::StreamOffline => {}

            // User Authorization
            SubscriptionType::UserAuthorizationGrant => {}
            SubscriptionType::UserAuthorizationRevoke => {}
            SubscriptionType::UserUpdate => {}

            // Whispers
            SubscriptionType::WhisperReceived => {}
        }
    }
};

#[cfg(test)]
mod tests {
    use tower::{limit::ConcurrencyLimitLayer, Service};

    use crate::eventsub::websocket::{
        router::{extract::State, goal_progress, welcome, Router},
        Request, Scanner,
    };

    #[tokio::test]
    async fn base() {
        let app = Router::new()
            .route(welcome(async move || {}))
            .route(goal_progress(async move || {}))
            .route(welcome(async move || {}));

        let data = r#"
{
    "metadata": {
        "message_id": "9e004721-472b-d507-8465-c7ad77872e6c",
        "message_type": "session_welcome",
        "message_timestamp": "2025-09-16T20:32:13.868394Z"
    },
    "payload": {
        "session": {
            "id": "8255e39d_d5e714f3",
            "status": "connected",
            "keepalive_timeout_seconds": 10,
            "reconnect_url": null,
            "connected_at": "2025-09-16T20:32:13.868343Z"
        }
    }
}
"#;

        let scanner = Scanner::new(data).unwrap();
        let req = Request {
            message_type: scanner.message_type,
            subscription_type: scanner.subscription_type,
            scanner,
            data: data.into(),
        };

        let resp = app.call_with_state(req, ()).await.unwrap();

        assert!(resp.is_success());
    }

    #[tokio::test]
    async fn with_state() {
        let app = Router::new()
            .route(welcome(async move |State(_string): State<String>| {}))
            .with_state("String".to_string())
            .route(goal_progress(async move |State(_u_64): State<u64>| {}))
            .with_state(64)
            .route(welcome(async move || {}));

        let data = r#"
{
    "metadata": {
        "message_id": "9e004721-472b-d507-8465-c7ad77872e6c",
        "message_type": "session_welcome",
        "message_timestamp": "2025-09-16T20:32:13.868394Z"
    },
    "payload": {
        "session": {
            "id": "8255e39d_d5e714f3",
            "status": "connected",
            "keepalive_timeout_seconds": 10,
            "reconnect_url": null,
            "connected_at": "2025-09-16T20:32:13.868343Z"
        }
    }
}
"#;

        let scanner = Scanner::new(data).unwrap();
        let req = Request {
            message_type: scanner.message_type,
            subscription_type: scanner.subscription_type,
            scanner,
            data: data.into(),
        };

        let _ = test_service(app, req).await;
    }

    #[tokio::test]
    async fn with_layer() {
        let app = Router::new()
            .route(welcome(async move |State(_string): State<String>| {}))
            .layer(ConcurrencyLimitLayer::new(10))
            .with_state("String".to_string());

        let data = r#"
{
    "metadata": {
        "message_id": "9e004721-472b-d507-8465-c7ad77872e6c",
        "message_type": "session_welcome",
        "message_timestamp": "2025-09-16T20:32:13.868394Z"
    },
    "payload": {
        "session": {
            "id": "8255e39d_d5e714f3",
            "status": "connected",
            "keepalive_timeout_seconds": 10,
            "reconnect_url": null,
            "connected_at": "2025-09-16T20:32:13.868343Z"
        }
    }
}
"#;

        let scanner = Scanner::new(data).unwrap();
        let req = Request {
            message_type: scanner.message_type,
            subscription_type: scanner.subscription_type,
            scanner,
            data: data.into(),
        };

        let _ = test_service(app, req).await;
    }

    async fn test_service<M>(mut svc: M, req: Request)
    where
        M: Service<Request> + Clone + Send + Sync + 'static,
    {
        let _ = svc.call(req).await;
    }
}
