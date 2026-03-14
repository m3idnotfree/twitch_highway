use std::{
    borrow::Borrow,
    fmt::{Display, Formatter, Result as FmtResult},
    ops::Deref,
};

use serde::{Deserialize, Serialize};

macro_rules! new_type {
    ($name:ident) => {
        new_type!(@impl $name {}[]);
    };

    ($name:ident {
        $($impl_id:ident: [$($check:ident),* $(,)?]),* $(,)?
    }
        [$($into:ident),* $(,)?]
    ) => {
        new_type!(@impl $name {$($impl_id: [$($check),*]),*} [$($into),*] );
    };

    (@impl
        $name:ident {
            $($impl_id:ident: [$($check:ident),* $(,)?]),*
        }
        [$($into:ident),* $(,)?]
        $(,)?
    ) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn as_str(&self) -> &str {
                &self.0
            }

            $($(new_type!(@$impl_id $check);)*)*
        }

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
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
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self(value)
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self(value.to_string())
            }
        }

        impl Borrow<str> for $name {
            fn borrow(&self) -> &str {
                &self.0
            }
        }

        impl PartialEq<&str> for $name {
            fn eq(&self, other: &&str) -> bool {
                self.0 == *other
            }
        }
        $(
            impl From<$into> for $name {
                fn from(id: $into) -> Self {
                    Self(id.0)
                }
            }
        )*

    };

    (@user to) => {
        #[deprecated(since = "0.1.0", note = "Use From trait instead")]
        pub fn to_user_id(&self) -> UserId {
            UserId::from(self.0.clone())
        }
    };

    (@user into) => {
        #[deprecated(since = "0.1.0", note = "Use From trait instead")]
        pub fn into_user_id(self) -> UserId {
            UserId::from(self.0)
        }
    };

    (@moderator to) => {
        #[deprecated(since = "0.1.0", note = "Use From trait instead")]
        pub fn to_moderator_id(&self) -> ModeratorId {
            ModeratorId::from(self.0.clone())
        }
    };
    (@moderator into) => {
        #[deprecated(since = "0.1.0", note = "Use From trait instead")]
        pub fn into_moderator_id(self) -> ModeratorId {
            ModeratorId::from(self.0)
        }
    };

    (@broadcaster to) => {
        #[deprecated(since = "0.1.0", note = "Use From trait instead")]
        pub fn to_broadcaster_id(&self) -> BroadcasterId {
            BroadcasterId::from(self.0.clone())
        }
    };
    (@broadcaster into) => {
        #[deprecated(since = "0.1.0", note = "Use From trait instead")]
        pub fn into_broadcaster_id(self) -> BroadcasterId {
            BroadcasterId::from(self.0)
        }
    };

    (@category to) => {
        #[deprecated(since = "0.1.0", note = "Use From trait instead")]
        pub fn to_category_id(&self) -> CategoryId {
            CategoryId::from(self.0.clone())
        }
    };
    (@category into) => {
        #[deprecated(since = "0.1.0", note = "Use From trait instead")]
        pub fn into_category_id(self) -> CategoryId {
            CategoryId::from(self.0)
        }
    };

    (@game to) => {
        #[deprecated(since = "0.1.0", note = "Use From trait instead")]
        pub fn to_game_id(&self) -> GameId {
            GameId::from(self.0.clone())
        }
    };
    (@game into) => {
        #[deprecated(since = "0.1.0", note = "Use From trait instead")]
        pub fn into_game_id(self) -> GameId {
            GameId::from(self.0)
        }
    };
}

new_type!(BroadcasterId {
    user: [to, into],
    moderator: [to, into],
    }
    [ModeratorId, UserId]
);
new_type!(ModeratorId {
    user: [to, into],
    broadcaster: [to, into],
    }
    [UserId,BroadcasterId]
);
new_type!(UserId {
    broadcaster: [to, into],
    moderator: [to, into],
    }
    [ModeratorId,BroadcasterId]
);

new_type!(CategoryId { game: [to, into] }[GameId]);
new_type!(
    GameId {
        category: [to, into],
    }[CategoryId]
);

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
