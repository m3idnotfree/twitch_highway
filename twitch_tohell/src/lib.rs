#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(any(feature = "webhook-actix", feature = "webhook-axum"))]
pub mod webhook;
#[cfg(feature = "websocket")]
pub mod websocket;

pub mod events;

pub use asknothingx2_util::serde::EmptyObject;
pub use twitch_onthe::eventsub::{Subscription, SubscriptionType};
pub use twitch_onthe::ids::{
    BroadcasterId, CampaignId, ConduitId, EmoteId, GoalId, HypeTrainId, MessageId, ModeratorId,
    PollId, PredictionId, RedemptionId, RewardId, SessionId, StreamId, TransactionId,
    UnbanRequestId, UserId, WhisperId,
};
pub use twitch_onthe::{Amount, Choice, Images, Outcome, Reward, TopContribution};
