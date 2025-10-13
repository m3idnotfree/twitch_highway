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

#[cfg(any(
    feature = "bits",
    feature = "channels",
    feature = "chat",
    feature = "entitlements",
    feature = "eventsub",
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
    feature = "conduits",
    feature = "entitlements",
    feature = "eventsub",
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
#[cfg(any(feature = "analytics", feature = "clips",))]
pub(crate) const ENDED_AT: &str = "ended_at";

pub(crate) const BEFORE: &str = "before";
pub(crate) const FIRST: &str = "first";
pub(crate) const AFTER: &str = "after";

#[cfg(any(feature = "eventsub", feature = "subscriptions"))]
pub(crate) const SUBSCRIPTIONS: &str = "subscriptions";

#[cfg(any(
    feature = "analytics",
    feature = "eventsub",
    feature = "streams",
    feature = "videos"
))]
pub(crate) const TYPE: &str = "type";

#[cfg(any(feature = "bits", feature = "extensions"))]
pub(crate) const BITS: &str = "bits";

#[cfg(any(feature = "chat", feature = "extensions", feature = "moderation"))]
pub(crate) const CHAT: &str = "chat";
#[cfg(any(feature = "chat", feature = "moderation", feature = "schedule"))]
pub(crate) const SETTINGS: &str = "settings";

#[cfg(any(feature = "games", feature = "analytics"))]
pub(crate) const GAMES: &str = "games";

#[cfg(any(feature = "conduits", feature = "eventsub"))]
pub(crate) const EVENTSUB: &str = "eventsub";

#[cfg(any(feature = "bits", feature = "videos"))]
pub(crate) const PERIOD: &str = "period";
#[cfg(feature = "bits")]
pub(crate) const COUNT: &str = "count";
#[cfg(feature = "bits")]
pub(crate) const LEADERBOARD: &str = "leaderboard";
#[cfg(feature = "bits")]
pub(crate) const TRANSACTIONS: &str = "transactions";
#[cfg(feature = "bits")]
pub(crate) const CHEERMOTES: &str = "cheermotes";

#[cfg(feature = "channel-points")]
pub(crate) const CHANNEL_POINTS: &str = "channel_points";
#[cfg(feature = "channel-points")]
pub(crate) const CUSTOM_REWARDS: &str = "custom_rewards";
#[cfg(feature = "channel-points")]
pub(crate) const REDEMPTIONS: &str = "redemptions";

#[cfg(any(
    feature = "channel-points",
    feature = "conduits",
    feature = "eventsub",
    feature = "polls"
))]
pub(crate) const STATUS: &str = "status";

#[cfg(any(feature = "channel-points", feature = "videos"))]
pub(crate) const SORT: &str = "sort";
#[cfg(feature = "channel-points")]
pub(crate) const REWARD_ID: &str = "reward_id";

#[cfg(feature = "channels")]
pub(crate) const FOLLOWERS: &str = "followers";
#[cfg(any(feature = "channels", feature = "streams"))]
pub(crate) const FOLLOWED: &str = "followed";
#[cfg(feature = "channels")]
pub(crate) const EDITORS: &str = "editors";

#[cfg(feature = "charity")]
pub(crate) const CHARITY: &str = "charity";
#[cfg(feature = "charity")]
pub(crate) const DONATIONS: &str = "donations";

#[cfg(feature = "entitlements")]
pub(crate) const ENTITLEMENTS: &str = "entitlements";
#[cfg(feature = "entitlements")]
pub(crate) const DROPS: &str = "drops";

#[cfg(feature = "chat")]
pub(crate) const CHATTERS: &str = "chatters";
#[cfg(feature = "chat")]
pub(crate) const GLOBAL: &str = "global";
#[cfg(feature = "chat")]
pub(crate) const SET: &str = "set";
#[cfg(feature = "chat")]
pub(crate) const EMOTE_SET_ID: &str = "emote_set_id";
#[cfg(feature = "chat")]
pub(crate) const BADGES: &str = "badges";
#[cfg(feature = "chat")]
pub(crate) const SHARED_CHAT: &str = "shared_chat";
#[cfg(feature = "chat")]
pub(crate) const SESSION: &str = "session";

#[cfg(any(feature = "chat", feature = "subscriptions"))]
pub(crate) const USER: &str = "user";

#[cfg(feature = "chat")]
pub(crate) const EMOTES: &str = "emotes";
#[cfg(feature = "chat")]
pub(crate) const ANNOUNCEMENTS: &str = "announcements";
#[cfg(feature = "chat")]
pub(crate) const SHOUTOUTS: &str = "shoutouts";
#[cfg(any(feature = "chat", feature = "raid"))]
pub(crate) const FROM_BROADCASTER_ID: &str = "from_broadcaster_id";
#[cfg(any(feature = "chat", feature = "raid"))]
pub(crate) const TO_BROADCASTER_ID: &str = "to_broadcaster_id";
#[cfg(feature = "chat")]
pub(crate) const MESSAGES: &str = "messages";
#[cfg(feature = "chat")]
pub(crate) const COLOR: &str = "color";

#[cfg(feature = "clips")]
pub(crate) const CLIPS: &str = "clips";
#[cfg(feature = "clips")]
pub(crate) const IS_FEATURED: &str = "is_featured";
#[cfg(feature = "clips")]
pub(crate) const HAS_DELAY: &str = "has_delay";

#[cfg(feature = "conduits")]
pub(crate) const CONDUITS: &str = "conduits";
#[cfg(feature = "conduits")]
pub(crate) const CONDUIT_ID: &str = "conduit_id";
#[cfg(feature = "conduits")]
pub(crate) const SHARDS: &str = "shards";

#[cfg(feature = "eventsub")]
pub(crate) const SUBSCRIPTION_ID: &str = "subscription_id";

#[cfg(feature = "raid")]
pub(crate) const RAIDS: &str = "raids";

#[cfg(any(feature = "games", feature = "teams"))]
pub(crate) const NAME: &str = "name";
#[cfg(feature = "games")]
pub(crate) const IGDB_ID: &str = "igdb_id";
#[cfg(feature = "games")]
pub(crate) const TOP: &str = "top";

#[cfg(feature = "goals")]
pub(crate) const GOALS: &str = "goals";

#[cfg(feature = "polls")]
pub(crate) const POLLS: &str = "polls";

#[cfg(feature = "schedule")]
pub(crate) const SCHEDULE: &str = "schedule";
#[cfg(feature = "schedule")]
pub(crate) const SEGMENT: &str = "segment";
#[cfg(feature = "schedule")]
pub(crate) const ICALENDAR: &str = "icalendar";
#[cfg(feature = "schedule")]
pub(crate) const START_TIME: &str = "start_time";
#[cfg(feature = "schedule")]
pub(crate) const VACATION_START_TIME: &str = "vacation_start_time";
#[cfg(feature = "schedule")]
pub(crate) const VACATION_END_TIME: &str = "vacation_end_time";
#[cfg(feature = "schedule")]
pub(crate) const TIMEZONE: &str = "timezone";
#[cfg(feature = "schedule")]
pub(crate) const IS_VACATION_ENABLED: &str = "is_vacation_enabled";

#[cfg(feature = "search")]
pub(crate) const QUERY: &str = "query";
#[cfg(feature = "search")]
pub(crate) const SEARCH: &str = "search";
#[cfg(feature = "search")]
pub(crate) const CATEGORIES: &str = "categories";

#[cfg(feature = "streams")]
pub(crate) const VIDEO_ID: &str = "video_id";
#[cfg(feature = "streams")]
pub(crate) const MARKERS: &str = "markers";
#[cfg(feature = "streams")]
pub(crate) const STREAMS: &str = "streams";
#[cfg(feature = "streams")]
pub(crate) const USER_LOGIN: &str = "user_login";
#[cfg(feature = "streams")]
pub(crate) const KEY: &str = "key";

#[cfg(feature = "teams")]
pub(crate) const TEAMS: &str = "teams";
#[cfg(feature = "teams")]
pub(crate) const CHANNEL: &str = "channel";

#[cfg(feature = "users")]
pub(crate) const USERS: &str = "users";
#[cfg(feature = "users")]
pub(crate) const TARGET_USER_ID: &str = "target_user_id";
#[cfg(feature = "users")]
pub(crate) const SOURCE_CONTEXT: &str = "source_context";
#[cfg(feature = "users")]
pub(crate) const REASON: &str = "reason";
#[cfg(feature = "users")]
pub(crate) const BLOCKS: &str = "blocks";
#[cfg(feature = "users")]
pub(crate) const LOGIN: &str = "login";
#[cfg(feature = "users")]
pub(crate) const LIST: &str = "list";
#[cfg(any(feature = "streams", feature = "users"))]
pub(crate) const DESCRIPTION: &str = "description";

#[cfg(feature = "videos")]
pub(crate) const VIDEOS: &str = "videos";
#[cfg(any(feature = "streams", feature = "videos"))]
pub(crate) const LANGUAGE: &str = "language";

#[cfg(feature = "whisper")]
pub(crate) const FROM_USER_ID: &str = "from_user_id";
#[cfg(feature = "whisper")]
pub(crate) const TO_USER_ID: &str = "to_user_id";
