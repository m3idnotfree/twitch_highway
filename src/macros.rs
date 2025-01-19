#[cfg(any(
    feature = "ads",
    feature = "analytics",
    feature = "bits",
    feature = "channels",
    feature = "channel_points",
    feature = "chat",
    feature = "clips",
    feature = "entitlements",
    //feature = "eventsub",
    feature = "extensions",
    feature = "games",
    feature = "guest-star",
    feature = "moderation",
    feature = "polls",
    feature = "predictions",
    feature = "raid",
    feature = "schedule",
    feature = "search",
    feature = "streams",
    feature = "teams",
    feature = "users",
    feature = "videos",
))]
macro_rules! request_struct {
    (
        $(#[$struct_meta:meta])*
        $struct_name:ident {
        $(
            $(#[$field_meta:meta])*
            $field_vis:vis $field:ident: $type:ty
        ),*$(,)?
        }
        $(; impl_body: $into_body:expr )?
    ) => {
        #[derive(Debug, Default)]
        $(#[$struct_meta])*
        pub struct $struct_name {
        $(
            $(#[$field_meta])*
            $field_vis $field: Option<$type>
        ),*
        }

        impl $struct_name {
            pub fn new() -> Self {
                Self::default()
            }

            $(
                request_struct!(@opt any $field: $type);
            )*
        }

        $(request_struct!(@into_request_body $struct_name $into_body);)?
    };
    (
        $(#[$struct_meta:meta])*
        $struct_name:ident {
            required {
                $(
                    $(#[$req_meta:meta])*
                    $fv:vis $req_field:ident: $req_type:ty
                ),* $(,)?
            }
        $(,
            optional {
                $(
                    $(#[$opt_meta:meta])*
                    $ofv:vis $opt_field:ident: $opt_type:ty
                ),*$(,)?
            }
        )?
        }
        $(; impl_body: $into_body:expr )?
    ) => {
        $(#[$struct_meta])*
        #[derive(Debug)]
        pub struct $struct_name {
            $(
                $(#[$req_meta])*
                $fv $req_field: $req_type,
            )*
            $(
                $(
                    $(#[$opt_meta])*
                    $ofv $opt_field: Option<$opt_type>,
                )*
            )?
        }

        impl $struct_name {
            pub fn new(
                $(
                    $req_field: $req_type,
                )*
            ) -> Self {
                Self {
                    $(
                        $req_field,
                    )*
                    $(
                        $(
                            $opt_field: None,
                        )*
                    )?
                }
            }

            $(
                $(
                    request_struct!(@opt any $opt_field: $opt_type);
                )*
            )?
        }

        $(request_struct!(@into_request_body $struct_name $into_body);)?
    };
    (
        $(#[$struct_meta:meta])*
        $struct_name:ident {
        $(
            string {
                $(
                    $(#[$string_meta:meta])*
                    $string_vis:vis $string_field:ident: $string_type:ty
                ),* $(,)?
            }
        )?
        $(,
            any {
                $(
                    $(#[$any_meta:meta])*
                    $any_vis:vis $any_field:ident: $any_type:ty
                ),* $(,)?
            }
        )?
        }
        $(; impl_body: $into_body:expr )?
    ) => {
        #[derive(Debug, Default)]
        $(#[$struct_meta])*
        pub struct $struct_name {
        $(
            $(
                $(#[$string_meta])*
                $string_vis $string_field: Option<$string_type>,
            )*
        )?
        $(
            $(
                $(#[$any_meta])*
                $any_vis $any_field: Option<$any_type>,
            )*
        )?
        }

        impl $struct_name {
            pub fn new() -> Self {
                Self::default()
            }

        $(
            $(
                request_struct!(@opt string $string_field);
            )*
        )?

        $(
            $(
                request_struct!(@opt any $any_field: $any_type);
            )*
        )?
        }

        $(request_struct!(@into_request_body $struct_name $into_body);)?
    };
    (@into_request_body $struct_name:ident $into_body:expr)=>{
        impl crate::IntoRequestBody for $struct_name {
            fn as_body(&self) -> Option<String> {
                Some(serde_json::to_string(self).unwrap())
            }
        }
    };
    (@opt any $field:ident: $type:ty) => {
        pub fn $field(mut self, value: $type) -> Self {
            self.$field = Some(value);
            self
        }
    };
    (@opt string $field:ident) => {
        pub fn $field<T: Into<String>>(mut self, value: T) -> Self {
            self.$field = Some(value.into());
            self
        }
    };
}
