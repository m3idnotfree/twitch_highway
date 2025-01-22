#[cfg(any(
    feature = "ads",
    feature = "bits",
    feature = "channel-points",
    feature = "channels",
    feature = "charity",
    feature = "chat",
    feature = "clips",
    feature = "extensions",
    feature = "goals",
    feature = "guest-star",
    feature = "hype-train",
    feature = "moderation",
    feature = "polls",
    feature = "predictions",
    feature = "raid",
    feature = "schedule",
    feature = "streams",
    feature = "subscriptions",
    feature = "teams",
    feature = "users",
))]
pub(crate) const BROADCASTER_ID: &str = "broadcaster_id";

#[cfg(any(feature = "analytics", feature = "bits", feature = "extensions"))]
pub(crate) const EXTENSION_ID: &str = "extension_id";

#[cfg(any(feature = "chat", feature = "guest-star", feature = "moderation",))]
pub(crate) const MODERATOR_ID: &str = "moderator_id";

#[cfg(feature = "channel-points")]
pub(crate) const REWARD_ID: &str = "reward_id";

#[cfg(any(
    feature = "bits",
    feature = "channels",
    feature = "chat",
    feature = "entitlements",
    feature = "moderation",
    feature = "streams",
    feature = "subscriptions",
    feature = "users",
    feature = "videos",
))]
pub(crate) const USER_ID: &str = "user_id";
#[cfg(any(
    feature = "bits",
    feature = "channel-points",
    feature = "clips",
    feature = "entitlements",
    feature = "games",
    feature = "moderation",
    feature = "polls",
    feature = "predictions",
    feature = "schedule",
    feature = "teams",
    feature = "users",
    feature = "videos",
))]
pub(crate) const ID: &str = "id";
#[cfg(any(
    feature = "streams",
    feature = "videos",
    feature = "analytics",
    feature = "clips",
    feature = "entitlements"
))]
pub(crate) const GAME_ID: &str = "game_id";
#[cfg(any(
    feature = "bits",
    feature = "analytics",
    feature = "extensions",
    feature = "users",
))]
pub(crate) const EXTENSIONS: &str = "extensions";
#[cfg(any(
    feature = "ads",
    feature = "channels",
    feature = "moderation",
    feature = "search"
))]
pub(crate) const CHANNELS: &str = "channels";

#[cfg(any(feature = "analytics", feature = "bits", feature = "clips",))]
pub(crate) const STARTED_AT: &str = "started_at";

pub(crate) const BEFORE: &str = "before";
pub(crate) const FIRST: &str = "first";
pub(crate) const AFTER: &str = "after";

#[cfg(feature = "streams")]
pub(crate) const STREAMS: &str = "streams";

#[cfg(feature = "subscriptions")]
pub(crate) const SUBSCRIPTIONS: &str = "subscriptions";

#[cfg(feature = "teams")]
pub(crate) const TEAMS: &str = "teams";

#[cfg(feature = "users")]
pub(crate) const USERS: &str = "users";
#[cfg(feature = "users")]
pub(crate) const BLOCKS: &str = "blocks";
#[cfg(feature = "users")]
pub(crate) const LOGIN: &str = "login";

#[cfg(feature = "whispers")]
pub(crate) const WHISPERS: &str = "whispers";

#[cfg(feature = "videos")]
pub(crate) const VIDEOS: &str = "videos";

#[cfg(any(feature = "analytics", feature = "videos"))]
pub(crate) const TYPE: &str = "type";

#[cfg(any(feature = "bits", feature = "extensions"))]
pub(crate) const BITS: &str = "bits";

#[cfg(feature = "channel-points")]
pub(crate) const CHANNEL_POINTS: &str = "channel_points";
#[cfg(feature = "channel-points")]
pub(crate) const CUSTOM_REWARDS: &str = "custom_rewards";

#[cfg(feature = "charity")]
pub(crate) const CHARITY: &str = "charity";

#[cfg(any(feature = "chat", feature = "extensions", feature = "moderation"))]
pub(crate) const CHAT: &str = "chat";
#[cfg(feature = "chat")]
pub(crate) const EMOTES: &str = "emotes";
#[cfg(any(feature = "chat", feature = "moderation", feature = "schedule"))]
pub(crate) const SETTINGS: &str = "settings";

#[cfg(any(feature = "games", feature = "analytics"))]
pub(crate) const GAMES: &str = "games";

#[cfg(feature = "analytics")]
pub(crate) const ANALYTICS: &str = "analytics";
