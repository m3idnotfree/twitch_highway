use serde::{de::Visitor, Deserialize, Serialize};
use std::fmt;

macro_rules! new_type {
    (
    $name:ident
    $({
        $($impl_id:ident: [$($check:ident),* $(,)?]),*
        $(,)?
    })?
    ) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(&self.0)
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct IdVisitor;

                impl<'de> Visitor<'de> for IdVisitor {
                    type Value = $name;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str(stringify!($name))
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok($name::from(value))
                    }

                    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok($name::from(value))
                    }
                }

                deserializer.deserialize_string(IdVisitor)
            }
        }
    };
    (@user to) => {
        pub fn to_user(&self) -> UserId {
            UserId::from(self.0.clone())
        }
    };

    (@user into) => {
        pub fn into_user(self) -> UserId {
            UserId::from(self.0)
        }
    };

    (@id to) => {
        pub fn to_id(&self) -> Id {
            Id::from(self.0.clone())
        }
    };
    (@id into) => {
        pub fn into_id(self) -> Id {
            Id::from(self.0)
        }
    };

    (@moderator to) => {
        pub fn to_moderator(&self) -> ModeratorId {
            ModeratorId::from(self.0.clone())
        }
    };
    (@moderator into) => {
        pub fn into_moderator(self) -> ModeratorId {
            ModeratorId::from(self.0)
        }
    };

    (@broadcaster to) => {
        pub fn to_broadcaster(&self) -> BroadcasterId {
            BroadcasterId::from(self.0.clone())
        }
    };
    (@broadcaster into) => {
        pub fn into_broadcaster(self) -> BroadcasterId {
            BroadcasterId::from(self.0)
        }
    };
}

new_type!(BroadcasterId {
    user: [to, into],
    id: [to, into],
    moderator: [to, into]
});
new_type!(ModeratorId {
    user: [to, into],
    id: [to, into],
    broadcaster: [to, into]
});
new_type!(UserId {
    id: [to, into],
    broadcaster: [to, into],
    moderator: [to, into]
});
new_type!(Id);
new_type!(ExtensionId);
new_type!(GameId);
new_type!(RewardId);
new_type!(CustomRewardId);
new_type!(RedemptionId);
new_type!(SessionId);
new_type!(ConduitId);
new_type!(OrganizationId);
new_type!(CategoryId);
new_type!(CampaignId);
new_type!(ExtensionClientId);

new_type!(JWTToken);

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use std::collections::HashMap;

    #[test]
    fn basic() {
        let user_id = UserId::from("userid");
        assert_eq!(user_id.as_str(), "userid");
        let broadcaster_id = BroadcasterId::from("broadcasterid");
        assert_eq!(broadcaster_id.as_str(), "broadcasterid");
        let moderator_id = ModeratorId::from("modid");
        assert_eq!(moderator_id.as_str(), "modid");
        let id = Id::from("id");
        assert_eq!(id.as_str(), "id");
        let extension_id = ExtensionId::from("extensionid");
        assert_eq!(extension_id.as_str(), "extensionid");
        let game_id = GameId::from("gameid");
        assert_eq!(game_id.as_str(), "gameid");
        let reward_id = RewardId::from("rewardid");
        assert_eq!(reward_id.as_str(), "rewardid");
        let custom_reward_id = CustomRewardId::from("customrewardid");
        assert_eq!(custom_reward_id.as_str(), "customrewardid");
        let redemption_id = RedemptionId::from("redemptionid");
        assert_eq!(redemption_id.as_str(), "redemptionid");
        let jwt_token = JWTToken::from("jwttoken");
        assert_eq!(jwt_token.as_str(), "jwttoken");
    }

    #[test]
    fn from_string() {
        let user_id: UserId = "userid".to_string().into();
        assert_eq!(user_id.as_str(), "userid");
        let broadcaster_id: BroadcasterId = "broadcasterid".into();
        assert_eq!(broadcaster_id.as_str(), "broadcasterid");
        let id: Id = "id".into();
        assert_eq!(id.as_str(), "id");
    }

    #[test]
    fn to_string() {
        let user_id = UserId::from("userid");
        let s: String = user_id.into();
        assert_eq!(s, "userid");
        let broadcaster_id = BroadcasterId::from("broadcasterid");
        let s: String = broadcaster_id.into();
        assert_eq!(s, "broadcasterid");
    }

    #[test]
    fn display() {
        let user_id = UserId::from("userid");
        assert_eq!(format!("{}", user_id), "userid");
        let game_id = GameId::from("gameid");
        assert_eq!(format!("{}", game_id), "gameid");
    }

    #[test]
    fn as_ref() {
        let reward_id = RewardId::from("rewardid");
        let s: &str = reward_id.as_ref();
        assert_eq!(s, "rewardid");
    }

    #[test]
    fn clone_and_eq() {
        let user_id1 = UserId::from("userid");
        let user_id2 = user_id1.clone();

        assert_eq!(user_id1, user_id2);
    }

    #[test]
    fn broadcaster_to_conversions() {
        let broadcaster_id = BroadcasterId::from("broadcasterid");

        let user_id = broadcaster_id.to_user();
        assert_eq!(user_id.as_str(), "broadcasterid");

        let id = broadcaster_id.to_id();
        assert_eq!(id.as_str(), "broadcasterid");

        let moderator_id = broadcaster_id.to_moderator();
        assert_eq!(moderator_id.as_str(), "broadcasterid");

        assert_eq!(broadcaster_id.as_str(), "broadcasterid");
    }

    #[test]
    fn broadcaster_into_conversions() {
        let broadcaster_id1 = BroadcasterId::from("broadcasterid1");
        let user_id = broadcaster_id1.into_user();
        assert_eq!(user_id.as_str(), "broadcasterid1");

        let broadcaster_id2 = BroadcasterId::from("broadcasterid2");
        let id = broadcaster_id2.into_id();
        assert_eq!(id.as_str(), "broadcasterid2");

        let broadcaster_id3 = BroadcasterId::from("broadcasterid3");
        let moderator_id = broadcaster_id3.into_moderator();
        assert_eq!(moderator_id.as_str(), "broadcasterid3");
    }

    #[test]
    fn moderator_to_conversions() {
        let moderator_id = ModeratorId::from("modid");

        let user_id = moderator_id.to_user();
        assert_eq!(user_id.as_str(), "modid");

        let id = moderator_id.to_id();
        assert_eq!(id.as_str(), "modid");

        let broadcaster_id = moderator_id.to_broadcaster();
        assert_eq!(broadcaster_id.as_str(), "modid");

        assert_eq!(moderator_id.as_str(), "modid");
    }

    #[test]
    fn moderator_into_conversions() {
        let moderator_id1 = ModeratorId::from("modid1");
        let user_id = moderator_id1.into_user();
        assert_eq!(user_id.as_str(), "modid1");

        let moderator_id2 = ModeratorId::from("modid2");
        let id = moderator_id2.into_id();
        assert_eq!(id.as_str(), "modid2");

        let moderator_id3 = ModeratorId::from("modid3");
        let broadcaster_id = moderator_id3.into_broadcaster();
        assert_eq!(broadcaster_id.as_str(), "modid3");
    }

    #[test]
    fn user_to_conversions() {
        let user_id = UserId::from("userid");

        let id = user_id.to_id();
        assert_eq!(id.as_str(), "userid");

        let broadcaster_id = user_id.to_broadcaster();
        assert_eq!(broadcaster_id.as_str(), "userid");

        let moderator_id = user_id.to_moderator();
        assert_eq!(moderator_id.as_str(), "userid");

        assert_eq!(user_id.as_str(), "userid");
    }

    #[test]
    fn user_into_conversions() {
        let user_id1 = UserId::from("userid1");
        let id = user_id1.into_id();
        assert_eq!(id.as_str(), "userid1");

        let user_id2 = UserId::from("userid2");
        let broadcaster_id = user_id2.into_broadcaster();
        assert_eq!(broadcaster_id.as_str(), "userid2");

        let user_id3 = UserId::from("userid3");
        let moderator_id = user_id3.into_moderator();
        assert_eq!(moderator_id.as_str(), "userid3");
    }

    #[test]
    fn serialize() {
        let user_id = UserId::from("userid");
        let json = serde_json::to_string(&user_id).unwrap();
        assert_eq!(json, "\"userid\"");

        let game_id = GameId::from("gameid");
        let json = serde_json::to_string(&game_id).unwrap();
        assert_eq!(json, "\"gameid\"");

        let jwt_token = JWTToken::from("jwttoken");
        let json = serde_json::to_string(&jwt_token).unwrap();
        assert_eq!(json, "\"jwttoken\"");
    }

    #[test]
    fn deserialize() {
        let user_id: UserId = serde_json::from_str("\"userid\"").unwrap();
        assert_eq!(user_id.as_str(), "userid");

        let game_id: GameId = serde_json::from_str("\"gameid\"").unwrap();
        assert_eq!(game_id.as_str(), "gameid");

        let jwt_token: JWTToken = serde_json::from_str("\"jwttoken\"").unwrap();
        assert_eq!(jwt_token.as_str(), "jwttoken");
    }

    #[test]
    fn roundtrip_serde() {
        let broadcaster_id = BroadcasterId::from("broadcasterid");
        let json = serde_json::to_string(&broadcaster_id).unwrap();
        let deserialized: BroadcasterId = serde_json::from_str(&json).unwrap();
        assert_eq!(broadcaster_id, deserialized);

        let moderator_id = ModeratorId::from("modid");
        let json = serde_json::to_string(&moderator_id).unwrap();
        let deserialized: ModeratorId = serde_json::from_str(&json).unwrap();
        assert_eq!(moderator_id, deserialized);
    }

    #[test]
    fn hash_map() {
        let mut map = HashMap::new();
        let game_id1 = GameId::from("game1");
        let game_id2 = GameId::from("game2");

        map.insert(game_id1.clone(), "Game One");
        map.insert(game_id2.clone(), "Game Two");

        assert_eq!(map.get(&game_id1), Some(&"Game One"));
        assert_eq!(map.get(&game_id2), Some(&"Game Two"));
    }

    #[test]
    fn debug() {
        let reward_id = RewardId::from("rewardid");
        let debug_str = format!("{:?}", reward_id);
        assert!(debug_str.contains("RewardId"));
        assert!(debug_str.contains("rewardid"));
    }

    #[test]
    fn empty_string() {
        let empty_id = Id::from("");
        assert_eq!(empty_id.as_str(), "");
        assert_eq!(format!("{}", empty_id), "");

        let json = serde_json::to_string(&empty_id).unwrap();
        assert_eq!(json, "\"\"");
        let deserialized: Id = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, empty_id);
    }

    #[test]
    fn conversion_chain() {
        let broadcaster_id = BroadcasterId::from("test");
        let user_id = broadcaster_id.to_user();
        let moderator_id = user_id.to_moderator();
        let id = moderator_id.to_id();

        assert_eq!(id.as_str(), "test");
    }

    #[test]
    fn serde_errors() {
        let result: Result<UserId, _> = serde_json::from_str("123");
        assert!(result.is_err());

        let result: Result<GameId, _> = serde_json::from_str("null");
        assert!(result.is_err());

        let result: Result<RewardId, _> = serde_json::from_str("[]");
        assert!(result.is_err());
    }
}
