use serde::{de::Visitor, Deserialize, Serialize};
use std::fmt;

macro_rules! new_type {
    (
    $name:ident
    $({ $($impl_id:ident: $check:expr),* $(,)?})?
    ) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $name(String);

        impl $name {
            pub fn new(id: impl Into<String>) -> Self {
                Self(id.into())
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }

            $($(new_type!(@$impl_id $check);)*)?
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
                        Ok($name::new(value.to_string()))
                    }

                    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok($name::new(value))
                    }
                }

                deserializer.deserialize_string(IdVisitor)
            }
        }
    };
    (@user $check:expr) => {
        pub fn as_user(&self) -> UserId {
            UserId::new(self.0.clone())
        }
    };
    (@id $check:expr) => {
        pub fn as_id(&self) -> Id {
            Id::new(self.0.clone())
        }
    };
    (@moderator $check:expr) => {
        pub fn as_moderator(&self) -> ModeratorId {
            ModeratorId::new(self.0.clone())
        }
    };
    (@broadcaster $check:expr) => {
        pub fn as_broadcaster(&self) -> BroadcasterId {
            BroadcasterId::new(self.0.clone())
        }
    };
}

new_type!(BroadcasterId {
    user: true,
    id: true,
    moderator: true
});
new_type!(ModeratorId {
    user: true,
    id: true,
    broadcaster: true
});
new_type!(UserId {
    id: true,
    broadcaster: true,
    moderator: true
});
new_type!(Id);
new_type!(ExtensionId);
new_type!(GameId);

new_type!(JWTToken);
