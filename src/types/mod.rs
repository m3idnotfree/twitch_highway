mod new_types;

#[cfg(any(
    feature = "ads",
    feature = "bits",
    feature = "channel-points",
    feature = "channels",
    feature = "charity",
    feature = "chat",
    feature = "clips",
    feature = "eventsub",
    feature = "extensions",
    feature = "goals",
    feature = "guest-star",
    feature = "hype-train",
    feature = "moderation",
    feature = "polls",
    feature = "predictions",
    feature = "raid",
    feature = "schedule",
    feature = "search",
    feature = "streams",
    feature = "subscriptions",
    feature = "teams",
    feature = "users",
))]
pub use new_types::BroadcasterId;

#[cfg(any(
    feature = "analytics",
    feature = "bits",
    feature = "extensions",
    feature = "users"
))]
pub use new_types::ExtensionId;

#[cfg(any(
    feature = "analytics",
    feature = "channels",
    feature = "clips",
    feature = "entitlements",
    feature = "streams",
    feature = "videos",
))]
pub use new_types::GameId;

#[cfg(any(
    feature = "bits",
    feature = "channel-points",
    feature = "channels",
    feature = "charity",
    feature = "chat",
    feature = "entitlements",
    feature = "eventsub",
    feature = "guest-star",
    feature = "hype-train",
    feature = "moderation",
    feature = "predictions",
    feature = "streams",
    feature = "subscriptions",
    feature = "teams",
    feature = "users",
    feature = "videos",
))]
pub use new_types::UserId;

#[cfg(any(feature = "bits", feature = "eventsub"))]
pub use new_types::TransactionId;

#[cfg(any(feature = "channel-points", feature = "eventsub"))]
pub use new_types::RedemptionId;

#[cfg(any(feature = "charity", feature = "eventsub"))]
pub use new_types::CampaignId;

#[cfg(any(
    feature = "chat",
    feature = "eventsub",
    feature = "guest-star",
    feature = "moderation"
))]
pub use new_types::ModeratorId;

#[cfg(feature = "clips")]
pub use new_types::ClipId;

#[cfg(any(feature = "conduits", feature = "eventsub", feature = "guest-star"))]
pub use new_types::SessionId;

#[cfg(any(feature = "conduits", feature = "eventsub"))]
pub use new_types::ConduitId;

#[cfg(feature = "conduits")]
pub use new_types::ShardId;

#[cfg(feature = "entitlements")]
pub use new_types::EntitlementId;

#[cfg(any(feature = "eventsub", feature = "moderation"))]
pub use new_types::UnbanRequestId;

#[cfg(any(feature = "eventsub", feature = "hype-train"))]
pub use new_types::HypeTrainId;

#[cfg(feature = "eventsub")]
pub use new_types::{
    EmoteId, ExtensionClientId, MessageId, OrganizationId, StreamId, SubscriptionId, WhisperId,
};

#[cfg(any(feature = "eventsub", feature = "goals"))]
pub use new_types::GoalId;
#[cfg(feature = "extensions")]
pub use new_types::JWTToken;

#[cfg(feature = "moderation")]
pub use new_types::BlockedTermId;

#[cfg(feature = "schedule")]
pub use new_types::SegmentId;

#[cfg(any(feature = "streams", feature = "videos"))]
pub use new_types::VideoId;

#[cfg(feature = "streams")]
pub use new_types::MarkerId;

#[cfg(feature = "teams")]
pub use new_types::TeamId;

pub(crate) mod constants;

#[cfg(any(feature = "charity", feature = "eventsub",))]
mod amount;
#[cfg(any(feature = "charity", feature = "eventsub",))]
pub use amount::Amount;

#[cfg(any(
    feature = "eventsub",
    feature = "games",
    feature = "schedule",
    feature = "search"
))]
pub use new_types::CategoryId;
#[cfg(any(feature = "games", feature = "schedule", feature = "search"))]
mod category;
#[cfg(any(feature = "games", feature = "schedule", feature = "search"))]
pub use category::Category;

#[cfg(any(feature = "eventsub", feature = "polls"))]
pub use new_types::{ChoiceId, PollId};
#[cfg(any(feature = "eventsub", feature = "polls"))]
mod choice;
#[cfg(any(feature = "eventsub", feature = "polls"))]
pub use choice::Choice;

#[cfg(any(feature = "bits", feature = "extensions"))]
mod cost;
#[cfg(any(feature = "bits", feature = "extensions"))]
pub use cost::{Cost, CostType};

#[cfg(any(feature = "analytics", feature = "bits"))]
mod date_range;
#[cfg(any(feature = "analytics", feature = "bits"))]
pub use date_range::DateRange;

#[cfg(any(feature = "channel-points", feature = "chat", feature = "eventsub"))]
mod image;
#[cfg(any(feature = "channel-points", feature = "chat", feature = "eventsub"))]
pub use image::Images;

#[cfg(any(feature = "eventsub", feature = "predictions"))]
pub use new_types::{OutcomeId, PredictionId};
#[cfg(any(feature = "eventsub", feature = "predictions"))]
mod outcome;
#[cfg(any(feature = "eventsub", feature = "predictions"))]
pub use outcome::{Outcome, OutocmeColor};

#[cfg(any(
    feature = "analytics",
    feature = "bits",
    feature = "channels",
    feature = "charity",
    feature = "chat",
    feature = "clips",
    feature = "entitlements",
    feature = "eventsub",
    feature = "games",
    feature = "hype-train",
    feature = "moderation",
    feature = "polls",
    feature = "predictions",
    feature = "schedule",
    feature = "search",
    feature = "streams",
    feature = "subscriptions",
    feature = "videos",
))]
mod pagination;
#[cfg(any(
    feature = "analytics",
    feature = "bits",
    feature = "channels",
    feature = "charity",
    feature = "chat",
    feature = "clips",
    feature = "entitlements",
    feature = "eventsub",
    feature = "games",
    feature = "hype-train",
    feature = "moderation",
    feature = "polls",
    feature = "predictions",
    feature = "schedule",
    feature = "search",
    feature = "streams",
    feature = "subscriptions",
    feature = "videos",
))]
pub use pagination::Pagination;

#[cfg(any(feature = "channel-points", feature = "eventsub"))]
pub use new_types::RewardId;
#[cfg(any(feature = "channel-points", feature = "eventsub"))]
mod reward;
#[cfg(any(feature = "channel-points", feature = "eventsub"))]
pub use reward::Reward;

#[cfg(any(feature = "eventsub", feature = "conduits"))]
mod status;
#[cfg(any(feature = "eventsub", feature = "conduits"))]
pub use status::Status;

#[cfg(any(feature = "polls", feature = "predictions"))]
mod title;
#[cfg(any(feature = "polls", feature = "predictions"))]
pub use title::Title;

#[cfg(any(feature = "eventsub", feature = "hype-train"))]
mod top_contribution;
#[cfg(any(feature = "eventsub", feature = "hype-train"))]
pub use top_contribution::{ContributionType, TopContribution};

#[cfg(any(feature = "eventsub", feature = "predictions"))]
mod top_predictor;
#[cfg(any(feature = "eventsub", feature = "predictions"))]
pub use top_predictor::TopPredictor;
