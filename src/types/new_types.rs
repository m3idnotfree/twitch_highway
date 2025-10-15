use std::{borrow::Borrow, fmt, ops::Deref};

use serde::{Deserialize, Serialize};

macro_rules! new_type {
    (
    $name:ident
    $({
        $($impl_id:ident: [$($check:ident),* $(,)?]),*
        $(,)?
    })?
    ) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn as_str(&self) -> &str {
                &self.0
            }

            $($(
                $(new_type!(@$impl_id $check);)*
            )*)?
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl Deref for $name {
            type Target = str;

            fn deref(&self) -> &str {
                self.as_str()
            }
        }

        impl From<$name> for String {
            fn from(id: $name) -> Self {
                id.0
            }
        }

        impl From<String> for $name {
            fn from(s: String) -> Self {
                Self(s)
            }
        }

        impl From<&str> for $name {
            fn from(s: &str) -> Self {
                Self(s.to_string())
            }
        }

        impl Borrow<str> for $name {
            fn borrow(&self) -> &str {
                &self.0
            }
        }
    };
    (@user to) => {
        pub fn to_user_id(&self) -> UserId {
            UserId::from(self.0.clone())
        }
    };

    (@user into) => {
        pub fn into_user_id(self) -> UserId {
            UserId::from(self.0)
        }
    };

    (@moderator to) => {
        pub fn to_moderator_id(&self) -> ModeratorId {
            ModeratorId::from(self.0.clone())
        }
    };
    (@moderator into) => {
        pub fn into_moderator_id(self) -> ModeratorId {
            ModeratorId::from(self.0)
        }
    };

    (@broadcaster to) => {
        pub fn to_broadcaster_id(&self) -> BroadcasterId {
            BroadcasterId::from(self.0.clone())
        }
    };
    (@broadcaster into) => {
        pub fn into_broadcaster_id(self) -> BroadcasterId {
            BroadcasterId::from(self.0)
        }
    };

    (@category to) => {
        pub fn to_category_id(&self) -> CategoryId {
            CategoryId::from(self.0.clone())
        }
    };
    (@category into) => {
        pub fn into_category_id(self) -> CategoryId {
            CategoryId::from(self.0)
        }
    };

    (@game to) => {
        pub fn to_game_id(&self) -> GameId {
            GameId::from(self.0.clone())
        }
    };
    (@game into) => {
        pub fn into_game_id(self) -> GameId {
            GameId::from(self.0)
        }
    };

    (@segment to) => {
        pub fn to_segment_id(&self) -> SegmentId {
            SegmentId::from(self.0.clone())
        }
    };
    (@segment into) => {
        pub fn into_segment_id(self) -> SegmentId {
            SegmentId::from(self.0)
        }
    };
}

new_type!(BroadcasterId {
    user: [to, into],
    moderator: [to, into]
});
new_type!(ModeratorId {
    user: [to, into],
    broadcaster: [to, into]
});
new_type!(UserId {
    broadcaster: [to, into],
    moderator: [to, into]
});

new_type!(CategoryId { game: [to, into] });
new_type!(GameId {
    category: [to, into],
});

new_type!(BlockedTermId);
new_type!(CampaignId);
new_type!(ChoiceId);
new_type!(ClipId);
new_type!(ConduitId);
new_type!(EmoteId);
new_type!(EntitlementId);
new_type!(ExtensionClientId);
new_type!(ExtensionId);
new_type!(GoalId);
new_type!(HypeTrainId);
new_type!(MarkerId);
new_type!(MessageId);
new_type!(OrganizationId);
new_type!(OutcomeId);
new_type!(PollId);
new_type!(PredictionId);
new_type!(RedemptionId);
new_type!(RewardId);
new_type!(SegmentId);
new_type!(SessionId);
new_type!(ShardId);
new_type!(StreamId);
new_type!(SubscriptionId);
new_type!(TeamId);
new_type!(TransactionId);
new_type!(UnbanRequestId);
new_type!(VideoId);
new_type!(WhisperId);

new_type!(JWTToken);

#[cfg(test)]
mod tests {
    use super::BroadcasterId;

    #[test]
    fn basic() {
        let broadcaster_id = BroadcasterId::from("broadcasterid");
        assert_eq!(broadcaster_id.as_str(), "broadcasterid");

        assert_eq!(&broadcaster_id.to_string(), "broadcasterid");
        assert_eq!(format!("{}", broadcaster_id), "broadcasterid");

        {
            let broadcaster_id2 = BroadcasterId::from("broadcasterid");
            assert_eq!(broadcaster_id, broadcaster_id2);
        }

        let json = serde_json::to_string(&broadcaster_id).unwrap();
        assert_eq!(json, "\"broadcasterid\"");

        let deserialize: BroadcasterId = serde_json::from_str(&json).unwrap();
        assert_eq!(broadcaster_id, deserialize);

        let empty_id = BroadcasterId::from("");
        assert_eq!(empty_id.as_str(), "");
        assert_eq!(format!("{}", empty_id), "");

        let json = serde_json::to_string(&empty_id).unwrap();
        assert_eq!(json, "\"\"");
    }

    #[test]
    fn serde_errors() {
        let result: Result<BroadcasterId, _> = serde_json::from_str("123");
        assert!(result.is_err());

        let result: Result<BroadcasterId, _> = serde_json::from_str("null");
        assert!(result.is_err());

        let result: Result<BroadcasterId, _> = serde_json::from_str("[]");
        assert!(result.is_err());
    }
}
