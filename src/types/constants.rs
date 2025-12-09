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

#[cfg(any(feature = "analytics", feature = "bits", feature = "clips"))]
pub(crate) const STARTED_AT: &str = "started_at";
#[cfg(any(feature = "analytics", feature = "clips"))]
pub(crate) const ENDED_AT: &str = "ended_at";

#[cfg(any(
    feature = "games",
    feature = "moderation",
    feature = "streams",
    feature = "subscriptions",
    feature = "videos",
))]
pub(crate) const BEFORE: &str = "before";
#[cfg(any(
    feature = "analytics",
    feature = "bits",
    feature = "channel-points",
    feature = "channels",
    feature = "charity",
    feature = "chat",
    feature = "clips",
    feature = "entitlements",
    feature = "extensions",
    feature = "games",
    feature = "hype-train",
    feature = "moderation",
    feature = "polls",
    feature = "predictions",
    feature = "schedule",
    feature = "search",
    feature = "streams",
    feature = "subscriptions",
    feature = "users",
    feature = "videos",
))]
pub(crate) const FIRST: &str = "first";
#[cfg(any(
    feature = "analytics",
    feature = "bits",
    feature = "channel-points",
    feature = "channels",
    feature = "charity",
    feature = "chat",
    feature = "clips",
    feature = "conduits",
    feature = "entitlements",
    feature = "eventsub",
    feature = "extensions",
    feature = "games",
    feature = "hype-train",
    feature = "moderation",
    feature = "polls",
    feature = "predictions",
    feature = "schedule",
    feature = "search",
    feature = "streams",
    feature = "subscriptions",
    feature = "users",
    feature = "videos",
))]
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

#[cfg(feature = "analytics")]
pub(crate) const ANALYTICS: &str = "analytics";

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

#[cfg(feature = "ads")]
pub(crate) const COMMERCIAL: &str = "commercial";
#[cfg(feature = "ads")]
pub(crate) const ADS: &str = "ads";
#[cfg(feature = "ads")]
pub(crate) const SNOOZE: &str = "snooze";

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

#[cfg(feature = "ccls")]
pub(crate) const LOCALE: &str = "locale";
#[cfg(feature = "ccls")]
pub(crate) const CONTENT_CLASSIFICATION_LABELS: &str = "content_classification_labels";

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
    feature = "hype-train",
    feature = "moderation",
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
#[cfg(any(feature = "chat", feature = "guest-star"))]
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
#[cfg(feature = "clips")]
pub(crate) const DOWNLOADS: &str = "downloads";
#[cfg(feature = "clips")]
pub(crate) const EDITOR_ID: &str = "editor_id";
#[cfg(feature = "clips")]
pub(crate) const CLIP_ID: &str = "clip_id";

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

#[cfg(feature = "extensions")]
pub(crate) const CONFIGURATIONS: &str = "configurations";
#[cfg(feature = "extensions")]
pub(crate) const LIVE: &str = "live";
#[cfg(feature = "extensions")]
pub(crate) const REQUIRED_CONFIGURATION: &str = "required_configuration";
#[cfg(feature = "extensions")]
pub(crate) const PUBSUB: &str = "pubsub";
#[cfg(feature = "extensions")]
pub(crate) const JWT: &str = "jwt";
#[cfg(feature = "extensions")]
pub(crate) const SECRETS: &str = "secrets";
#[cfg(feature = "extensions")]
pub(crate) const DELAY: &str = "delay";
#[cfg(feature = "extensions")]
pub(crate) const EXTENSION_VERSION: &str = "extension_version";
#[cfg(feature = "extensions")]
pub(crate) const RELEASED: &str = "released";
#[cfg(feature = "extensions")]
pub(crate) const SHOULD_INCLUDE_ALL: &str = "should_include_all";

#[cfg(any(feature = "games", feature = "teams"))]
pub(crate) const NAME: &str = "name";
#[cfg(feature = "games")]
pub(crate) const IGDB_ID: &str = "igdb_id";
#[cfg(feature = "games")]
pub(crate) const TOP: &str = "top";

#[cfg(feature = "goals")]
pub(crate) const GOALS: &str = "goals";

#[cfg(feature = "guest-star")]
pub(crate) const GUEST_STAR: &str = "guest_star";
#[cfg(feature = "guest-star")]
pub(crate) const CHANNEL_SETTINGS: &str = "channel_settings";
#[cfg(feature = "guest-star")]
pub(crate) const IS_MODERATOR_SEND_LIVE_ENABLED: &str = "is_moderator_send_live_enabled";
#[cfg(feature = "guest-star")]
pub(crate) const SLOT_COUNT: &str = "slot_count";
#[cfg(feature = "guest-star")]
pub(crate) const IS_BROWSER_SOURCE_AUDIO_ENABLED: &str = "is_browser_source_audio_enabled";
#[cfg(feature = "guest-star")]
pub(crate) const GROUP_LAYOUT: &str = "group_layout";
#[cfg(feature = "guest-star")]
pub(crate) const REGENERATE_BROWSER_SOURCES: &str = "regenerate_browser_sources";
#[cfg(feature = "guest-star")]
pub(crate) const SESSION_ID: &str = "session_id";
#[cfg(feature = "guest-star")]
pub(crate) const INVITES: &str = "invites";
#[cfg(feature = "guest-star")]
pub(crate) const GUEST_ID: &str = "guest_id";
#[cfg(feature = "guest-star")]
pub(crate) const SLOT: &str = "slot";
#[cfg(feature = "guest-star")]
pub(crate) const SLOT_ID: &str = "slot_id";
#[cfg(feature = "guest-star")]
pub(crate) const SOURCE_SLOT_ID: &str = "source_slot_id";
#[cfg(feature = "guest-star")]
pub(crate) const DESTINATION_SLOT_ID: &str = "destination_slot_id";
#[cfg(feature = "guest-star")]
pub(crate) const SHOULD_REINVITE_GUEST: &str = "should_reinvite_guest";
#[cfg(feature = "guest-star")]
pub(crate) const IS_AUDIO_ENABLED: &str = "is_audio_enabled";
#[cfg(feature = "guest-star")]
pub(crate) const IS_VIDEO_ENABLED: &str = "is_video_enabled";
#[cfg(feature = "guest-star")]
pub(crate) const IS_LIVE: &str = "is_live";
#[cfg(feature = "guest-star")]
pub(crate) const VOLUME: &str = "volume";
#[cfg(feature = "guest-star")]
pub(crate) const SLOT_SETTINGS: &str = "slot_settings";

#[cfg(feature = "hype-train")]
pub(crate) const HYPE_TRAIN: &str = "hypetrain";
#[cfg(feature = "hype-train")]
pub(crate) const EVENTS: &str = "events";

#[cfg(feature = "moderation")]
pub(crate) const AUTOMOD: &str = "automod";
#[cfg(feature = "moderation")]
pub(crate) const MODERATION: &str = "moderation";
#[cfg(feature = "moderation")]
pub(crate) const BLOCKED_TERMS: &str = "blocked_terms";
#[cfg(feature = "moderation")]
pub(crate) const MODERATORS: &str = "moderators";
#[cfg(feature = "moderation")]
pub(crate) const VIPS: &str = "vips";
#[cfg(feature = "moderation")]
pub(crate) const BANNED: &str = "banned";
#[cfg(feature = "moderation")]
pub(crate) const BANS: &str = "bans";
#[cfg(feature = "moderation")]
pub(crate) const UNBAN_REQUESTS: &str = "unban_requests";
#[cfg(feature = "moderation")]
pub(crate) const SHIELD_MODE: &str = "shield_mode";
#[cfg(feature = "moderation")]
pub(crate) const ENFORCEMENTS: &str = "enforcements";
#[cfg(feature = "moderation")]
pub(crate) const MESSAGE: &str = "message";
#[cfg(feature = "moderation")]
pub(crate) const WARNINGS: &str = "warnings";

#[cfg(feature = "polls")]
pub(crate) const POLLS: &str = "polls";

#[cfg(feature = "predictions")]
pub(crate) const PREDICTIONS: &str = "predictions";

#[cfg(any(feature = "ads", feature = "schedule"))]
pub(crate) const SCHEDULE: &str = "schedule";
#[cfg(any(feature = "schedule", feature = "extensions"))]
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
#[cfg(feature = "users")]
pub(crate) const DESCRIPTION: &str = "description";
#[cfg(feature = "users")]
pub(crate) const AUTHORIZATION: &str = "authorization";

#[cfg(feature = "videos")]
pub(crate) const VIDEOS: &str = "videos";
#[cfg(any(feature = "streams", feature = "videos"))]
pub(crate) const LANGUAGE: &str = "language";

#[cfg(feature = "whisper")]
pub(crate) const FROM_USER_ID: &str = "from_user_id";
#[cfg(feature = "whisper")]
pub(crate) const TO_USER_ID: &str = "to_user_id";
#[cfg(feature = "whisper")]
pub(crate) const WHISPERS: &str = "whispers";
