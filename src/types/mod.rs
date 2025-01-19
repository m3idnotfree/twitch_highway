mod cost;
mod date_range;
mod image;
mod new_types;
mod title;

#[cfg(any(
    feature = "analytics",
    feature = "bits",
    feature = "ccls",
    feature = "channel_points",
    feature = "channels",
    feature = "charity",
    feature = "chat",
    feature = "clips",
    feature = "entitlements",
    feature = "extensions",
    feature = "games",
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
    feature = "videos",
    feature = "whispers",
))]
mod pagination;

#[cfg(any(
    feature = "analytics",
    feature = "bits",
    feature = "ccls",
    feature = "channel_points",
    feature = "channels",
    feature = "charity",
    feature = "chat",
    feature = "clips",
    feature = "entitlements",
    feature = "extensions",
    feature = "games",
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
    feature = "videos",
    feature = "whispers",
))]
pub use pagination::{Pagination, PaginationQuery};

#[cfg(any(
    feature = "schedule",
    feature = "search",
    feature = "games",
    feature = "test"
))]
mod category;

#[cfg(any(
    feature = "schedule",
    feature = "search",
    feature = "games",
    feature = "test"
))]
pub use category::Category;

pub use cost::{Cost, CostType};
pub use date_range::DateRange;
pub use image::Images;
pub use new_types::{BroadcasterId, ExtensionId, GameId, Id, JWTToken, ModeratorId, UserId};
pub use title::Title;

pub(crate) mod constants;
